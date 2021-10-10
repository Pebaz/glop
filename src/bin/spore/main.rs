/*
1. Use the right tool for the job: load PE in Python
2. Pipe bytecode to STDIN
3. Composeable programs are beautiful. Need to get interface right instead of
Bash. What if polyglot programs exposed same interface?
4. Do it right the first time. Don't forget to document each program.
*/

/// Reads in an EFI Bytecode file from STDIN and prints the disassembly.
fn main()
{
    for bytecode_file in std::env::args().skip(1).take(1)
    {
        println!("{}", bytecode_file);
    }
}
