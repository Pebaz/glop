use std::collections::{HashSet, HashMap};
use std::io::Write;
use std::fs::File;
use indextree::{Arena, NodeId};
use crate::parser::AstNode;

const PRELUDE: &'static str = include_str!("prelude.inc");
const POSTLUDE: &'static str = include_str!("postlude.inc");
const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct CompilerContext<'a>
{
    section: String,
    variable_section: String,
    ast: &'a mut Arena<AstNode>,
    variables: HashSet<String>,
    constants: HashMap<u64, String>,
    loop_stack: Vec<(String, u16)>,
    if_counter: u16,
    loop_counter: u16,
}

/// Pushes a U64, Symbol Lookup, or Intrinsic Call to the stack.
fn generate_push_argument(context: &mut CompilerContext, node: NodeId) -> ()
{
    match context.ast[node].get()
    {
        AstNode::U64(value) =>
        {
            if !context.constants.contains_key(value)
            {
                context.constants.insert(
                    *value,
                    String::from(format!("const_{}", context.constants.len()))
                );
            }

            let constant_name = context.constants.get(value).unwrap();

            // PushU64
            context.section += &format!("    MOVREL R1, {}\n", constant_name);
            context.section += &format!("    PUSH64 R1\n\n");
        }

        AstNode::Lookup(symbol) =>
        {
            // Push Address (remember to lookup)
            context.section += &format!("    MOVREL R1, {}\n", symbol);
            context.section += &format!("    PUSH64 R1\n\n");
        }

        AstNode::Intrinsic(_) =>
        {
            generate_intrinsic(context, node);
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
fn generate_intrinsic(context: &mut CompilerContext, node: NodeId) -> ()
{
    let function_name = match context.ast[node].get()
    {
        AstNode::Intrinsic(symbol_name) =>
        {
            symbol_name.to_uppercase().replace('-', "")
        }

        _ => panic!("Expected an intrinsic call, got {:?}", node),
    };

    // Push each argument onto the stack
    let children: Vec<NodeId> = node.children(context.ast).collect();
    for child in children
    {
        generate_push_argument(context, child);
    }

    context.section += &format!("    ASMCALL {}\n\n", function_name);
}

fn generate_loop(context: &mut CompilerContext, node: NodeId) -> ()
{
    match context.ast[node].get()
    {
        AstNode::Loop => (),
        _ => panic!("Expected loop, got {:?}", node),
    }

    let loop_name = format!("loop_{}", context.loop_counter);
    context.loop_stack.push((loop_name.clone(), context.loop_counter));
    context.loop_counter += 1;

    context.section += &format!("{}:\n\n", loop_name);

    // Process each child statement
    let children: Vec<NodeId> = node.children(context.ast).collect();
    for child in children
    {
        generate_statement(context, child);
    }

    context.section += &format!("    JMP32 R0({})\n", loop_name);

    context.section += &format!("{}_break: PASS\n\n", loop_name);

    context.loop_stack.pop().unwrap();
}

fn generate_break(context: &mut CompilerContext, _node: NodeId) -> ()
{
    let loop_name = &context.loop_stack[context.loop_stack.len() - 1].0;
    context.section += &format!("    JMP32 R0({}_break)\n\n", loop_name);
}

/// Condition argument must push something to compare.
fn generate_if_else(context: &mut CompilerContext, node: NodeId) -> ()
{
    let if_name = format!("if_{}", context.if_counter);
    let true_name = format!("{}_truthy", if_name);
    let false_name = format!("{}_falsey", if_name);
    let end_if_name = format!("{}_end", if_name);
    context.if_counter += 1;

    let condition = context.ast[node].first_child().unwrap();
    let truthy_block = context.ast[condition].next_sibling().unwrap();
    let falsey_block = context.ast[truthy_block].next_sibling().unwrap();

    generate_statement(context, context.ast[condition].first_child().unwrap());

    context.section += &format!("{}:  ;; UNUSED LABEL\n", if_name);
    context.section += &format!("    POP64 R1\n");
    context.section += &format!("    MOVREL R4, literal_1\n");
    context.section += &format!("    CMP64eq R1, @R4\n");
    context.section += &format!("    MOVREL R1, {}\n", true_name);
    context.section += &format!("    JMP32cs R1\n");
    context.section += &format!("    MOVREL R1, {}\n", false_name);
    context.section += &format!("    JMP32cc R1\n");

    context.section += &format!("{}:\n", true_name);

    let children: Vec<NodeId> = truthy_block.children(context.ast).collect();
    for child in children
    {
        generate_statement(context, child);
    }

    context.section += &format!("    JMP32 R0({})\n", end_if_name);

    context.section += &format!("{}:\n", false_name);

    let children: Vec<NodeId> = falsey_block.children(context.ast).collect();
    for child in children
    {
        generate_statement(context, child);
    }

    context.section += &format!("    JMP32 R0({})\n", end_if_name);

    context.section += &format!("{}: PASS\n\n", end_if_name);
}


fn generate_statement(context: &mut CompilerContext, node: NodeId) -> ()
{
    match context.ast[node].get()
    {
        AstNode::Let(variable_name) =>
        {
            if context.variables.contains(variable_name)
            {
                panic!(
                    "Duplicate variable declaration for {:?}. \
                    {} supports only a global scope as of v{}.",
                    variable_name,
                    CRATE_NAME,
                    CRATE_VERSION
                );
            }
            context.variables.insert(variable_name.to_string());

            let variable_name = variable_name.replace('-', "_");

            context.variable_section += &format!(
                "    {}: rb 8\n",
                &variable_name
            );

            generate_push_argument(
                context,
                context.ast[node].first_child().unwrap()
            );

            // The top of the stack now contains the value to assign
            context.section += &format!("    POP64 R2\n");
            context.section += &format!("    MOVREL R1, {}\n", variable_name);
            context.section += &format!("    MOVq @R1, @R2\n\n");
        }

        AstNode::Set(variable_name) =>
        {
            if !context.variables.contains(variable_name)
            {
                panic!("Undeclared variable {:?}.", variable_name);
            }

            let variable_name = variable_name.replace('-', "_");

            generate_push_argument(
                context,
                context.ast[node].first_child().unwrap()
            );

            // The top of the stack now contains the value to assign
            context.section += &format!("    POP64 R2\n");
            context.section += &format!("    MOVREL R1, {}\n", variable_name);
            context.section += &format!("    MOVq @R1, @R2\n\n");
        }

        AstNode::Intrinsic(_) =>
        {
            generate_intrinsic(context, node);
        }

        AstNode::Loop =>
        {
            generate_loop(context, node);
        }

        AstNode::Break =>
        {
            generate_break(context, node);
        }

        AstNode::IfElse =>
        {
            generate_if_else(context, node);
        }

        _ => unreachable!(),
    }
}

pub fn generate_efi_bytecode_asm(
    mut out_file: File,
    mut ast: Arena<AstNode>,
    root: NodeId
) -> ()
{

    let ast = &mut ast;
    let variable_section = String::with_capacity(1024);
    let body_section = String::with_capacity(1024);
    let constants = HashMap::new();
    let variables = HashSet::new();
    let if_counter = 0u16;
    let loop_counter = 0u16;

    let mut loop_stack = Vec::with_capacity(8);
    loop_stack.push((String::from("PLACEHOLDER"), 0));

    out_file.write_fmt(format_args!("{}", PRELUDE)).unwrap();

    let mut context = CompilerContext
    {
        section: body_section,
        variable_section,
        ast,
        variables,
        constants,
        loop_stack,
        if_counter,
        loop_counter,
    };

    let children: Vec<NodeId> = root.children(context.ast).collect();

    for child in children
    {
        generate_statement(&mut context, child);
    }

    out_file.write_fmt(format_args!("    ;; Initialize Variables\n")).unwrap();
    out_file.write_fmt(format_args!("{}", context.section)).unwrap();
    out_file.write_fmt(format_args!("{}", POSTLUDE)).unwrap();
    out_file.write_fmt(format_args!("    ;; Variables\n")).unwrap();
    out_file.write_fmt(
        format_args!("{}\n", context.variable_section)
    ).unwrap();
    out_file.write_fmt(format_args!("    ;; Constants\n")).unwrap();

    for (constant, name) in context.constants.iter()
    {
        out_file.write_fmt(
            format_args!("    {}: dq {}\n", name, constant)
        ).unwrap();
    }
}
