use std::iter::Peekable;
use std::slice::Iter;
use logos::*;
use crate::lexer::Token;

enum AstNode
{


    // Block(AstNode),
}

pub fn expect_call_open(tokens: &mut Peekable<Iter<Token>>) -> bool
{
    if let Some(Token::CallOpen) = tokens.next() { true } else { false }
}

pub fn expect_call_close(tokens: &mut Peekable<Iter<Token>>) -> bool
{
    if let Some(Token::CallClose) = tokens.next() { true } else { false }
}

pub fn expect_comma(tokens: &mut Peekable<Iter<Token>>) -> bool
{
    if let Some(Token::Comma) = tokens.next() { true } else { false }
}

// pub fn accept_token(tokens: &mut Peekable<Iter<Token>>) -> bool
// {
//     if let Some(Token::CallOpen) = tokens.next()
//     {
//         true
//     }
//     else
//     {
//         false
//     }
// }

/// U64, Intrinsic, Symbol
fn parse_argument(tokens: &mut Peekable<Iter<Token>>)  // -> AstNode
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::U64 =>
            {

            }

            Token::Intrinsic => parse_intrinsic_call(tokens),

            Token::Symbol =>
            {

            }

            _ => panic!("Expected a U64, Intrinsic Call, or Symbol. Found: {:?}", token),
        }
    }
}

pub fn parse_intrinsic_call(tokens: &mut Peekable<Iter<Token>>)  // -> AstNode
{
    if let Some(Token::Symbol) = tokens.next()
    {
        assert!(expect_call_open(tokens), "PARSE ERROR: Expected open paren");

        while let Some(token) = tokens.peek()
        {
            let ast_node = match token
            {
                Token::Comma =>
                {
                    tokens.next();
                }

                Token::CallClose => break,

                _ => parse_argument(tokens),
            };
        }

        assert!(
            expect_call_close(tokens),
            "PARSE ERROR: Expected closed paren"
        );

        if let Some(Token::BlockOpen) = tokens.peek() { }
        else
        {
            assert!(expect_comma(tokens), "PARSE ERROR: Expected comma");
        }

        println!("INTRINSIC CALL COMPLETED");
    }
    else
    {
        unreachable!();
    }
}

pub fn parse(tokens: Vec<Token>)
{
    let mut it = tokens.iter().peekable();
    it.peek();

    while let Some(token) = it.next()
    {
        let ast_node = match token
        {
            Token::Intrinsic => parse_intrinsic_call(&mut it),

            _ => unreachable!(),
        };
    }

    for i in it
    {
        println!("{:?}", i);
    }
}
