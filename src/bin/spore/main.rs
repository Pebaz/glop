/*
1. Use the right tool for the job: load PE in Python
2. Pipe bytecode to STDIN
3. Composeable programs are beautiful. Need to get interface right instead of
Bash. What if polyglot programs exposed same interface?
4. Do it right the first time. Don't forget to document each program.
*/

use std::io::prelude::*;

/// Reads in an EFI Bytecode file from STDIN and prints the disassembly.
fn main()
{
    for bytecode_file in std::env::args().skip(1).take(1)
    {
        println!("{}", bytecode_file);

        let mut file = std::fs::File::open(bytecode_file.clone())
            .expect(format!("File {} does not exist", bytecode_file).as_str());
        let mut bytes = file.bytes().map(|b| b.unwrap());
        let mut instruction = [0; 4];

        for (i, byte) in bytes.enumerate()
        {
            instruction[i % 4] = byte;

            if i % 4 == 0 && i > 0
            {
                println!("{:?}", instruction);
            }
        }

        // TODO(pbz): Bytes can be left over in the instruction. Process them.

        // loop
        // {
        //     let byte = bytes.next();
        //     if byte.is_none() { break; }

        //     println!("{:?}", byte.unwrap());
        // }
    }
}
