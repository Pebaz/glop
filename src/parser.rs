use std::iter::Peekable;
use std::slice::Iter;
use indextree::{Arena, NodeId};
use crate::lexer::Token;

const TRACE_EXECUTION_ENABLED: bool = false;

#[derive(Debug)]
pub enum AstNode
{
    Program,

    IfElse,

    IfElseCondition,

    IfElseTruthyBlock,

    IfElseFalseyBlock,

    Loop,  // [Loop Block]

    Break,

    Let(String),  // [Value]

    Set(String),  // [Value]

    Intrinsic(String),  // [Arguments ...]

    // * No need for this since Let/Set/Intrinsic store their names
    // Symbol(String),

    Lookup(String),

    U64(u64),
}

pub struct ParserContext<'a>
{
    pub tokens: &'a mut Peekable<Iter<'a, Token<'a>>>,
    pub ast: &'a mut Arena<AstNode>
}

pub fn expect_call_open(context: &mut ParserContext, _parent: NodeId) -> ()
{
    if let Some(token) = context.tokens.next()
    {
        match token
        {
            Token::CallOpen => return,
            _ => panic!("PARSE ERROR: Expected open paren, found {:?}", token),
        }
    }
}

pub fn expect_call_close(context: &mut ParserContext, _parent: NodeId) -> ()
{
    if let Some(token) = context.tokens.next()
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

pub fn expect_comma(context: &mut ParserContext, _parent: NodeId) -> ()
{
    if let Some(token) = context.tokens.next()
    {
        match token
        {
            Token::Comma => return,
            _ => panic!("PARSE ERROR: Expected comma, found {:?}", token),
        }
    }
}

pub fn expect_equal(context: &mut ParserContext, _parent: NodeId) -> ()
{
    if let Some(token) = context.tokens.next()
    {
        match token
        {
            Token::Equal => return,
            _ => panic!("PARSE ERROR: Expected equal sign, found {:?}", token),
        }
    }
}

pub fn expect_block_open(context: &mut ParserContext, _parent: NodeId) -> ()
{
    if let Some(token) = context.tokens.next()
    {
        match token
        {
            Token::BlockOpen => return,
            _ => panic!("PARSE ERROR: Expected open block, found {:?}", token),
        }
    }
}

pub fn expect_else(context: &mut ParserContext, _parent: NodeId) -> ()
{
    if let Some(token) = context.tokens.next()
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

/// U64, Intrinsic, Symbol
fn parse_argument(context: &mut ParserContext, parent: NodeId) -> ()
{
    if let Some(token) = context.tokens.next()
    {
        match token
        {
            Token::U64(value) =>
            {
                parent.append(context.ast.new_node(AstNode::U64(*value)), context.ast);
            }

            Token::Intrinsic => parse_intrinsic_call(context, parent),

            Token::Symbol(symbol) =>
            {
                parent.append(
                    context.ast.new_node(AstNode::Lookup(symbol.to_string())),
                    context.ast
                );
            }

            _ => panic!(
                "Expected a U64, Intrinsic Call, or Symbol. Found: {:?}",
                token
            ),
        }
    }
}

pub fn parse_intrinsic_call(context: &mut ParserContext, parent: NodeId) -> ()
{
    if let Some(Token::Symbol(symbol)) = context.tokens.next()
    {
        let intrinsic_call = context.ast.new_node(
            AstNode::Intrinsic(symbol.to_string())
        );
        parent.append(intrinsic_call, context.ast);

        expect_call_open(context, intrinsic_call);

        while let Some(token) = context.tokens.peek()
        {
            let _ast_node = match token
            {
                Token::Comma =>
                {
                    context.tokens.next();
                }

                Token::CallClose => break,

                _ => parse_argument(context, intrinsic_call),
            };
        }

        expect_call_close(context, intrinsic_call);

        if TRACE_EXECUTION_ENABLED
        {
            println!("      INTRINSIC CALL: {:?}", symbol);
        }
    }
    else
    {
        unreachable!();
    }
}

pub fn parse_if(context: &mut ParserContext, parent: NodeId) -> ()
{
    if TRACE_EXECUTION_ENABLED
    {
        println!("PARSE IF");
    }

    let if_statement = context.ast.new_node(AstNode::IfElse);
    parent.append(if_statement, context.ast);

    let condition = context.ast.new_node(AstNode::IfElseCondition);
    if_statement.append(condition, context.ast);

    let truthy_block = context.ast.new_node(AstNode::IfElseTruthyBlock);
    if_statement.append(truthy_block, context.ast);

    let falsey_block = context.ast.new_node(AstNode::IfElseFalseyBlock);
    if_statement.append(falsey_block, context.ast);

    parse_argument(context, condition);

    expect_block_open(context, if_statement);

    while let Some(token) = context.tokens.peek()
    {
        if TRACE_EXECUTION_ENABLED
        {
            println!("    IF BLOCK PARSER STATE: {:?}", token);
        }

        if let Token::BlockClose = token
        {
            if TRACE_EXECUTION_ENABLED
            {
                println!("HERE. Breaking out of if block");
            }

            context.tokens.next();
            break;
        }
        else
        {
            if TRACE_EXECUTION_ENABLED
            {
                println!("    IF BLOCK STATEMENT: {:?}", token);
            }

            parse_statement(context, truthy_block);
        }
    }

    expect_else(context, if_statement);

    expect_block_open(context, if_statement);

    while let Some(token) = context.tokens.peek()
    {
        if TRACE_EXECUTION_ENABLED
        {
            println!("    ELSE BLOCK PARSER STATE: {:?}", token);
        }

        if let Token::BlockClose = token
        {
            if TRACE_EXECUTION_ENABLED
            {
                println!("HERE. Breaking out of else block");
            }

            context.tokens.next();
            break;
        }
        else
        {
            if TRACE_EXECUTION_ENABLED
            {
                println!("    ELSE BLOCK STATEMENT: {:?}", token);
            }

            parse_statement(context, falsey_block);
        }
    }
}

/// Intrinsic, If+Else, Loop, Break, Let, Set
pub fn parse_block(context: &mut ParserContext, parent: NodeId) -> ()
{
    expect_block_open(context, parent);

    while let Some(token) = context.tokens.peek()
    {
        if TRACE_EXECUTION_ENABLED
        {
            println!("  BLOCK PARSER STATE: {:?}", token);
        }

        if let Token::BlockClose = token
        {
            context.tokens.next();
            break;
        }
        else
        {
            parse_statement(context, parent);
        }
    }
}

pub fn parse_loop(context: &mut ParserContext, parent: NodeId) -> ()
{
    let loop_statement = context.ast.new_node(AstNode::Loop);
    parent.append(loop_statement, context.ast);

    parse_block(context, loop_statement);
}

pub fn parse_break(context: &mut ParserContext, parent: NodeId) -> ()
{
    let break_statement = context.ast.new_node(AstNode::Break);
    parent.append(break_statement, context.ast);

    expect_comma(context, break_statement);
}

pub fn parse_let(context: &mut ParserContext, parent: NodeId) -> ()
{
    if let Some(token) = context.tokens.next()
    {
        match token
        {
            Token::Symbol(symbol) =>
            {
                let let_statement = context.ast.new_node(
                    AstNode::Let(symbol.to_string())
                );
                parent.append(let_statement, context.ast);

                expect_equal(context, let_statement);

                parse_argument(context, let_statement);
            }

            _ => panic!("Expected Symbol. Found: {:?}", token),
        }
    }

    expect_comma(context, parent);
}

pub fn parse_set(context: &mut ParserContext, parent: NodeId) -> ()
{
    if let Some(token) = context.tokens.next()
    {
        match token
        {
            Token::Symbol(symbol) =>
            {
                let set_statement = context.ast.new_node(
                    AstNode::Set(symbol.to_string())
                );
                parent.append(set_statement, context.ast);

                expect_equal(context, set_statement);

                parse_argument(context, set_statement);
            }

            _ => panic!("Expected Symbol. Found: {:?}", token),
        }
    }

    expect_comma(context, parent);
}

/// Intrinsic, If+Else, Loop, Break, Let, Set
pub fn parse_statement(context: &mut ParserContext, parent: NodeId) -> ()
{
    if let Some(token) = context.tokens.next()
    {
        if TRACE_EXECUTION_ENABLED { println!("  STATEMENT: {:?}", token); }

        match token
        {
            Token::Intrinsic =>
            {
                parse_intrinsic_call(context, parent);

                // Top-level statement, expect a comma.
                expect_comma(context, parent);
            }

            Token::If => parse_if(context, parent),

            Token::Loop => parse_loop(context, parent),

            Token::Break => parse_break(context, parent),

            Token::Let => parse_let(context, parent),

            Token::Set => parse_set(context, parent),

            _ =>
            {
                panic!("ERROR: Found token {:?} instead of statement", token);
            }
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> (Arena<AstNode>, NodeId)
{
    let mut it = tokens.iter().peekable();
    it.peek();

    let mut ast = Arena::new();
    let root = ast.new_node(AstNode::Program);

    let mut context = ParserContext
    {
        tokens: &mut it,
        ast: &mut ast,
    };

    while let Some(token) = context.tokens.peek()
    {
        if TRACE_EXECUTION_ENABLED { println!("PARSER STATE: {:?}", token); }

        parse_statement(&mut context, root);
    }

    if TRACE_EXECUTION_ENABLED { emit_ast_node(&ast, root, 0); }

    (ast, root)
}

fn emit_ast_node(ast: &Arena<AstNode>, node: NodeId, depth: usize)
{
    let tab = "    ".repeat(depth);
    println!("{}{:?}", tab, ast[node].get());

    for child in node.children(ast)
    {
        emit_ast_node(ast, child, depth + 1);
    }
}
