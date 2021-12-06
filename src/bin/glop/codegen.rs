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

// fn generate_argument(
//     section: &mut String,
//     ast: &Arena<AstNode>,
//     node: NodeId,
//     variable_name: String,
//     constants: &mut HashMap<u64, String>
// ) -> ()
// {
//     let mut stack = Vec::with_capacity(32);
//     stack.push(node);

//     while let Some(node) = stack.pop()
//     {
//         match ast[node].get()
//         {
//             AstNode::U64(value) =>
//             {
//                 if !constants.contains_key(value)
//                 {
//                     constants.insert(*value, String::from(format!("const_{}", constants.len())));
//                 }

//                 let constant_name = constants.get(value).unwrap();

//                 *section += &format!("    MOVREL R1, {}\n", variable_name);
//                 *section += &format!("    MOVREL R2, {}\n", constant_name);
//                 *section += &format!("    MOVq R1, R2\n\n");
//             }

//             AstNode::Lookup(symbol) =>
//             {
//                 // *section += &format!("    MOVREL R1, {}\n", variable_name);
//                 // *section += &format!("    MOVREL R2, {}\n", symbol);
//                 // *section += &format!("    MOVq R1, R2\n\n");
//             }

//             ast_node @ _ => panic!(
//                 "INTERNAL ERROR: Unexpected AstNode in variable assignment: {:?}",
//                 ast_node
//             ),
//         }
//     }
// }

/// Pushes a U64, Symbol Lookup, or Intrinsic Call to the stack.
fn generate_push_argument(
    section: &mut String,
    variable_section: &mut String,
    ast: &Arena<AstNode>,
    node: NodeId,
    variables: &mut HashSet<String>,
    constants: &mut HashMap<u64, String>,
    loop_stack: &mut Vec<(String, u16)>,
) -> ()
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

            // PushU64
            *section += &format!("    MOVREL R1, {}\n", constant_name);
            *section += &format!("    PUSH64 R1\n\n");
        }

        AstNode::Lookup(symbol) =>
        {
            // Push Address (remember to lookup)
            *section += &format!("    MOVREL R1, {}\n", symbol);
            *section += &format!("    PUSH64 R1\n\n");
        }

        AstNode::Intrinsic(_) =>
        {
            generate_intrinsic(section, variable_section, ast, node, variables, constants, loop_stack);
        }

        ast_node @ _ => panic!(
            "INTERNAL ERROR: Unexpected AstNode as argument: {:?}",
            ast_node
        ),
    }
}

/// Push result of function onto stack. If the intrinsic doesn't push anything
/// onto the stack, it shouldn't be used during an assignment or there will be
/// stack consequences.
fn generate_intrinsic(
    section: &mut String,
    variable_section: &mut String,
    ast: &Arena<AstNode>,
    node: NodeId,
    variables: &mut HashSet<String>,
    constants: &mut HashMap<u64, String>,
    loop_stack: &mut Vec<(String, u16)>,
) -> ()
{
    let function_name = match ast[node].get()
    {
        AstNode::Intrinsic(symbol_name) => symbol_name.to_uppercase().replace('-', ""),
        _ => panic!("Expected an intrinsic call, got {:?}", node),
    };

    // Push each argument onto the stack
    for child in node.children(ast)
    {
        generate_push_argument(section, variable_section, ast, child, variables, constants, loop_stack);
    }

    *section += &format!("    ASMCALL {} \n\n", function_name);
}

/*
loop_1:  ;; Special blocks start with their name?
    loop_2:  ;; Special blocks start with their name?
        JMP32 R0(loop_2_break)  ;; Break the loop

        JMP32 R0(loop_2)
    loop_2_break: PASS

    JMP32 R0(loop_1_break)  ;; Break the loop

    JMP32 R0(loop_1)
loop_1_break: PASS
*/
fn generate_loop(
    section: &mut String,
    variable_section: &mut String,
    ast: &Arena<AstNode>,
    node: NodeId,
    variables: &mut HashSet<String>,
    constants: &mut HashMap<u64, String>,
    loop_stack: &mut Vec<(String, u16)>,
) -> ()
{
    match ast[node].get()
    {
        AstNode::Loop => (),
        _ => panic!("Expected loop, got {:?}", node),
    }

    let loop_counter = loop_stack[loop_stack.len() - 1].1;
    let loop_name = format!("loop_{}", loop_counter);
    loop_stack.push((loop_name.clone(), loop_counter + 1));

    *section += &format!("{}:\n\n", loop_name);

    // Process each child statement
    for child in node.children(ast)
    {
        generate_statement(section, variable_section, ast, child, variables, constants, loop_stack);
    }

    *section += &format!("    JMP32 R0({})\n", loop_name);

    *section += &format!("{}_break: PASS\n\n", loop_name);
}

