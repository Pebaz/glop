/*
1. Use the right tool for the job: load PE in Python
2. Pipe bytecode to STDIN
3. Composeable programs are beautiful. Need to get interface right instead of
Bash. What if polyglot programs exposed same interface?
4. Do it right the first time. Don't forget to document each program.
*/

use std::io::prelude::*;

// Make Instruction trait
// Make structs for each instruction that can parse themselves


// #[repr(u8)]
#[derive(Debug)]
enum OpCode
{
    ADD = 0x0C,
    AND = 0x14,
    ASHR = 0x19,
    BREAK = 0x00,
    CALL = 0x03,
    CMPeq = 0x05,
    CMPlte = 0x06,
    CMPgte = 0x07,
    CMPulte = 0x08,
    CMPugte = 0x09,
    CMPIeq = 0x2D,
    CMPIlte = 0x2E,
    CMPIgte = 0x2F,
    CMPIulte = 0x30,
    CMPIugte = 0x31,
    DIV = 0x10,
    DIVU = 0x11,
    EXTNDB = 0x1A,
    EXTNDD = 0x1C,
    EXTNDW = 0x1B,
    JMP = 0x01,
    JMP8 = 0x02,
    LOADSP = 0x29,
    MOD = 0x12,
    MODU = 0x13,
    MOVbw = 0x1D,
    MOVww = 0x1E,
    MOVdw = 0x1F,
    MOVqw = 0x20,
    MOVbd = 0x21,
    MOVwd = 0x22,
    MOVdd = 0x23,
    MOVqd = 0x24,
    MOVqq = 0x28,
    MOVI = 0x37,
    MOVIn = 0x38,
    MOVnw = 0x32,
    MOVnd = 0x33,
    MOVREL = 0x39,
    MOVsnw = 0x25,
    MOVsnd = 0x26,
    MUL = 0x0E,
    MULU = 0x0F,
    NEG = 0x0B,
    NOT = 0x0A,
    OR = 0x15,
    POP = 0x2C,
    POPn = 0x36,
    PUSH = 0x2B,
    PUSHn = 0x35,
    RET = 0x04,
    SHL = 0x17,
    SHR = 0x18,
    STORESP = 0x2A,
    SUB = 0x0D,
    XOR = 0x16
}

/// Needed since stringifying the OpCode is part of application functionality.
impl std::fmt::Display for OpCode
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
    }
}

impl OpCode
{
    // parse from [u8] -> Result

    // fn parse() -> Self
    // {
    //     match
    // }

    // fn parse;
    // fn print;

    fn disassemble<T: Iterator<Item=u8>>(bytes: &mut T) -> Option<()>
    {
        let byte0 = if let Some(byte) = bytes.next()
        {
            byte
        }
        else
        {
            return None;
        };
        let byte0_bits = bits(byte0);
        let op = bits_to_byte(&byte0_bits[0 ..= 5]);

        println!("OpCode: {}", op);

        match op
        {
            op if op == OpCode::ADD as u8 =>
            {
                println!("ADD");
            }

            op if op == OpCode::MOD as u8 =>
            {
                // println!("{:?}", OpCode::MOD);

                // let index_or_immediate_present = byte0_bits[7];
                // let is_64_bit = byte0_bits[6];

                // println!("  Index/Immediate: {}", index_or_immediate_present);
                // println!("  x64: {}", is_64_bit);

                // let byte1 = bytes.next().expect("Unexpected end of bytes");
                // let byte1_bits = bits(byte1);
                // let operand2_is_indirect = byte1_bits[7];
                // let operand2 = bits_to_byte(&byte1_bits[4 ..= 6]);
                // let operand1_is_indirect = byte1_bits[3];
                // let operand1 = bits_to_byte(&byte1_bits[0 ..= 2]);
                // TODO(pbz): Not done yet
            }

            // MOVsn{d} {@}R1 {Index32}, {@}R2 {Index32|Immed32}
            op if op == OpCode::MOVsnd as u8 =>
            {
                let operand1_index_present = byte0_bits[7];
                let operand2_index_present = byte0_bits[6];

                let byte1 = bytes.next().expect("Unexpected end of bytes");
                let byte1_bits = bits(byte1);
                let operand2_is_indirect = byte1_bits[7];
                let operand2 = bits_to_byte(&byte1_bits[4 ..= 6]);
                let operand1_is_indirect = byte1_bits[3];
                let operand1 = bits_to_byte(&byte1_bits[0 ..= 2]);

                let op1_x32_index_or_immediate =
                {
                    if operand1_index_present
                    {
                        let mut value = [0u8; 4];

                        value[0] = bytes.next().unwrap();
                        value[1] = bytes.next().unwrap();
                        value[2] = bytes.next().unwrap();
                        value[3] = bytes.next().unwrap();

                        Some(value)
                    }
                    else
                    {
                        None
                    }
                };

                let op2_x32_index_or_immediate =
                {
                    if operand2_index_present
                    {
                        let mut value = [0u8; 4];

                        value[0] = bytes.next().unwrap();
                        value[1] = bytes.next().unwrap();
                        value[2] = bytes.next().unwrap();
                        value[3] = bytes.next().unwrap();

                        Some(value)
                    }
                    else
                    {
                        None
                    }
                };

                if let Some(value) = op1_x32_index_or_immediate
                {
                    println!("op1_x32_index_or_immediate")
                }

                if let Some(value) = op2_x32_index_or_immediate
                {
                    println!("op2_x32_index_or_immediate")
                }

                println!("{}", OpCode::MOVsnd);
            }

            _ => ()
        }

        Some(())
    }
}

fn bits(byte: u8) -> [bool; 8]
{
    let mut bits = [false; 8];

    for i in 0 .. 8
    {
        if byte & 2u8.pow(i) > 0
        {
            bits[i as usize] = true;
        }
    }

    bits
}

fn bits_to_byte(bits: &[bool]) -> u8
{
    let mut byte = 0;

    for (i, bit) in bits.iter().enumerate()
    {
        if *bit
        {
            byte += 2u8.pow((bits.len() - i) as u32);
        }
    }
    byte
}

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

        loop
        {
            if OpCode::disassemble(&mut bytes).is_none()
            {
                break;
            }
        }

        /*
        for (i, byte) in bytes.enumerate()
        {
            instruction[i % 4] = byte;

            let bits = bits(byte);
            let instruction_type_bits = &bits[0 .. 5];
            let op = bits_to_byte(instruction_type_bits);

            if op > 0
            {
                println!("{:?}", op);
            }

            match op
            {
                op if op == OpCode::MOD as u8 =>
                {
                    println!("MOD: {:?}", OpCode::MOD);
                }

                _ => ()
            }

            // if i % 4 == 0 && i > 0
            // {
            //     println!("{:?}", instruction);
            //     instruction = [0; 4];
            // }
        }
        */

        // TODO(pbz): Bytes can be left over in the instruction. Process them.

        // loop
        // {
        //     let byte = bytes.next();
        //     if byte.is_none() { break; }

        //     println!("{:?}", byte.unwrap());
        // }
    }
}
