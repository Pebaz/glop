use std::collections::HashSet;
use std::io::{Read, Write};
use std::fs::File;
use indextree::{Arena, NodeId};
use crate::parser::AstNode;

const PRELUDE: &'static str = include_str!("../../../asm/prelude.inc");
const POSTLUDE: &'static str = include_str!("../../../asm/postlude.inc");
const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn depth_first_search(ast: &Arena<AstNode>, node: NodeId, depth: usize)
{
    let tab = "    ".repeat(depth);
    println!("{}{:?}", tab, ast[node].get());

    for child in node.children(ast)
    {
        depth_first_search(ast, child, depth + 1);
    }
}

pub fn generate_variable_declarations(
    mut out_file: &mut File,
    variables: HashSet<&String>
) -> ()
{
    for variable in variables.into_iter()
    {
        let variable = variable.replace('-', "_");
        out_file.write_fmt(format_args!("    {}: rb 8\n", variable)).unwrap();
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

    let mut variables = HashSet::new();
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

            AstNode::Let(variable_name) =>
            {
                if variables.contains(variable_name)
                {
                    panic!(
                        "Duplicate variable declaration for {:?}. \
                        {} supports only a global scope as of v{}.",
                        variable_name,
                        CRATE_NAME,
                        CRATE_VERSION
                    );
                }
                variables.insert(variable_name);
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

            _ =>
            {
                println!("\n-------------------\n");
                println!("- CODE GENERATOR STATE -");

                for variable in variables.iter()
                {
                    println!("VARIABLE: {}", variable);
                }

                println!("\n-------------------\n");

                // panic!("Unexpected AstNode: {:?}", ast[node].get());
                break;
            }
        }
    }

    out_file.write_fmt(format_args!("{}", POSTLUDE)).unwrap();

    generate_variable_declarations(&mut out_file, variables);
}

/*
fn gen_if_statement(node: &AstNode)
{
    gen_argument(node.condition);

    gen_block(node.truthy_block);

    gen_block(node.falsey_block);
}
*/
