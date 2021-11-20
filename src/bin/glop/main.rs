use std::io::{Read, Write};

const PRELUDE: &'static str = include_str!("../../../asm/prelude.inc");
const POSTLUDE: &'static str = include_str!("../../../asm/postlude.inc");

fn main()
{
    let filename = std::env::args().skip(1).next().unwrap();

    println!("{}", filename);

    let mut source_file = std::fs::File::open(filename).unwrap();
    let mut source_code = String::with_capacity(2048);
    source_file.read_to_string(&mut source_code).unwrap();

    println!("{}", source_code);

    let mut output_file = std::fs::File::create("a.asm").unwrap();
    output_file.write_fmt(format_args!("{}", PRELUDE)).unwrap();

    // TODO(pbz): Refine the parser at each stage
    println!("---------------------------------------------");

    let intrinsic_calls: Vec<&str> = source_code.split("@")
        .map(|s| s.trim())
        .collect();

    for intrinsic_call in intrinsic_calls
    {
        if intrinsic_call.trim().is_empty()
        {
            continue;
        }

        println!("{}", intrinsic_call);

        let mut elements = intrinsic_call.split("(").collect::<Vec<&str>>();

        let intrinsic_name = elements.remove(0);

        println!("CALL {}", intrinsic_name);

        output_file.write_fmt(
            format_args!(
                "    STORESP R6, [IP]\n    JMP {}\n\n",
                lookup_intrinsic(intrinsic_name)
            )
        ).unwrap();

        if *elements.last().unwrap() == "),"
        {
            continue;
        }

        println!("NUM-ARGS: {}", elements.len());
        println!("{:?}", elements);
    }

    output_file.write_fmt(format_args!("{}", POSTLUDE)).unwrap();
}


fn lookup_intrinsic<'a>(name: &'a str) -> &'static str
{
    match name
    {
        "clear-screen" => "CLEARSCREEN",
        _ => unreachable!(),
    }
}
