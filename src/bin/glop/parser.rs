use std::iter::Peekable;
use std::slice::Iter;
use logos::*;
use crate::lexer::Token;

enum AstNode
{
    // Block(AstNode),
}

pub fn expect_call_open(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::CallOpen => return,
            _ => panic!("PARSE ERROR: Expected open paren, found {:?}", token),
        }
    }
}

pub fn expect_call_close(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::CallClose => return,
            _ => panic!(
                "PARSE ERROR: Expected closing paren, found {:?}",
                token
            ),
        }
    }
}

pub fn expect_comma(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::Comma => return,
            _ => panic!("PARSE ERROR: Expected comma, found {:?}", token),
        }
    }
}

pub fn expect_equal(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::Equal => return,
            _ => panic!("PARSE ERROR: Expected equal sign, found {:?}", token),
        }
    }
}

pub fn expect_block_open(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::BlockOpen => return,
            _ => panic!("PARSE ERROR: Expected open block, found {:?}", token),
        }
    }
}

pub fn expect_block_close(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::BlockClose => return,
            _ => panic!(
                "PARSE ERROR: Expected closing block, found {:?}",
                token
            ),
        }
    }
}

pub fn expect_else(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::Else => return,
            _ => panic!(
                "PARSE ERROR: Expected else keyword, found {:?}",
                token
            ),
        }
    }
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
        expect_call_open(tokens);

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

        expect_call_close(tokens);

        // if let Some(Token::BlockOpen) = tokens.peek() { }
        // else
        // {
        //     assert!(expect_comma(tokens), "PARSE ERROR: Expected comma");
        // }

        println!("      INTRINSIC CALL: {:?}", symbol);
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

    expect_block_open(tokens);

    while let Some(token) = tokens.peek()
    {
        println!("    IF BLOCK PARSER STATE: {:?}", token);

        if let Token::BlockClose = token
        {
            println!("HERE. Breaking out of if block");
            tokens.next();
            break;
        }
        else
        {
            println!("    IF BLOCK STATEMENT: {:?}", token);
            parse_statement(tokens);
        }
    }

    expect_else(tokens);

    expect_block_open(tokens);

    while let Some(token) = tokens.peek()
    {
        println!("    ELSE BLOCK PARSER STATE: {:?}", token);

        if let Token::BlockClose = token
        {
            println!("HERE. Breaking out of else block");
            tokens.next();
            break;
        }
        else
        {
            println!("    ELSE BLOCK STATEMENT: {:?}", token);
            parse_statement(tokens);
        }
    }
}

/// Intrinsic, If+Else, Loop, Break, Let, Set
pub fn parse_block(tokens: &mut Peekable<Iter<Token>>)
{
    expect_block_open(tokens);

    while let Some(token) = tokens.peek()
    {
        println!("  BLOCK PARSER STATE: {:?}", token);

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
}

pub fn parse_loop(tokens: &mut Peekable<Iter<Token>>)
{
    parse_block(tokens);
}

pub fn parse_break(tokens: &mut Peekable<Iter<Token>>)
{
    expect_comma(tokens);
}

pub fn parse_let(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::Symbol(symbol) =>
            {
                expect_equal(tokens);

                parse_argument(tokens);
            }

            _ => panic!("Expected Symbol. Found: {:?}", token),
        }
    }

    expect_comma(tokens);
}

pub fn parse_set(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::Symbol(symbol) =>
            {
                expect_equal(tokens);

                parse_argument(tokens);
            }

            _ => panic!("Expected Symbol. Found: {:?}", token),
        }
    }

    expect_comma(tokens);
}

/// Intrinsic, If+Else, Loop, Break, Let, Set
pub fn parse_statement(tokens: &mut Peekable<Iter<Token>>)
{
    if let Some(token) = tokens.next()
    {
        println!("  STATEMENT: {:?}", token);
        match token
        {
            Token::Intrinsic =>
            {
                parse_intrinsic_call(tokens);

                // Top-level statement, expect a comma.
                expect_comma(tokens);
            }

            Token::If => parse_if(tokens),

            Token::Loop => parse_loop(tokens),

            Token::Break => parse_break(tokens),

            Token::Let => parse_let(tokens),

            Token::Set => parse_set(tokens),

            // Token::Comma => return,

            // _ => unreachable!(),
            _ =>
            {
                panic!("ERROR: Found token {:?} instead of statement", token);
            }
        }
    }

    // assert!(expect_comma(tokens), "Expected comma to end statement");
}

pub fn parse(tokens: Vec<Token>)
{
    // println!("TOKENS: {:?}", tokens);

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
