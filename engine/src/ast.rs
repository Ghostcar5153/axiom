/// Abstract Syntax Tree node definitions for Axiom.
#[derive(Debug)]
pub enum AstNode {
    Let { name: String, expr: Box<AstNode> },
    Call { func: String, args: Vec<AstNode> },
    Number(f64),
    StrLiteral(String),
    Ident(String),
    Noop,
}
