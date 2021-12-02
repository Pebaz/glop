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

pub fn expect_equal(tokens: &mut Peekable<Iter<Token>>) -> bool
{
    if let Some(Token::Equal) = tokens.next() { true } else { false }
}

pub fn expect_block_open(tokens: &mut Peekable<Iter<Token>>) -> bool
{
    if let Some(Token::BlockOpen) = tokens.next() { true } else { false }
}

pub fn expect_block_close(tokens: &mut Peekable<Iter<Token>>) -> bool
{
    if let Some(Token::BlockClose) = tokens.next() { true } else { false }
}

pub fn expect_else(tokens: &mut Peekable<Iter<Token>>) -> bool
{
    if let Some(Token::Else) = tokens.next() { true } else { false }
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
            Token::U64(value) =>
            {

            }

            Token::Intrinsic => parse_intrinsic_call(tokens),

            Token::Symbol(symbol) =>
            {

            }

            _ => panic!("Expected a U64, Intrinsic Call, or Symbol. Found: {:?}", token),
        }
    }
}

pub fn parse_intrinsic_call(tokens: &mut Peekable<Iter<Token>>)  // -> AstNode
{
    if let Some(Token::Symbol(symbol)) = tokens.next()
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

        // if let Some(Token::BlockOpen) = tokens.peek() { }
        // else
        // {
        //     assert!(expect_comma(tokens), "PARSE ERROR: Expected comma");
        // }

        println!("INTRINSIC CALL: {:?}", symbol);
    }
    else
    {
        unreachable!();
    }
}

pub fn parse_if(tokens: &mut Peekable<Iter<Token>>)
{
    println!("PARSE IF");

    parse_argument(tokens);

    assert!(expect_block_open(tokens), "PARSE ERROR: Expected open block");

    while let Some(token) = tokens.peek()
    {
        println!("IF BLOCK PARSER STATE: {:?}", token);

        if let Token::BlockClose = token
        {
            break;
        }
        else
        {
            parse_statement(tokens);
        }
    }

    assert!(expect_block_close(tokens), "PARSE ERROR: Expected closed block");

    assert!(expect_else(tokens), "PARSE ERROR: Expected else keyword");

    assert!(expect_block_open(tokens), "PARSE ERROR: Expected open block");

    while let Some(token) = tokens.peek()
    {
        println!("ELSE BLOCK PARSER STATE: {:?}", token);

        if let Token::BlockClose = token
        {
            break;
        }
        else
        {
            parse_statement(tokens);
        }
    }

    println!("HERE");

    assert!(expect_block_close(tokens), "PARSE ERROR: Expected closed block");
}

/// Intrinsic, If+Else, Loop, Break, Let, Set
pub fn parse_block(tokens: &mut Peekable<Iter<Token>>)
{
    assert!(expect_block_open(tokens), "PARSE ERROR: Expected open block");

    while let Some(token) = tokens.peek()
    {
        println!("BLOCK PARSER STATE: {:?}", token);

        if let Token::BlockClose = token
        {
            tokens.next();
            break;
        }
        else
        {
            parse_statement(tokens);
        }
    }

    println!("HERE");

    assert!(expect_block_close(tokens), "PARSE ERROR: Expected closed block");
}

pub fn parse_loop(tokens: &mut Peekable<Iter<Token>>)
{
    parse_block(tokens);
}

pub fn parse_break(tokens: &mut Peekable<Iter<Token>>)
{

}

pub fn parse_let(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::Symbol(symbol) =>
            {
                assert!(
                    expect_equal(tokens),
                    "PARSE ERROR: Expected equal sign"
                );

                parse_argument(tokens);
            }

            _ => panic!("Expected Symbol. Found: {:?}", token),
        }
    }
}

pub fn parse_set(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::Symbol(symbol) =>
            {
                assert!(
                    expect_equal(tokens),
                    "PARSE ERROR: Expected equal sign"
                );

                parse_argument(tokens);
            }

            _ => panic!("Expected Symbol. Found: {:?}", token),
        }
    }
}

/// Intrinsic, If+Else, Loop, Break, Let, Set
pub fn parse_statement(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::Intrinsic => parse_intrinsic_call(tokens),

            Token::If => parse_if(tokens),

            Token::Loop => parse_loop(tokens),

            Token::Break => parse_break(tokens),

            Token::Let => parse_let(tokens),

            Token::Set => parse_set(tokens),

            Token::Comma => return,

            _ => unreachable!(),
        }
    }
}

pub fn parse(tokens: Vec<Token>)
{
    // let mut program = Vec::new();
    let mut it = tokens.iter().peekable();
    it.peek();

    while let Some(token) = it.peek()
    {
        println!("PARSER STATE: {:?}", token);

        parse_statement(&mut it);
    }

    for i in it
    {
        println!("{:?}", i);
    }
}
