use std::io::{Read, Write};
use indextree::{Arena, NodeId};
use crate::parser::AstNode;

const PRELUDE: &'static str = include_str!("../../../asm/prelude.inc");
const POSTLUDE: &'static str = include_str!("../../../asm/postlude.inc");

pub fn generate_efi_bytecode_asm(mut out_file: std::fs::File, ast: Arena<AstNode>)
{
    out_file.write_fmt(format_args!("{}", PRELUDE)).unwrap();

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
