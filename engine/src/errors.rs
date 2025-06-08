/// Compile‐time errors in Axiom
#[derive(Debug)]
pub enum CompileError {
    LexError(String),
    ParseError(String),
    RuntimeError(String),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::LexError(e)   => write!(f, "Lex error: {}", e),
            CompileError::ParseError(e) => write!(f, "Parse error: {}", e),
            CompileError::RuntimeError(e) => write!(f, "Runtime error: {}", e),
        }
    }
}

impl std::error::Error for CompileError {}
