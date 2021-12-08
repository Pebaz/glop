

// TODO(pbz): Finalize the CLI


mod lexer;
mod parser;
mod codegen;

use std::io::{Read, Write};
use logos::*;
use fasmg_ebc_rs::assemble_ebc;
use lexer::Token;
use parser::parse;
use codegen::generate_efi_bytecode_asm;

fn main()
{
    let filename = std::env::args().skip(1).next().unwrap();
    let mut source_file = std::fs::File::open(filename).unwrap();
    let mut source_code = String::with_capacity(2048);
    source_file.read_to_string(&mut source_code).unwrap();

    let temp_dir = std::path::Path::new(&std::env::temp_dir()).join(
        std::env::var("CARGO_PKG_NAME").unwrap()
    );
    std::fs::create_dir_all(&temp_dir).unwrap();

    let assembly_filename = temp_dir.join("a.asm");
    let mut assembly_file = std::fs::File::create(&assembly_filename).unwrap();

    let bin_filename = std::env::args().skip(2).next().unwrap();

    let mut lexer = Token::lexer(&source_code);
    let mut tokens = Vec::new();

    while let Some(i) = lexer.next()
    {
        if let Token::Error = i
        {
            println!("ERROR AT: {:?}", lexer.slice());
            break;
        }
        else
        {
            tokens.push(i);
        }
    }

    let (ast, root) = parse(tokens);

    {
        generate_efi_bytecode_asm(assembly_file, ast, root);
    }

    // Make sure fasmg-ebc-rs can see the include file
    {
        let full_path = temp_dir.join("instructions.inc");
        let mut file = std::fs::File::create(full_path).unwrap();
        file.write_all(include_bytes!("instructions.inc")).unwrap();
    }

    assemble_ebc(
        &assembly_filename.into_os_string().into_string().unwrap(),
        &bin_filename
    );
}
