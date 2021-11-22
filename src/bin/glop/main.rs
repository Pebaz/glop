use std::io::{Read, Write};
use std::iter::Peekable;
use std::str::Chars;

const PRELUDE: &'static str = include_str!("../../../asm/prelude.inc");
const POSTLUDE: &'static str = include_str!("../../../asm/postlude.inc");


#[inline]
fn is_space(chr: char) -> bool
{
    chr == '\n' || chr == '\r' || chr == ' ' || chr == '\t'
}

/// If this character is there, great, if not, no worries
/// Skips whitespace
fn accept_char(mut ptr: &mut Peekable<Chars>, chr: char) -> bool
{
    while let Some(character) = ptr.peek()
    {
        if is_space(*character)
        {
            ptr.next();
            continue;
        }

        if *character == chr
        {
            ptr.next();
            return true;
        }

        return false;
    }

    false
}

/// This character NEEDS to be there.
/// Skips whitespace
fn expect_char(mut ptr: &mut Peekable<Chars>, chr: char) -> bool
{
    accept_char(ptr, chr)
}

/// This character NEEDS to be there.
/// Skips whitespace
fn expect_string(mut ptr: &mut Peekable<Chars>, out_string: &mut String) -> bool
{
    if let Some(ch) = ptr.peek()
    {
        if *ch == '"'
        {
            ptr.next();
            while let Some(ch2) = ptr.peek()
            {
                if *ch2 == '"'
                {
                    ptr.next();
                    return true;
                }
                else
                {
                    out_string.push(*ch2);
                    ptr.next();
                }
            }
        }
    }

    while let Some(character) = ptr.peek()
    {
        let m0 = *character >= 'a' && *character <= 'z';
        let m1 = *character >= 'A' && *character <= 'Z';
        let m2 = *character >= '0' && *character <= '9';
        let m3 = *character == '-' || *character == '_';

        if m0 | m1 | m2 | m3
        {
            out_string.push(*character);
            ptr.next();
        }
        else
        {
            return out_string.len() > 0;
        }
    }
    out_string.len() > 0
}

fn main()
{
    let filename = std::env::args().skip(1).next().unwrap();
    let mut source_file = std::fs::File::open(filename).unwrap();
    let mut source_code = String::with_capacity(2048);
    source_file.read_to_string(&mut source_code).unwrap();

    let mut output_file = std::fs::File::create("a.asm").unwrap();
    output_file.write_fmt(format_args!("{}", PRELUDE)).unwrap();

    // TODO(pbz): Refine the parser at each stage
    println!("---------------------------------------------");


    let mut ptr = &mut source_code.chars().peekable();
    ptr.peek().unwrap();
    let mut symbol = String::with_capacity(64);

    loop
    {
        // It's ok for a comma from the previous line to be here
        accept_char(ptr, ',');

        if let None = ptr.peek()
        {
            break;
        }

        println!("-----> {:?}", ptr.peek());

        // Must have opening compiler intrinsic call
        if !expect_char(ptr, '@')
        {
            println!("ERROR: Expected call to compiler intrinsic");
            return;
        }

        // Must have intrinsic call name
        if !expect_string(ptr, &mut symbol)
        {
            println!("ERROR: Expected symbol");
            return;
        }

        println!("INTRINSIC CALL: {}", symbol);
        symbol.clear();

        if !expect_char(ptr, '(')
        {
            println!("ERROR: Expected opening parenthesis");
            return;
        }

        while !accept_char(ptr, ')')
        {
            // println!("ERROR: Expected closing parenthesis");
            // return;

            accept_char(ptr, ',');  // From last iteration

            if !expect_string(ptr, &mut symbol)
            {
                println!("ERROR: Expected number, symbol, or string");
                return;
            }

            println!("ARGUMENT: {:?}", symbol);
            symbol.clear();
        }
    }

    return;

    let mut source_code = String::with_capacity(2048);
    source_file.read_to_string(&mut source_code).unwrap();

    let intrinsic_calls: Vec<&str> = source_code.split("@")
        .map(|s| s.trim())
        .collect();

    for intrinsic_call in intrinsic_calls
    {
        if intrinsic_call.trim().is_empty()
        {
            continue;
        }

        let mut elements = intrinsic_call.split("(").collect::<Vec<&str>>();

        let intrinsic_name = elements.remove(0);

        println!("CALL {}", intrinsic_name);

        let mut elements = elements.remove(0).split(",").collect::<Vec<&str>>();
        println!("{:?}", elements);

        for argument in elements
        {
            if argument == ")"
            {
                break;
            }

            println!("ARGUMENT: {}", argument);
        }

        output_file.write_fmt(
            format_args!(
                "    STORESP R6, [IP]\n    JMP {}\n\n",
                lookup_intrinsic(intrinsic_name)
            )
        ).unwrap();
    }

    output_file.write_fmt(format_args!("{}", POSTLUDE)).unwrap();
}


fn lookup_intrinsic<'a>(name: &'a str) -> &'static str
{
    match name
    {
        "clear-screen" => "CLEARSCREEN",
        "emit-string" => "EMITSTR",
        _ => unreachable!(),
    }
}

fn allocate_variable() { }
fn allocate_constant() { }
