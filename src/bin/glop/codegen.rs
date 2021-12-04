use std::collections::HashSet;
use std::io::{Read, Write};
use std::fs::File;
use indextree::{Arena, NodeId};
use crate::parser::AstNode;

const PRELUDE: &'static str = include_str!("../../../asm/prelude.inc");
const POSTLUDE: &'static str = include_str!("../../../asm/postlude.inc");

fn depth_first_search(ast: &Arena<AstNode>, node: NodeId, depth: usize)
{
    let tab = "    ".repeat(depth);
    println!("{}{:?}", tab, ast[node].get());

    for child in node.children(ast)
    {
        depth_first_search(ast, child, depth + 1);
    }
}

pub fn generate_efi_bytecode_asm(
    mut out_file: File,
    mut ast: Arena<AstNode>,
    root: NodeId
) -> ()
{
    let ast = &mut ast;

    out_file.write_fmt(format_args!("{}", PRELUDE)).unwrap();

    let mut stack = Vec::with_capacity(32);
    stack.push(root);

    while let Some(node) = stack.pop()
    {
        match ast[node].get()
        {
            AstNode::Program =>
            {
                for child in node.children(ast)
                {
                    println!("ADDING: {:?} from {:?}", ast[child].get(), ast[node].get());
                }

                let children = node.children(ast).collect::<Vec<NodeId>>();

                for child in children.into_iter().rev()
                {
                    stack.push(child);
                }
            }

            // Block,
            // IfElse,
            // IfElseCondition,
            // IfElseTruthyBlock,
            // IfElseFalseyBlock,
            // Loop,
            // Break,
            // Let(String),
            // Set(String),
            // Intrinsic(String),
            // Lookup(String),
            // U64(u64),

            _ => panic!("Unexpected AstNode: {:?}", node),
        }
    }

    let mut variables = HashSet::new();

    variables.insert(String::from("HI"));

    out_file.write_fmt(format_args!("{}", POSTLUDE)).unwrap();
}

/*
fn gen_if_statement(node: &AstNode)
{
    gen_argument(node.condition);

    gen_block(node.truthy_block);

    gen_block(node.falsey_block);
}
*/
