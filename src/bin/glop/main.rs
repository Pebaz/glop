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
fn expect_symbol(
    mut ptr: &mut Peekable<Chars>,
    out_string: &mut String
) -> bool
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

fn expect_string(
    mut ptr: &mut Peekable<Chars>,
    out_string: &mut String,
    delimiter: char
) -> bool
{
    if !expect_char(ptr, delimiter)
    {
        return false;
    }

    while let Some(ch) = ptr.next()
    {
        if ch == '"'
        {
            return true;
        }

        out_string.push(ch);
    }

    false
}

/// Tokens store their beginning index but not their end. The line/columns can
/// can be calculated when needed.
enum Token
{
    Intrinsic(u32),
    Symbol(u32),
    CallOpen(u32),
    CallClose(u32),
    String(u32),
    Comma(u32),
    Equals(u32),
    Number(u32),
}

/// Tokens store their beginning index as an offset from the previous token or
/// beginning of file. To calculate file/line numbers, get true offset by
/// summing all the previous token's offsets and then reiterating back through
/// the file to get to that offset to find the file/line.
// enum Token  // Total size: 16 bytes =)
// {
//     Intrinsic(u16),
//     Symbol(u16),
//     CallOpen(u16),
//     CallClose(u16),
//     String(u16),
//     Comma(u16),
//     Equals(u16),
//     Number(u16),
// }

fn main()
{
    let filename = std::env::args().skip(1).next().unwrap();
    let mut source_file = std::fs::File::open(filename).unwrap();
    let mut source_code = String::with_capacity(2048);
    source_file.read_to_string(&mut source_code).unwrap();

    let out_filename = std::env::args().skip(2).next().unwrap();
    let mut output_file = std::fs::File::create(out_filename).unwrap();
    output_file.write_fmt(format_args!("{}", PRELUDE)).unwrap();

    // TODO(pbz): Refine the parser at each stage
    println!("---------------------------------------------");


    let mut ptr = &mut source_code.chars().peekable();
    ptr.peek().unwrap();
    let mut buffer = String::with_capacity(64);
    // let mut arguments: [Argument; 16];

    let mut u64_constants = Vec::with_capacity(64);
    let mut str_constants = Vec::with_capacity(64);

    loop
    {
        // It's ok for a comma from the previous line to be here
        accept_char(ptr, ',');

        if let None = ptr.peek()
        {
            break;
        }

        // Must have opening compiler intrinsic call
        if !expect_char(ptr, '@')
        {
            // println!("ERROR: Expected call to compiler intrinsic");
            break;
        }

        // Must have intrinsic call name
        if !expect_symbol(ptr, &mut buffer)
        {
            return println!("ERROR: Expected symbol");
        }

        let intrinsic_call_name = buffer.clone();
        println!("INTRINSIC CALL: {}", buffer);
        buffer.clear();

        if !expect_char(ptr, '(')
        {
            return println!("ERROR: Expected opening parenthesis");
        }

        while !accept_char(ptr, ')')
        {
            // println!("ERROR: Expected closing parenthesis");
            // return;

            accept_char(ptr, ',');  // From last iteration

            if accept_char(ptr, '"')
            {
                while let Some(ch) = ptr.next()
                {
                    if ch == '"'
                    {
                        break;
                    }
                    buffer.push(ch);
                }

                // String constant
                str_constants.push(buffer.clone());
            }

            else if expect_symbol(ptr, &mut buffer)
            {
                // Integer constant or variable

                // Integer constant
                if buffer.chars().next().unwrap().is_numeric()
                {
                    u64_constants.push(buffer.clone());

                    output_file.write_fmt(
                        format_args!(
                            "    MOVREL R1, const_u64_{}\n    PUSH R1\n",
                            u64_constants.len() - 1
                        )
                    ).unwrap();
                }

                // Variable
                else
                {
                    return println!("ERROR: Expected number");
                }
            }

            else
            {
                return println!("ERROR: Expected number, symbol, or string");
            }

            println!("ARGUMENT: {:?}", buffer);
            buffer.clear();

        }

        output_file.write_fmt(
            format_args!(
                "    STORESP R6, [IP]\n    JMP {}\n\n",
                lookup_intrinsic(&intrinsic_call_name)
            )
        ).unwrap();
    }

    output_file.write_fmt(
        format_args!("{}", POSTLUDE)
    ).unwrap();
    output_file.write_fmt(
        format_args!(";; This is for initialized global variables\n")
    ).unwrap();
    output_file.write_fmt(
        format_args!("section 'DATA' data readable writeable\n")
    ).unwrap();

    for (i, u64_constant) in u64_constants.into_iter().enumerate()
    {
        output_file.write_fmt(format_args!("    const_u64_{}: dq {}\n", i, u64_constant)).unwrap();
    }
}


fn lookup_intrinsic<'a>(name: &'a str) -> &'static str
{
    match name
    {
        "clear-screen" => "CLEARSCREEN",
        "emit-string" => "EMITSTR",
        "draw-pixel" => "DRAWPIXEL",
        _ => unreachable!(),
    }
}

fn allocate_variable() { }
fn allocate_constant() { }