fn generate_break(
    section: &mut String,
    variable_section: &mut String,
    ast: &Arena<AstNode>,
    node: NodeId,
    constants: &mut HashMap<u64, String>,
    loop_stack: &mut Vec<(String, u16)>,
) -> ()
{
    let loop_name = &loop_stack[loop_stack.len() - 1].0;
    *section += &format!("    JMP32 R0({}_break): PASS\n\n", loop_name);
}

fn generate_statement(
    section: &mut String,
    variable_section: &mut String,
    ast: &Arena<AstNode>,
    node: NodeId,
    variables: &mut HashSet<String>,
    constants: &mut HashMap<u64, String>,
    loop_stack: &mut Vec<(String, u16)>,
) -> ()
{
    match ast[node].get()
    {
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
            variables.insert(variable_name.to_string());

            let variable_name = variable_name.replace('-', "_");

            *variable_section += &format!(
                "    {}: rb 8\n",
                &variable_name
            );

            generate_push_argument(
                section,
                variable_section,
                ast,
                ast[node].first_child().unwrap(),
                variables,
                constants,
                loop_stack
            );

            // The top of the stack now contains the value to assign
            *section += &format!("    POP64 R2\n");
            *section += &format!("    MOVREL R1, {}\n", variable_name);
            *section += &format!("    MOVq @R1, @R2\n\n");
        }

        // AstNode::Intrinsic(intrinsic_name) =>
        // {
        //     let intrinsic_name = intrinsic_name.to_uppercase().replace("-", "");

        //     println!("INTRINSIC: {}", intrinsic_name);
        // }

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

        AstNode::Intrinsic(_) =>
        {
            generate_intrinsic(
                section,
                variable_section,
                ast,
                node,
                variables,
                constants,
                loop_stack
            );
        }

        AstNode::Loop =>
        {
            generate_loop(
                section,
                variable_section,
                ast,
                node,
                variables,
                constants,
                loop_stack
            );
        }

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
            return;
        }
    }
}

pub fn generate_efi_bytecode_asm(
    mut out_file: File,
    mut ast: Arena<AstNode>,
    root: NodeId
) -> ()
{

    let ast = &mut ast;
    let mut variable_section = String::with_capacity(1024);
    let mut body_section = String::with_capacity(1024);
    let mut constants = HashMap::new();
    let mut variables = HashSet::new();

    let mut loop_stack = Vec::with_capacity(8);
    loop_stack.push((String::from("PLACEHOLDER"), 0));

    out_file.write_fmt(format_args!("{}", PRELUDE)).unwrap();

    for child in root.children(ast)
    {
        generate_statement(&mut body_section, &mut variable_section, ast, child, &mut variables, &mut constants, &mut loop_stack);
    }

    out_file.write_fmt(format_args!("    ;; Initialize Variables\n")).unwrap();
    out_file.write_fmt(format_args!("{}", body_section)).unwrap();
    out_file.write_fmt(format_args!("{}", POSTLUDE)).unwrap();
    out_file.write_fmt(format_args!("    ;; Variables\n")).unwrap();
    out_file.write_fmt(format_args!("{}\n", variable_section)).unwrap();
    out_file.write_fmt(format_args!("    ;; Constants\n")).unwrap();

    for (constant, name) in constants.iter()
    {
        out_file.write_fmt(
            format_args!("    {}: dq {}\n", name, constant)
        ).unwrap();
    }
}

/*
fn gen_if_statement(node: &AstNode)
{
    gen_argument(node.condition);

    gen_block(node.truthy_block);

    gen_block(node.falsey_block);
}
*/
