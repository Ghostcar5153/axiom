use logos::Logos;

/// All the tokens in Axiom.
#[derive(Logos, Debug, PartialEq)]
pub enum TokenKind {
    // Keywords
    #[token("let")]    Let,
    #[token("cast")]   Cast,
    #[token("when")]   When,
    #[token("else")]   Else,
    #[token("unsafe")] Unsafe,

    // Punctuation
    #[token("=")]      Eq,
    #[token(",")]      Comma,
    #[token(";")]      Semi,
    #[token("(")]      LParen,
    #[token(")")]      RParen,
    #[token("{")]      LBrace,
    #[token("}")]      RBrace,

    // Literals
    #[regex(r"[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse())]
    Number(f64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = &lex.slice()[1..lex.slice().len()-1];
        s.to_string()
    })]
    StrLiteral(String),

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),
    
    // Comments, skip entire line
    #[regex(r"#.*", logos::skip)]
    Comment,

    // Logos error for unrecognized tokens, skip whitespace
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub type Token<'a> = (TokenKind, std::ops::Range<usize>);

pub fn lex(source: &str) -> Vec<Token<'_>> {
    TokenKind::lexer(source)
        .spanned()
        .map(|(tok, span)| (tok, span))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{lex, TokenKind};

    #[test]
    fn tokenize_let_stmt() {
        let src = r#"let x = 42;"#;
        let toks: Vec<_> = lex(src)
            .into_iter()
            .map(|(t, _)| t)
            .collect();
        assert_eq!(
            toks,
            vec![
                TokenKind::Let,
                TokenKind::Ident("x".into()),
                TokenKind::Eq,
                TokenKind::Number(42.0),
                TokenKind::Semi
            ]
        );
    }

    #[test]
    fn tokenize_string_literal() {
        let src = r#"let s = "hello";"#;
        let toks: Vec<_> = lex(src)
            .into_iter()
            .map(|(t, _)| t)
            .collect();
        assert_eq!(
            toks,
            vec![
                TokenKind::Let,
                TokenKind::Ident("s".into()),
                TokenKind::Eq,
                TokenKind::StrLiteral("hello".into()),
                TokenKind::Semi
            ]
        );
    }
}
