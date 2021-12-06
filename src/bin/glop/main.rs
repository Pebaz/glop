mod lexer;
mod parser;
mod codegen;

use std::io::Read;
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

    let out_filename = std::env::args().skip(2).next().unwrap();
    let mut output_file = std::fs::File::create(&out_filename).unwrap();
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
            // * println!("TOKEN: {:?}", i);
            tokens.push(i);
        }
    }

    let (ast, root) = parse(tokens);

    generate_efi_bytecode_asm(output_file, ast, root);

    assemble_ebc(&out_filename, "drive/EFI/BOOT/BOOTX64.efi");
}
