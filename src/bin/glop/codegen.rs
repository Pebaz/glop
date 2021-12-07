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

/// Pushes a U64, Symbol Lookup, or Intrinsic Call to the stack.
fn generate_push_argument(
    section: &mut String,
    variable_section: &mut String,
    ast: &Arena<AstNode>,
    node: NodeId,
    variables: &mut HashSet<String>,
    constants: &mut HashMap<u64, String>,
    loop_stack: &mut Vec<(String, u16)>,
    if_counter: &mut u16,
    loop_counter: &mut u16,
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
            generate_intrinsic(
                section,
                variable_section,
                ast,
                node,
                variables,
                constants,
                loop_stack,
                if_counter,
                loop_counter,
            );
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
    if_counter: &mut u16,
    loop_counter: &mut u16,
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
        generate_push_argument(
            section,
            variable_section,
            ast,
            child,
            variables,
            constants,
            loop_stack,
            if_counter,
            loop_counter,
        );
    }

    *section += &format!("    ASMCALL {}\n\n", function_name);
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
    if_counter: &mut u16,
    loop_counter: &mut u16,
) -> ()
{
    match ast[node].get()
    {
        AstNode::Loop => (),
        _ => panic!("Expected loop, got {:?}", node),
    }

    let loop_name = format!("loop_{}", loop_counter);
    loop_stack.push((loop_name.clone(), *loop_counter));
    *loop_counter += 1;

    *section += &format!("{}:\n\n", loop_name);

    // Process each child statement
    for child in node.children(ast)
    {
        generate_statement(
            section,
            variable_section,
            ast,
            child,
            variables,
            constants,
            loop_stack,
            if_counter,
            loop_counter,
        );
    }

    *section += &format!("    JMP32 R0({})\n", loop_name);

    *section += &format!("{}_break: PASS\n\n", loop_name);

    loop_stack.pop().unwrap();
}

fn generate_break(
    section: &mut String,
    variable_section: &mut String,
    ast: &Arena<AstNode>,
    node: NodeId,
    variables: &mut HashSet<String>,
    constants: &mut HashMap<u64, String>,
    loop_stack: &mut Vec<(String, u16)>,
    if_counter: &mut u16,
    loop_counter: &mut u16,
) -> ()
{
    let loop_name = &loop_stack[loop_stack.len() - 1].0;
    *section += &format!("    JMP32 R0({}_break)\n\n", loop_name);
}

/// Condition argument must push something to compare.
// TODO(pbz): How to tell if it's a variable or not?
fn generate_if_else(
    section: &mut String,
    variable_section: &mut String,
    ast: &Arena<AstNode>,
    node: NodeId,
    variables: &mut HashSet<String>,
    constants: &mut HashMap<u64, String>,
    loop_stack: &mut Vec<(String, u16)>,
    if_counter: &mut u16,
    loop_counter: &mut u16,
) -> ()
{
    /*

    if_1_truthy:
        PUSHADDR string_if_1_truthy
        ASMCALL EMITSTR
        JMP32 R0(if_1_endif)
    if_1_falsey:
        PUSHADDR string_if_1_falsey
        ASMCALL EMITSTR
        JMP32 R0(if_1_endif)
    if_1_endif: PASS
    */

    let if_name = format!("if_{}", if_counter);
    let true_name = format!("{}_truthy", if_name);
    let false_name = format!("{}_falsey", if_name);
    let end_if_name = format!("{}_end", if_name);
    *if_counter += 1;

    let condition = ast[node].first_child().unwrap();
    let truthy_block = ast[condition].next_sibling().unwrap();
    let falsey_block = ast[truthy_block].next_sibling().unwrap();

    generate_statement(
        section,
        variable_section,
        ast,
        ast[condition].first_child().unwrap(),
        variables,
        constants,
        loop_stack,
        if_counter,
        loop_counter,
    );

    *section += &format!("{}:  ;; UNUSED LABEL\n", if_name);
    *section += &format!("    POP64 R1\n");
    *section += &format!("    MOVREL R4, literal_1\n");
    *section += &format!("    CMP64eq R1, @R4\n");
    // *section += &format!("    CMPI64eq R1, 1\n");
    *section += &format!("    MOVREL R1, {}\n", true_name);
    *section += &format!("    JMP32cs R1\n");
    *section += &format!("    MOVREL R1, {}\n", false_name);
    *section += &format!("    JMP32cc R1\n");

    *section += &format!("{}:\n", true_name);

    for child in truthy_block.children(ast)
    {
        generate_statement(
            section,
            variable_section,
            ast,
            child,
            variables,
            constants,
            loop_stack,
            if_counter,
            loop_counter,
        );
    }

    *section += &format!("    JMP32 R0({})\n", end_if_name);

    *section += &format!("{}:\n", false_name);

    for child in falsey_block.children(ast)
    {
        generate_statement(
            section,
            variable_section,
            ast,
            child,
            variables,
            constants,
            loop_stack,
            if_counter,
            loop_counter,
        );
    }

    *section += &format!("    JMP32 R0({})\n", end_if_name);

    *section += &format!("{}: PASS\n\n", end_if_name);
}

