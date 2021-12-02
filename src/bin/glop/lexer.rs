use logos::*;

#[derive(Logos, Debug, PartialEq)]
pub enum Token
{
    // Keywords
    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("loop")]
    Loop,

    #[token("break")]
    Break,

    #[token("let")]
    Let,

    #[token("set")]
    Set,

    // Symbols
    #[token("@")]
    Intrinsic,

    #[token(",")]
    Comma,

    #[token("=")]
    Equal,

    #[token("[")]
    BlockOpen,

    #[token("]")]
    BlockClose,

    #[token("(")]
    CallOpen,

    #[token(")")]
    CallClose,

    // Values
    #[regex(r"[a-zA-Z_\-]+")]
    Symbol,

    #[regex("[0-9]+")]
    U64,

    #[error]
    #[regex(r"[] \t\n\r]+", skip)]
    Error,
}
