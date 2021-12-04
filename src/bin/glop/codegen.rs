use std::collections::{HashSet, HashMap};
use std::io::Write;
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

// pub fn generate_variable_initializers(
//     mut out_file: &mut File,
//     ast: &Arena<AstNode>,
//     initializers: Vec<NodeId>,
// ) -> ()
// {
//     for node in initializers.into_iter()
//     {
//         if let Some(variable) = ast[node].get()
//         {
//             let variable = variable.replace('-', "_");

//             out_file.write_fmt(
//                 format_args!(
//                     "    MOVREL R1 {}\n \

//                     ",
//                     variable
//                 )
//             ).unwrap();
//         }
//     }
// }

fn generate_argument(
    section: &mut String,
    ast: &Arena<AstNode>,
    node: NodeId,
    variable_name: String,
    constants: &mut HashMap<u64, String>
) -> ()
{
    let mut stack = Vec::with_capacity(32);
    stack.push(node);

    while let Some(node) = stack.pop()
    {
        match ast[node].get()
        {
            AstNode::U64(value) =>
            {
                if !constants.contains_key(value)
                {
                    constants.insert(*value, String::from(format!("const_{}", constants.len())));
                }

                let constant_name = constants.get(value).unwrap();

                *section += &format!("    MOVREL R1, {}\n", variable_name);
                *section += &format!("    MOVREL R2, {}\n", constant_name);
                *section += &format!("    MOVq R1, R2\n\n");
            }

            ast_node @ _ => panic!(
                "INTERNAL ERROR: Unexpected AstNode in variable assignment: {:?}",
                ast_node
            ),
        }
    }
}

fn generate_constants() { }

pub fn generate_efi_bytecode_asm(
    mut out_file: File,
    mut ast: Arena<AstNode>,
    root: NodeId
) -> ()
{
    let ast = &mut ast;

    out_file.write_fmt(format_args!("{}", PRELUDE)).unwrap();

    let mut variable_section = String::with_capacity(1024);
    let mut variable_init_section = String::with_capacity(1024);

    let mut constants = HashMap::new();
    let mut variables = HashSet::new();
    // let mut variable_initializers = Vec::with_capacity(32);
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

                let variable_name = variable_name.replace('-', "_");

                variable_section += &format!(
                    "    {}: rb 8\n",
                    variable_name
                );

                generate_argument(
                    &mut variable_init_section,
                    ast,
                    ast[node].first_child().unwrap(),
                    variable_name,
                    &mut constants
                );
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

    // generate_variable_initializers(&mut out_file, ast, variable_initializers);
    out_file.write_fmt(format_args!("{}", variable_init_section)).unwrap();

    out_file.write_fmt(format_args!("{}", POSTLUDE)).unwrap();
    out_file.write_fmt(format_args!("{}", variable_section)).unwrap();
}

/*
fn gen_if_statement(node: &AstNode)
{
    gen_argument(node.condition);

    gen_block(node.truthy_block);

    gen_block(node.falsey_block);
}
*/