fn generate_statement(
    section: &mut String,
    variable_section: &mut String,
    ast: &Arena<AstNode>,
    node: NodeId,
    variables: &mut HashSet<String>,
    constants: &mut HashMap<u64, String>,
    loop_stack: &mut Vec<(String, u16)>,
    if_counter: &mut u16,
    loop_counter: &mut u16,
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

            let value_node = ast[ast[node].first_child().unwrap()].get();
            let value_is_address = match value_node
            {
                AstNode::Lookup(_) => true,
                AstNode::U64(_) => true,
                _ => false,
            };

            generate_push_argument(
                section,
                variable_section,
                ast,
                ast[node].first_child().unwrap(),
                variables,
                constants,
                loop_stack,
                if_counter,
                loop_counter,
            );

            // The top of the stack now contains the value to assign
            *section += &format!("    POP64 R2\n");

            if value_is_address
            {
                *section += &format!(
                    "    MOVq R2, @R2  ;; Assign variable to variable\n"
                );
            }

            *section += &format!("    MOVREL R1, {}\n", variable_name);
            *section += &format!("    MOVq @R1, R2\n\n");
        }

        AstNode::Set(variable_name) =>
        {
            if !variables.contains(variable_name)
            {
                panic!("Undeclared variable {:?}.", variable_name);
            }

            let variable_name = variable_name.replace('-', "_");

            let value_node = ast[ast[node].first_child().unwrap()].get();
            let value_is_address = match value_node
            {
                AstNode::Lookup(_) => true,
                AstNode::U64(_) => true,
                _ => false,
            };

            generate_push_argument(
                section,
                variable_section,
                ast,
                ast[node].first_child().unwrap(),
                variables,
                constants,
                loop_stack,
                if_counter,
                loop_counter,
            );

            // The top of the stack now contains the value to assign
            *section += &format!("    POP64 R2\n");

            if value_is_address
            {
                *section += &format!(
                    "    MOVq R2, @R2  ;; Assign variable to variable\n"
                );
            }

            *section += &format!("    MOVREL R1, {}\n", variable_name);
            *section += &format!("    MOVq @R1, R2\n\n");
        }

        AstNode::Intrinsic(_) =>
        {
            generate_intrinsic(
                section,
                variable_section,
                ast,
                node,
                variables,
                constants,
                loop_stack,
                if_counter,
                loop_counter,
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
                loop_stack,
                if_counter,
                loop_counter,
            );
        }

        AstNode::Break =>
        {
            generate_break(
                section,
                variable_section,
                ast,
                node,
                variables,
                constants,
                loop_stack,
                if_counter,
                loop_counter,
            );
        }

        AstNode::IfElse =>
        {
            generate_if_else(
                section,
                variable_section,
                ast,
                node,
                variables,
                constants,
                loop_stack,
                if_counter,
                loop_counter,
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
    let mut loop_counter: &mut u16;
    let mut body_section = String::with_capacity(1024);
    let mut constants = HashMap::new();
    let mut variables = HashSet::new();
    let mut if_counter = 0u16;

    let mut loop_stack = Vec::with_capacity(8);
    loop_stack.push((String::from("PLACEHOLDER"), 0));

    let mut loop_counter = 0u16;

    out_file.write_fmt(format_args!("{}", PRELUDE)).unwrap();

    for child in root.children(ast)
    {
        generate_statement(
            &mut body_section,
            &mut variable_section,
            ast,
            child,
            &mut variables,
            &mut constants,
            &mut loop_stack,
            &mut if_counter,
            &mut loop_counter,
        );
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
