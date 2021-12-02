use std::io::{Read, Write};
use logos::*;

#[derive(Logos, Debug, PartialEq)]
enum Token
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

fn main()
{
    let filename = std::env::args().skip(1).next().unwrap();
    let mut source_file = std::fs::File::open(filename).unwrap();
    let mut source_code = String::with_capacity(2048);
    source_file.read_to_string(&mut source_code).unwrap();

    let out_filename = std::env::args().skip(2).next().unwrap();
    let mut output_file = std::fs::File::create(out_filename).unwrap();

    let mut lexer = Token::lexer(&source_code);

    for i in lexer
    {
        if let Token::Error = i
        {
            println!("ERROR: {:?}", i);
            break;
        }
        else
        {
            println!("TOKEN: {:?}", i);
        }
    }
}
