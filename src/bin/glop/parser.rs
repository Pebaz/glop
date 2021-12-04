use std::iter::Peekable;
use std::slice::Iter;
use logos::*;
use indextree::{Arena, NodeId};
use crate::lexer::Token;

#[derive(Debug)]
pub enum AstNode<'a>
{
    Program,

    Block,  // [Statements ...]

    IfElse,  // [Condition, Truthy Block, Falsey Block]

    Loop,  // [Loop Block]

    Let(&'a str),  // [Value]

    Set(&'a str),  // [Value]

    Intrinsic(&'a str),  // [Arguments ...]

    Symbol(&'a str),

    U64(u64),
}

pub fn expect_call_open(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
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

pub fn expect_call_close(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
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

pub fn expect_comma(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
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

pub fn expect_equal(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
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

pub fn expect_block_open(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
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

pub fn expect_block_close(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
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

pub fn expect_else(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
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

// pub fn accept_token(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId) -> bool
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
fn parse_argument(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)  // -> AstNode
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::U64(value) =>
            {
                parent.append(ast.new_node(AstNode::U64(*value)), ast);
            }

            Token::Intrinsic => parse_intrinsic_call(tokens, ast, parent),

            Token::Symbol(symbol) =>
            {

            }

            _ => panic!("Expected a U64, Intrinsic Call, or Symbol. Found: {:?}", token),
        }
    }
}

pub fn parse_intrinsic_call(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)  // -> AstNode
{
    if let Some(Token::Symbol(symbol)) = tokens.next()
    {
        expect_call_open(tokens, ast, parent);

        while let Some(token) = tokens.peek()
        {
            let ast_node = match token
            {
                Token::Comma =>
                {
                    tokens.next();
                }

                Token::CallClose => break,

                _ => parse_argument(tokens, ast, parent),
            };
        }

        expect_call_close(tokens, ast, parent);

        // if let Some(Token::BlockOpen) = tokens.peek() { }
        // else
        // {
        //     assert!(expect_comma(tokens, ast), "PARSE ERROR: Expected comma", parent);
        // }

        println!("      INTRINSIC CALL: {:?}", symbol);
    }
    else
    {
        unreachable!();
    }
}

pub fn parse_if(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
{
    println!("PARSE IF");

    parse_argument(tokens, ast, parent);

    expect_block_open(tokens, ast, parent);

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
            parse_statement(tokens, ast, parent);
        }
    }

    expect_else(tokens, ast, parent);

    expect_block_open(tokens, ast, parent);

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
            parse_statement(tokens, ast, parent);
        }
    }
}

/// Intrinsic, If+Else, Loop, Break, Let, Set
pub fn parse_block(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
{
    expect_block_open(tokens, ast, parent);

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
            parse_statement(tokens, ast, parent);
        }
    }
}

pub fn parse_loop(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
{
    parse_block(tokens, ast, parent);
}

pub fn parse_break(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
{
    expect_comma(tokens, ast, parent);
}

pub fn parse_let(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::Symbol(symbol) =>
            {
                expect_equal(tokens, ast, parent);

                parse_argument(tokens, ast, parent);
            }

            _ => panic!("Expected Symbol. Found: {:?}", token),
        }
    }

    expect_comma(tokens, ast, parent);
}

pub fn parse_set(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
{
    if let Some(token) = tokens.next()
    {
        match token
        {
            Token::Symbol(symbol) =>
            {
                expect_equal(tokens, ast, parent);

                parse_argument(tokens, ast, parent);
            }

            _ => panic!("Expected Symbol. Found: {:?}", token),
        }
    }

    expect_comma(tokens, ast, parent);
}

/// Intrinsic, If+Else, Loop, Break, Let, Set
pub fn parse_statement(tokens: &mut Peekable<Iter<Token>>, ast: &mut Arena<AstNode>, parent: NodeId)
{
    if let Some(token) = tokens.next()
    {
        println!("  STATEMENT: {:?}", token);
        match token
        {
            Token::Intrinsic =>
            {
                parse_intrinsic_call(tokens, ast, parent);

                // Top-level statement, expect a comma.
                expect_comma(tokens, ast, parent);
            }

            Token::If => parse_if(tokens, ast, parent),

            Token::Loop => parse_loop(tokens, ast, parent),

            Token::Break => parse_break(tokens, ast, parent),

            Token::Let => parse_let(tokens, ast, parent),

            Token::Set => parse_set(tokens, ast, parent),

            // Token::Comma => return,

            // _ => unreachable!(),
            _ =>
            {
                panic!("ERROR: Found token {:?} instead of statement", token);
            }
        }
    }

    // assert!(expect_comma(tokens, ast), "Expected comma to end statement", parent);
}

pub fn parse(tokens: Vec<Token>)
{
    // println!("TOKENS: {:?}", tokens);

    let mut it = tokens.iter().peekable();
    it.peek();

    let mut ast = &mut Arena::new();
    let root = ast.new_node(AstNode::Program);

    while let Some(token) = it.peek()
    {
        println!("PARSER STATE: {:?}", token);

        parse_statement(&mut it, ast, root);
    }

    println!("\n---------------\n");




    // // Program -> Intrinsic("print") -> U64(123)
    // let a = ast.new_node(AstNode::Intrinsic("print"));
    // root.append(a, ast);

    // let b = ast.new_node(AstNode::U64(123));
    // a.append(b, ast);

    for child in root.children(ast)
    {
        println!("ASTNODE: {:?}", ast[child].get());
    }
}
