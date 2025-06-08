use crate::ast::AstNode;
use crate::errors::CompileError;
use crate::lexer::{lex, Token, TokenKind};

pub fn parse(source: &str) -> Result<Vec<AstNode>, CompileError> {
    let tokens = lex(source);
    parse_tokens(tokens)
}

/// Parse a Vec of tokens into AST nodes (statements).
fn parse_tokens(tokens: Vec<Token>) -> Result<Vec<AstNode>, CompileError> {
    let mut nodes = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        
        if tokens[i].0 == TokenKind::Cast {
    let mut depth = 0;
    i += 1;
    while i < tokens.len() {
        match &tokens[i].0 {
            TokenKind::LBrace => {
                depth += 1;
            }
            TokenKind::RBrace => {
                if depth == 0 {
                    i += 1;
                    break;
                } else {
                    depth -= 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    continue;
}

        match &tokens[i].0 {
            TokenKind::Let => {
                if i + 3 >= tokens.len() {
                    return Err(CompileError::ParseError("Incomplete let".into()));
                }
                // 1- Name
                let name = match &tokens[i + 1].0 {
                    TokenKind::Ident(n) => n.clone(),
                    _ => return Err(CompileError::ParseError("Expected identifier".into())),
                };
                // 2- '='
                if tokens[i + 2].0 != TokenKind::Eq {
                    return Err(CompileError::ParseError("Expected '='".into()));
                }
                // 3- Expression (literal, ident, or call‐expr)
                let (expr_node, next_i) = match &tokens[i + 3].0 {
                    TokenKind::Number(n)     => (AstNode::Number(*n), i + 4),
                    TokenKind::StrLiteral(s) => (AstNode::StrLiteral(s.clone()), i + 4),
                    TokenKind::Ident(_)      => parse_call_expr(&tokens, i + 3)?,
                    _ => return Err(CompileError::ParseError("Expected literal or call".into())),
                };
                // 4- Expect semicolon at next_i
                if tokens.get(next_i).map(|t| &t.0) != Some(&TokenKind::Semi) {
                    return Err(CompileError::ParseError("Missing ';' in let".into()));
                }
                nodes.push(AstNode::Let {
                    name,
                    expr: Box::new(expr_node),
                });
                i = next_i + 1;
            }

            // Stand‐alone call statement
            TokenKind::Ident(_) if tokens.get(i + 1).map(|t| &t.0) == Some(&TokenKind::LParen) => {
                let (call_node, next_i) = parse_call(&tokens, i)?;
                nodes.push(call_node);
                i = next_i;
            }

            // Fallback no‐op
            _ => {
                nodes.push(AstNode::Noop);
                i += 1;
            }
        }
    }

    Ok(nodes)
}


fn parse_call_expr(tokens: &[Token], start: usize) -> Result<(AstNode, usize), CompileError> {
    let func_name = if let TokenKind::Ident(n) = &tokens[start].0 {
        n.clone()
    } else {
        return Err(CompileError::ParseError("Invalid call syntax".into()));
    };

    let mut args = Vec::new();
    let mut j = start + 2;

    while j < tokens.len() {
        match &tokens[j].0 {
            // Nested call‐expr
            TokenKind::Ident(_) if tokens.get(j + 1).map(|t| &t.0) == Some(&TokenKind::LParen) => {
                let (nested, next_j) = parse_call_expr(tokens, j)?;
                args.push(nested);
                j = next_j;
            }
            // Number
            TokenKind::Number(n) => {
                args.push(AstNode::Number(*n));
                j += 1;
            }
            // String literal
            TokenKind::StrLiteral(s) => {
                args.push(AstNode::StrLiteral(s.clone()));
                j += 1;
            }
            // Bare ident
            TokenKind::Ident(id) => {
                args.push(AstNode::Ident(id.clone()));
                j += 1;
            }
            // Comma separator
            TokenKind::Comma => {
                j += 1;
            }
            // End of args
            TokenKind::RParen => {
                j += 1;
                break;
            }
            other => {
                return Err(CompileError::ParseError(format!(
                    "Invalid call argument: {:?}",
                    other
                )));
            }
        }
    }

    Ok((AstNode::Call { func: func_name, args }, j))
}

fn parse_call(tokens: &[Token], start: usize) -> Result<(AstNode, usize), CompileError> {
    let (node, j) = parse_call_expr(tokens, start)?;
    if tokens.get(j).map(|t| &t.0) != Some(&TokenKind::Semi) {
        return Err(CompileError::ParseError("Missing ';' after call".into()));
    }
    Ok((node, j + 1))
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::ast::AstNode;

    #[test]
    fn parse_standalone_call() {
        let ast = parse("do_stuff(1, 2, \"hi\");").unwrap();
        match &ast[0] {
            AstNode::Call { func, args } => {
                assert_eq!(func, "do_stuff");
                assert_eq!(args.len(), 3);
            }
            _ => panic!("Expected Call node"),
        }
    }

    #[test]
    fn parse_call_in_let() {
        let ast = parse("let x = foo(\"a\", b);").unwrap();
        match &ast[0] {
            AstNode::Let { name, expr } => {
                assert_eq!(name, "x");
                if let AstNode::Call { func, args } = &**expr {
                    assert_eq!(func, "foo");
                    assert_eq!(args.len(), 2);
                } else {
                    panic!("Expected Call in let");
                }
            }
            _ => panic!("Expected Let node"),
        }
    }

    #[test]
    fn parse_nested_call_in_arg() {
        let ast = parse("execute_command(read_stdin());").unwrap();
        match &ast[0] {
            AstNode::Call { func, args } => {
                assert_eq!(func, "execute_command");
                assert_eq!(args.len(), 1);
                if let AstNode::Call { func: inner, args: _ } = &args[0] {
                    assert_eq!(inner, "read_stdin");
                } else {
                    panic!("Expected nested Call");
                }
            }
            _ => panic!("Expected top-level Call"),
        }
    }
}
