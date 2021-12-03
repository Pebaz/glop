use logos::*;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a>
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
    #[regex(r"[a-zA-Z_\-][a-zA-Z_\-0-9]*", |lex| lex.slice())]
    Symbol(&'a str),

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    U64(u64),

    #[error]
    #[regex(r"[ \t\n\r]+", skip)]
    Error,
}
