use std::collections::HashMap;
use crate::ast::AstNode;
use crate::types::{Taint, Value};
use crate::errors::CompileError;
use crate::sanitizer;

pub fn run(source: &str) -> Result<(), CompileError> {
    let ast_nodes = crate::parser::parse(source)?;
    let mut env: HashMap<String, Value<String>> = HashMap::new();

    for node in ast_nodes {
        match node {
            AstNode::Let { name, expr } => {
                let val = eval(*expr, &mut env)?;
                env.insert(name, val);
            }
            AstNode::Call { func, args } => {
                // Allow top‐level calls too
                eval_call(&func, args, &mut env)?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn eval(
    node: AstNode,
    env: &mut HashMap<String, Value<String>>,
) -> Result<Value<String>, CompileError> {
    match node {
        AstNode::Number(n)       => Ok(Value::new(n.to_string(), Taint::Clean)),
        AstNode::StrLiteral(s)   => Ok(Value::new(s, Taint::Clean)),
        AstNode::Ident(name)     => env.get(&name)
            .cloned()
            .ok_or_else(|| CompileError::RuntimeError(format!("Unknown variable '{}'", name))),
        AstNode::Call { func, args } => eval_call(&func, args, env),
        _ => Err(CompileError::RuntimeError("Unsupported expression".into())),
    }
}

fn eval_call(
    func: &str,
    args: Vec<AstNode>,
    env: &mut HashMap<String, Value<String>>,
) -> Result<Value<String>, CompileError> {
    let mut vals = Vec::with_capacity(args.len());
    for arg in args {
        vals.push(eval(arg, env)?);
    }

    match func {
        "read_stdin" => Ok(Value::new(read_stdin(), Taint::Tainted)),

        // Sanitizer: accepts tainted or clean, returns clean
        "sanitize_shell_input" => {
            if vals.len() != 1 {
                return Err(CompileError::RuntimeError("sanitize_shell_input expects 1 arg".into()));
            }
            let v = vals.into_iter().next().unwrap();
            let cleaned = sanitizer::sanitize_shell_input(v)
                .map_err(CompileError::RuntimeError)?;
            Ok(cleaned)
        }

        // Dangerous: reject tainted
        "execute_command" => {
            if vals.len() != 1 {
                return Err(CompileError::RuntimeError("execute_command expects 1 arg".into()));
            }
            let v = vals.into_iter().next().unwrap();
            if v.taint == Taint::Tainted {
                return Err(CompileError::RuntimeError(
                    "Attempted to execute tainted command".into(),
                ));
            }
            // (In real code, exec; here we just print)
            println!("> {}", v.inner);
            Ok(Value::new("".to_string(), Taint::Clean))
        }

        // Print any string (clean or tainted) to stdout
        "say" => {
            if vals.len() != 1 {
                return Err(CompileError::RuntimeError(
                    "say expects 1 arg".into(),
                ));
            }
            let v = vals.into_iter().next().unwrap();
            println!("{}", v.inner);
            Ok(Value::new("".to_string(), Taint::Clean))
        }

        other => Err(CompileError::RuntimeError(format!("Unknown function '{}'", other))),
    }
}

/// Simulate stdin input for demo
fn read_stdin() -> String {
    "<<user input>>".to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn taint_prevents_direct_execute() {
        let src = r#"execute_command(read_stdin());"#;
        let err = run(src).unwrap_err();
        assert!(
            format!("{}", err).contains("tainted"),
            "Expected taint error, got: {}",
            err
        );
    }

    #[test]
    fn sanitize_allows_execute() {
        let src = r#"
            let u = read_stdin();
            let c = sanitize_shell_input(u);
            execute_command(c);
        "#;
        // Should not error
        run(src).unwrap();
    }
}
