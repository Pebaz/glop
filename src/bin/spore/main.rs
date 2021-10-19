/*
1. Use the right tool for the job: load PE in Python
2. Pipe bytecode to STDIN
3. Composeable programs are beautiful. Need to get interface right instead of
Bash. What if polyglot programs exposed same interface?
4. Do it right the first time. Don't forget to document each program.
*/

// TODO(pbz): Perhaps put all the "Behaviors and Restrictions" bullet points in
// TODO(pbz): comments by each instruction so that you can read exact behavior.

// TODO(pbz): Remove these in the future
#![allow(unused_variables)]
#![allow(dead_code)]

use std::io::prelude::*;
use std::convert::TryInto;
use colored::*;

const BLUE: (u8, u8, u8) = (98, 168, 209);











/*
INSTRUCTION [INDIRECT]OP1 ARGUMENT, ARGUMENT
*/




enum Operand
{
    GeneralPurpose
    {
        register_index: u8,
        indirect: bool,
    },

    Dedicated
    {
        register_index: u8,
        indirect: bool,
    },
}

impl Operand
{
    pub fn new_general_purpose(register_index: u8, indirect: bool) -> Self
    {
        assert!((0u8 ..= 7u8).contains(&register_index));

        Self::GeneralPurpose
        {
            register_index,
            indirect
        }
    }

    pub fn new_dedicated(register_index: u8, indirect: bool) -> Self
    {
        assert!((0u8 ..= 1u8).contains(&register_index));

        Self::Dedicated
        {
            register_index,
            indirect
        }
    }
}

impl std::fmt::Display for Operand
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Self::GeneralPurpose { register_index: index, indirect: at } =>
            {
                assert!((0u8 ..= 7u8).contains(&index));

                write!(
                    f,
                    "{}R{}",
                    if *at { "@" } else { "" },
                    index
                )
            }

            Self::Dedicated { register_index: index, indirect: at } =>
            {
                assert!((0u8 ..= 1u8).contains(&index));

                if *index == 0
                {
                    write!(f, "{}FLAGS", if *at { "@" } else { "" })
                }

                else
                {
                    write!(f, "{}IP", if *at { "@" } else { "" })
                }
            }
        }
    }
}

// ? Should this be genric over T for i32/u32 etc.
enum Argument
{
    Index16(u16),
    Index32(u32),
    Index64(u64),
    ImmediateU16(u16),
    ImmediateU32(u32),
    ImmediateU64(u64),
    ImmediateI16(i16),
    ImmediateI32(i32),
    ImmediateI64(i64),
}

impl std::fmt::Display for Argument
{
    // TODO(pbz): May want to remove + from here and only use - because it
    // TODO(pbz): doesn't make sense for BREAK CODE
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Self::Index16(index) =>
            {
                let natural_index = NaturalIndex::from_u16(*index);

                write!(f, "{}", natural_index)
            }

            Self::Index32(index) =>
            {
                let natural_index = NaturalIndex::from_u32(*index);

                write!(f, "{}", natural_index)
            }

            Self::Index64(index) =>
            {
                let natural_index = NaturalIndex::from_u64(*index);

                write!(f, "{}", natural_index)
            }

            Self::ImmediateU16(immediate) => write!(f, "{}", immediate),

            Self::ImmediateU32(immediate) => write!(f, "{}", immediate),

            Self::ImmediateU64(immediate) => write!(f, "{}", immediate),

            Self::ImmediateI16(immediate) => write!(f, "{}", immediate),

            Self::ImmediateI32(immediate) => write!(f, "{}", immediate),

            Self::ImmediateI64(immediate) => write!(f, "{}", immediate),
        }
    }
}


/*
8. INSTRUCTION OP1 ARGUMENT, OP2 ARGUMENT
    * GOTTA MATCH ON
        MOV? (such as MOVqq. Technically they are all the same instruction)
        MOV?, MOVn, MOVsn (these have overlap with MOV, but MOVqq is tricky)

7. ✅ INSTRUCTION OP1, OP2 ARGUMENT (16 bit optional index/immediate)
    ADD
    AND
    ASHR
    CMP
    DIV
    DIVU
    EXTENDB
    EXTENDD
    EXTENDW
    MOD
    MODU
    MUL
    MULU
    NEG
    NOT
    OR
    SHL
    SHR
    SUB
    XOR

6. INSTRUCTION OP1 ARGUMENT, ARGUMENT
    * GOTTA MATCH ON
        CMPI
        MOVI
        MOVIn
        MOVREL <- Check these to make sure parsing is the same

5. ✅ INSTRUCTION OP1, OP2
    STORESP
    LOADSP

4. INSTRUCTION OP1 ARGUMENT
    * GOTTA MATCH ON
        CALL32
        JMP32
        PUSH
        PUSHn
        POP
        POPn

    * THESE ARE TECHNICALLY HERE ALSO BUT THEY CAN'T BE MATCHED UPON ❌
        JMP64
        CALL64


3. INSTRUCTION OP1 ARGUMENT, ARGUMENT
    CMPI

2. ✅ INSTRUCTION ARGUMENT
    * GOTTA MATCH ON
        JMP8
        BREAK

    * THESE ARE TECHNICALLY HERE ALSO BUT THEY CAN'T BE MATCHED UPON ❌
        JMP64
        CALL64

1. ✅ INSTRUCTION
    RET
*/

fn parse_instruction1<T: Iterator<Item=u8>>(
    bytes: &mut T,
    byte0_bits: [bool; 8],
    op_value: u8,
    op: OpCode,
) -> Option<()>
{
    disassemble_instruction(
        format!("{}", op).truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        None,
        None,
        None,
        None,
        None
    );

    Some(())
}

fn parse_instruction2<T: Iterator<Item=u8>>(
    bytes: &mut T,
    byte0_bits: [bool; 8],
    op_value: u8,
    op: OpCode,
) -> Option<()>
{
    let mut name = format!("{}", op);

    let byte1 = bytes.next().expect("Unexpected end of bytes");
    let byte1_bits = bits_rev(byte1);

    let arg1 = match op
    {
        OpCode::BREAK =>
        {
            if byte1 == 0
            {
                panic!(
                    "Runaway program break (found 2 zeros in a row, BREAK 0)"
                );
            }

            Argument::ImmediateU16(byte1 as u16)
        }

        OpCode::JMP8 =>
        {
            let conditional = byte0_bits[7];

            if conditional
            {
                let condition_bit_set = byte0_bits[6];

                name += if condition_bit_set
                {
                    "cs"
                }
                else
                {
                    "cc"
                };
            }

            // TODO(pbz): Comments might not be the best idea.
            // TODO(pbz): `JMP` means unconditional jump. `JMPcs` means conditional
            // else
            // {
            //     comment = Some(String::from("Unconditional"));
            // }

            Argument::ImmediateI16((byte1 as i8) as i16)
        }

        _ => unreachable!(),
    };

    // let (op1, op2) = match op
    // {
    //     OpCode::STORESP => (
    //         Operand::new_general_purpose(operand1_value, false),
    //         Operand::new_dedicated(operand2_value, false)
    //     ),

    //     OpCode::LOADSP => (
    //         Operand::new_dedicated(operand1_value, false),
    //         Operand::new_general_purpose(operand2_value, false)
    //     ),

    //     _ => unreachable!(),
    // };

    disassemble_instruction(
        name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        None,
        Some(arg1),
        None,
        None,
        None
    );

    Some(())
}

fn parse_instruction4<T: Iterator<Item=u8>>(
    bytes: &mut T,
    byte0_bits: [bool; 8],
    op_value: u8,
    op: OpCode,
) -> Option<()>
{
    let mut name = format!("{}", op);
    let mut postfix = String::with_capacity(5);
    let immediate_data_present = byte0_bits[7];
    let is_64_bit = byte0_bits[6];  // Not used by PUSHn & POPn

    let byte1 = bytes.next().expect("Unexpected end of bytes");
    let byte1_bits = bits_rev(byte1);

    match op
    {
        OpCode::CALL
        | OpCode::JMP
        | OpCode::PUSH
        | OpCode::POP =>
        {
            postfix += if is_64_bit { "64" } else { "32" };
        }

        _ => (),
    }

    // TODO(pbz): Have postfixes colored differently? =)
    let (op1, arg1, op2, arg2, comment) = match op
    {
        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        // TODO(pbz): THIS IS VERY IMPORTANT. CHECK THIS VERY CAREFULLY
        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        OpCode::CALL =>
        {
            let is_native_call = byte1_bits[5];
            postfix += if is_native_call { "EX" } else { "" };

            let is_relative_address = byte1_bits[4];
            let operand1_is_indirect = byte1_bits[3];
            let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
            let op1 = if !is_64_bit
            {
                Some(
                    Operand::new_general_purpose(
                        operand1_value,
                        operand1_is_indirect
                    )
                )
            }
            else
            {
                None
            };

            let arg1 = if is_64_bit
            {
                postfix += "a";  // CALL64 is always an absolute address

                let mut value = [0u8; 8];

                for i in 0 .. value.len()
                {
                    value[i] = bytes.next().unwrap();
                }

                // TODO(pbz): For absolute, display in hex
                Some(Argument::ImmediateI64(i64::from_le_bytes(value)))
            }
            else
            {
                postfix += if is_relative_address { "" } else { "a" };

                let arg = if immediate_data_present
                {
                    let mut value = [0u8; 4];

                    for i in 0 .. value.len()
                    {
                        value[i] = bytes.next().unwrap();
                    }

                    if operand1_is_indirect
                    {
                        Some(Argument::Index32(u32::from_le_bytes(value)))
                    }
                    else
                    {
                        Some(Argument::ImmediateI32(i32::from_le_bytes(value)))
                    }
                }
                else
                {
                    None
                };

                arg
            };

            (op1, arg1, None, None, None)
        }

        OpCode::JMP =>
        {
            let conditional_jump = byte1_bits[7];
            let jump_if_condition_bit_set = byte1_bits[6];

            if conditional_jump
            {
                postfix += if jump_if_condition_bit_set { "cs" } else { "cc" };
            }

            let relative_address = byte1_bits[4];
            let operand1_is_indirect = byte1_bits[3];
            let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
            let op1 = if !is_64_bit
            {
                Some(
                    Operand::new_general_purpose(
                        operand1_value,
                        operand1_is_indirect
                    )
                )
            }
            else
            {
                None
            };

            let arg1 = if is_64_bit
            {
                let mut value = [0u8; 8];

                for i in 0 .. value.len()
                {
                    value[i] = bytes.next().unwrap();
                }

                // TODO(pbz): Check if absolute, then display in hex
                Some(Argument::ImmediateI64(i64::from_le_bytes(value)))
            }
            else
            {
                let arg = if immediate_data_present
                {
                    let mut value = [0u8; 4];

                    for i in 0 .. value.len()
                    {
                        value[i] = bytes.next().unwrap();
                    }

                    if operand1_is_indirect
                    {
                        Some(Argument::Index32(u32::from_le_bytes(value)))
                    }
                    else
                    {
                        Some(Argument::ImmediateI32(i32::from_le_bytes(value)))
                    }
                }
                else
                {
                    None
                };

                arg
            };

            let comment = if relative_address
            {
                Some(String::from("Relative Address"))
            }
            else
            {
                Some(String::from("Absolute Address"))
            };

            (op1, arg1, None, None, comment)
        }

        OpCode::PUSH
        | OpCode::POP
        | OpCode::PUSHn
        | OpCode::POPn =>
        {
            let operand1_is_indirect = byte1_bits[3];
            let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
            let arg1 = if immediate_data_present
            {
                let mut value = [0u8; 2];

                value[0] = bytes.next().unwrap();
                value[1] = bytes.next().unwrap();

                let arg = if operand1_is_indirect
                {
                    Argument::Index16(u16::from_le_bytes(value))
                }
                else
                {
                    Argument::ImmediateI16(i16::from_le_bytes(value))
                };

                Some(arg)
            }
            else
            {
                None
            };

            (
                Some(
                    Operand::new_general_purpose(
                        operand1_value,
                        operand1_is_indirect
                    )
                ),
                arg1,
                None,
                None,
                None
            )
        }

        _ => unreachable!(),
    };

    name += &postfix;

    disassemble_instruction(
        format!("{}", name).truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        op1,
        arg1,
        op2,
        arg2,
        comment
    );

    Some(())
}

fn parse_instruction5<T: Iterator<Item=u8>>(
    bytes: &mut T,
    byte0_bits: [bool; 8],
    op_value: u8,
    op: OpCode,
) -> Option<()>
{
    let name = format!("{}", op);

    let byte1 = bytes.next().expect("Unexpected end of bytes");
    let byte1_bits = bits_rev(byte1);
    let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
    let operand2_value = bits_to_byte_rev(&byte1_bits[4 ..= 6]);

    let (op1, op2) = match op
    {
        OpCode::STORESP => (
            Operand::new_general_purpose(operand1_value, false),
            Operand::new_dedicated(operand2_value, false)
        ),

        OpCode::LOADSP => (
            Operand::new_dedicated(operand1_value, false),
            Operand::new_general_purpose(operand2_value, false)
        ),

        _ => unreachable!(),
    };

    disassemble_instruction(
        name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        Some(op1),
        None,
        Some(op2),
        None,
        None
    );

    Some(())
}

fn parse_instruction7<T: Iterator<Item=u8>>(
    bytes: &mut T,
    byte0_bits: [bool; 8],
    op_value: u8,
    op: OpCode,
) -> Option<()>
{
    let mut name = format!("{}", op);
    let immediate_data_present = byte0_bits[7];

    // TODO(pbz): Have postfixes colored differently? =)
    name += if byte0_bits[6]
    {
        "64"
    }
    else
    {
        "32"
    };

    let byte1 = bytes.next().expect("Unexpected end of bytes");
    let byte1_bits = bits_rev(byte1);
    let operand1_is_indirect = byte1_bits[3];
    let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);
    let operand2_is_indirect = byte1_bits[7];
    let operand2_value = bits_to_byte_rev(&byte1_bits[4 ..= 6]);

    let op1_x16_index_or_immediate =
    {
        if immediate_data_present
        {
            let mut value = [0u8; 2];

            value[0] = bytes.next().unwrap();
            value[1] = bytes.next().unwrap();

            let arg = if operand2_is_indirect
            {
                Argument::Index16(u16::from_le_bytes(value))
            }
            else
            {
                Argument::ImmediateI16(i16::from_le_bytes(value))
            };

            Some(arg)
        }
        else
        {
            None
        }
    };

    disassemble_instruction(
        name.truecolor(BLUE.0, BLUE.1, BLUE.2).to_string(),
        Some(
            Operand::new_general_purpose(operand1_value, operand1_is_indirect)
        ),
        None,
        Some(
            Operand::new_general_purpose(operand2_value, operand2_is_indirect)
        ),
        op1_x16_index_or_immediate,
        None
    );

    Some(())
}






















// TODO(pbz): Invest in some left/right justification
fn disassemble_instruction(
    instruction: String,  // Must concatenate postfixes manually
    operand1: Option<Operand>,
    argument1: Option<Argument>,
    operand2: Option<Operand>,
    argument2: Option<Argument>,
    comment: Option<String>,
)
{
    print!("{}", instruction);

    if let Some(op1) = operand1
    {
        print!(" {}", op1);
    }

    if let Some(arg1) = argument1
    {
        match arg1
        {
            Argument::Index16(index) => print!("{}", arg1),
            Argument::Index32(index) => print!("{}", arg1),
            Argument::Index64(index) => print!("{}", arg1),
            _ => print!(" {}", arg1),
        }
    }

    if operand2.is_some() || argument2.is_some()
    {
        print!(",");
    }

    if let Some(op2) = operand2
    {
        print!(" {}", op2);
    }

    if let Some(arg2) = argument2
    {
        match arg2
        {
            Argument::Index16(index) => print!("{}", arg2),
            Argument::Index32(index) => print!("{}", arg2),
            Argument::Index64(index) => print!("{}", arg2),
            _ => print!(" {}", arg2),
        }
    }

    // TODO(pbz): Adhere to a column so they line up
    if let Some(line_comment) = comment
    {
        print!("  ;; {}", line_comment);
    }

    println!("");
}











/// As noted in UEFI.22.3, these registers are only to be interpreted as
/// "general purpose" when using normal instructions. Specialized instructions
/// such as CMP can reference these same indices, but they refer to registers
/// like FLAGS, IP, and some reserved registers.
#[derive(Debug)]
enum Register
{
    R0 = 0,
    R1 = 1,
    R2 = 2,  // * Not valid from here down for Dedicated VM Register Indices
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7
}

/// Needed since stringifying the OpCode is part of application functionality.
impl std::fmt::Display for Register
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
    }
}

impl Register
{
    fn from_u8(value: u8) -> Self
    {
        match value
        {
            0 => Self::R0,
            1 => Self::R1,
            2 => Self::R2,
            3 => Self::R3,
            4 => Self::R4,
            5 => Self::R5,
            6 => Self::R6,
            7 => Self::R7,
            _ => unreachable!(),
        }
    }
}

struct NaturalIndex
{
    value: u64,
    sign: i8,
    constant: u64,
    natural: u64,
    offset: i64,
}

const SIZE_OF_VOID_PTR: u16 = 8;
const HEADER_SIZE: usize = 4;

impl NaturalIndex
{
    /// It is critical that the right method be selected per index size.
    /// Do not use `from_u64()` for a 16 bit value.
    fn from_u16(value: u16) -> Self
    {
        const ENCODING_SIZE: u16 = 2;

        let bits = bits_u16(value);
        let sign = if bits[0] { -1i64 } else { 1i64 };
        let width_base = bits_to_byte_u16(&bits[1 .. 4]);
        let actual_width = width_base * ENCODING_SIZE;
        let natural = bits_to_byte_u16(&bits[bits.len() - actual_width as usize ..]);
        let constant = bits_to_byte_u16(&bits[HEADER_SIZE .. bits.len() - actual_width as usize]);
        let offset = sign * (constant + natural * SIZE_OF_VOID_PTR) as i64;

        Self
        {
            value: value as u64,
            sign: sign as i8,
            constant: constant as u64,
            natural: natural as u64,
            offset: offset as i64
        }
    }

    /// It is critical that the right method be selected per index size.
    /// Do not use `from_u64()` for a 16 bit value.
    fn from_u32(value: u32) -> Self
    {
        const ENCODING_SIZE: u32 = 4;

        let bits = bits_u32(value);
        let sign = if bits[0] { -1i64 } else { 1i64 };
        let width_base = bits_to_byte_u32(&bits[1 .. 4]);
        let actual_width = width_base * ENCODING_SIZE;
        let natural = bits_to_byte_u32(&bits[bits.len() - actual_width as usize ..]);
        let constant = bits_to_byte_u32(&bits[HEADER_SIZE .. bits.len() - actual_width as usize]);
        let offset = sign * (constant + natural * SIZE_OF_VOID_PTR as u32) as i64;

        Self
        {
            value: value as u64,
            sign: sign as i8,
            constant: constant as u64,
            natural: natural as u64,
            offset: offset as i64
        }
    }

    /// It is critical that the right method be selected per index size.
    /// Do not use `from_u64()` for a 16 bit value.
    fn from_u64(value: u64) -> Self
    {
        const ENCODING_SIZE: u64 = 8;

        let bits = bits_u64(value);
        let sign = if bits[0] { -1i64 } else { 1i64 };
        let width_base = bits_to_byte_u64(&bits[1 .. 4]);
        let actual_width = width_base * ENCODING_SIZE;
        let natural = bits_to_byte_u64(&bits[bits.len() - actual_width as usize ..]);
        let constant = bits_to_byte_u64(&bits[HEADER_SIZE .. bits.len() - actual_width as usize]);
        let offset = sign * (constant + natural * SIZE_OF_VOID_PTR as u64) as i64;

        Self
        {
            value: value,
            sign: sign as i8,
            constant: constant,
            natural: natural,
            offset: offset as i64
        }
    }
}

impl std::fmt::Display for NaturalIndex
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(
            f,
            "({}{}, {}{})",
            if self.sign < 0 { "-" } else { "+" },
            self.natural,
            if self.sign < 0 { "-" } else { "+" },
            self.constant
        )
    }
}

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

impl std::convert::TryFrom<u8> for OpCode
{
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error>
    {
        match v
        {
            x if x == Self::ADD as u8 => Ok(Self::ADD),
            x if x == Self::AND as u8 => Ok(Self::AND),
            x if x == Self::ASHR as u8 => Ok(Self::ASHR),
            x if x == Self::BREAK as u8 => Ok(Self::BREAK),
            x if x == Self::CALL as u8 => Ok(Self::CALL),
            x if x == Self::CMPeq as u8 => Ok(Self::CMPeq),
            x if x == Self::CMPlte as u8 => Ok(Self::CMPlte),
            x if x == Self::CMPgte as u8 => Ok(Self::CMPgte),
            x if x == Self::CMPulte as u8 => Ok(Self::CMPulte),
            x if x == Self::CMPugte as u8 => Ok(Self::CMPugte),
            x if x == Self::CMPIeq as u8 => Ok(Self::CMPIeq),
            x if x == Self::CMPIlte as u8 => Ok(Self::CMPIlte),
            x if x == Self::CMPIgte as u8 => Ok(Self::CMPIgte),
            x if x == Self::CMPIulte as u8 => Ok(Self::CMPIulte),
            x if x == Self::CMPIugte as u8 => Ok(Self::CMPIugte),
            x if x == Self::DIV as u8 => Ok(Self::DIV),
            x if x == Self::DIVU as u8 => Ok(Self::DIVU),
            x if x == Self::EXTNDB as u8 => Ok(Self::EXTNDB),
            x if x == Self::EXTNDD as u8 => Ok(Self::EXTNDD),
            x if x == Self::EXTNDW as u8 => Ok(Self::EXTNDW),
            x if x == Self::JMP as u8 => Ok(Self::JMP),
            x if x == Self::JMP8 as u8 => Ok(Self::JMP8),
            x if x == Self::LOADSP as u8 => Ok(Self::LOADSP),
            x if x == Self::MOD as u8 => Ok(Self::MOD),
            x if x == Self::MODU as u8 => Ok(Self::MODU),
            x if x == Self::MOVbw as u8 => Ok(Self::MOVbw),
            x if x == Self::MOVww as u8 => Ok(Self::MOVww),
            x if x == Self::MOVdw as u8 => Ok(Self::MOVdw),
            x if x == Self::MOVqw as u8 => Ok(Self::MOVqw),
            x if x == Self::MOVbd as u8 => Ok(Self::MOVbd),
            x if x == Self::MOVwd as u8 => Ok(Self::MOVwd),
            x if x == Self::MOVdd as u8 => Ok(Self::MOVdd),
            x if x == Self::MOVqd as u8 => Ok(Self::MOVqd),
            x if x == Self::MOVqq as u8 => Ok(Self::MOVqq),
            x if x == Self::MOVI as u8 => Ok(Self::MOVI),
            x if x == Self::MOVIn as u8 => Ok(Self::MOVIn),
            x if x == Self::MOVnw as u8 => Ok(Self::MOVnw),
            x if x == Self::MOVnd as u8 => Ok(Self::MOVnd),
            x if x == Self::MOVREL as u8 => Ok(Self::MOVREL),
            x if x == Self::MOVsnw as u8 => Ok(Self::MOVsnw),
            x if x == Self::MOVsnd as u8 => Ok(Self::MOVsnd),
            x if x == Self::MUL as u8 => Ok(Self::MUL),
            x if x == Self::MULU as u8 => Ok(Self::MULU),
            x if x == Self::NEG as u8 => Ok(Self::NEG),
            x if x == Self::NOT as u8 => Ok(Self::NOT),
            x if x == Self::OR as u8 => Ok(Self::OR),
            x if x == Self::POP as u8 => Ok(Self::POP),
            x if x == Self::POPn as u8 => Ok(Self::POPn),
            x if x == Self::PUSH as u8 => Ok(Self::PUSH),
            x if x == Self::PUSHn as u8 => Ok(Self::PUSHn),
            x if x == Self::RET as u8 => Ok(Self::RET),
            x if x == Self::SHL as u8 => Ok(Self::SHL),
            x if x == Self::SHR as u8 => Ok(Self::SHR),
            x if x == Self::STORESP as u8 => Ok(Self::STORESP),
            x if x == Self::SUB as u8 => Ok(Self::SUB),
            x if x == Self::XOR as u8 => Ok(Self::XOR),
            _ => Err(()),
        }
    }
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

    /// Bytes are read from left to right. Bits are read from right to left.
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

        // println!("BYTE: {}", byte0);

        // * Using reverse number parsing to make indexing the individual bits
        // * easier since the UEFI spec specifies them in reverse.

        let byte0_bits = bits_rev(byte0);
        let op_value = bits_to_byte_rev(&byte0_bits[0 ..= 5]);
        let op: OpCode = op_value.try_into().expect(
            format!("Invalid OpCode: {}", op_value).as_str()
        );

        // TODO(pbz): Reorder these by 1-7
        match op
        {
            OpCode::ADD
            | OpCode::AND
            | OpCode::ASHR
            | OpCode::CMPeq
            | OpCode::CMPlte
            | OpCode::CMPgte
            | OpCode::CMPulte
            | OpCode::CMPugte
            | OpCode::CMPIeq
            | OpCode::CMPIlte
            | OpCode::CMPIgte
            | OpCode::CMPIulte
            | OpCode::CMPIugte
            | OpCode::DIV
            | OpCode::DIVU
            | OpCode::EXTNDB
            | OpCode::EXTNDD
            | OpCode::EXTNDW
            | OpCode::MOD
            | OpCode::MODU
            | OpCode::MUL
            | OpCode::MULU
            | OpCode::NEG
            | OpCode::NOT
            | OpCode::OR
            | OpCode::SHL
            | OpCode::SHR
            | OpCode::SUB
            | OpCode::XOR =>
            {
                parse_instruction7(bytes, byte0_bits, op_value, op)
            }

            OpCode::CALL
            | OpCode::JMP
            | OpCode::PUSH
            | OpCode::PUSHn
            | OpCode::POP
            | OpCode::POPn
            | OpCode::JMP
            | OpCode::CALL =>
            {
                parse_instruction4(bytes, byte0_bits, op_value, op)
            }

            OpCode::LOADSP
            | OpCode::STORESP =>
            {
                parse_instruction5(bytes, byte0_bits, op_value, op)
            }

            OpCode::JMP8
            | OpCode::BREAK =>
            {
                parse_instruction2(bytes, byte0_bits, op_value, op)
            }

            OpCode::RET => parse_instruction1(bytes, byte0_bits, op_value, op),

            _ =>  // TODO(pbz): Remove this once all instructions are covered
            {
                println!("OpCode: {}", op);

                Some(())
            }
        }

        // println!("{:?} bytes", &byte0_bits[0 ..= 5]);
        // println!("OpCode: {}", op);
        // return None;

        /*
        match op
        {
            OpCode::MOVnw =>
            {
                let operand1_index_present = byte0_bits[7];
                let operand2_index_present = byte0_bits[6];

                let byte1 = bytes.next().expect("Unexpected end of bytes");
                let byte1_bits = bits_rev(byte1);
                let operand2_is_indirect = byte1_bits[7];
                let operand2_value = bits_to_byte_rev(&byte1_bits[4 ..= 6]);
                let operand1_is_indirect = byte1_bits[3];
                let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);

                let op1_x16_index_or_immediate =
                {
                    if operand1_index_present
                    {
                        let mut value = [0u8; 2];

                        value[0] = bytes.next().unwrap();
                        value[1] = bytes.next().unwrap();

                        Some(value)
                    }
                    else
                    {
                        None
                    }
                };

                let op2_x16_index_or_immediate =
                {
                    if operand2_index_present
                    {
                        let mut value = [0u8; 2];

                        value[0] = bytes.next().unwrap();
                        value[1] = bytes.next().unwrap();

                        Some(value)
                    }
                    else
                    {
                        None
                    }
                };

                print!("    {} ", op);

                // Operand 1
                if operand1_is_indirect
                {
                    print!("@");
                }

                let operand1 = Register::from_u8(operand1_value);

                print!("{}", operand1);

                if let Some(value) = op1_x16_index_or_immediate
                {
                    print!("({})", u16::from_le_bytes(value));
                }

                print!(", ");

                // Operand 2
                if operand2_is_indirect
                {
                    print!("@");
                }

                let operand2 = Register::from_u8(operand2_value);

                print!("{}", operand2);

                if let Some(value) = op2_x16_index_or_immediate
                {
                    let index = u16::from_le_bytes(value);
                    let natural_index = NaturalIndex::from_u16(index);
                    print!("{}", natural_index);
                }

                println!("");
            }

            OpCode::MOVREL =>
            {
                let size_of_immediate_data = bits_to_byte_rev(
                    &byte0_bits[6 ..= 7]
                );

                let byte1 = bytes.next().expect("Unexpected end of bytes");
                let byte1_bits = bits_rev(byte1);
                let operand1_index_present = byte1_bits[6];
                let operand1_is_indirect = byte1_bits[3];
                let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);

                let op1_x16_index_or_immediate =
                {
                    if operand1_index_present
                    {
                        let mut value = [0u8; 2];

                        value[0] = bytes.next().unwrap();
                        value[1] = bytes.next().unwrap();

                        Some(value)
                    }
                    else
                    {
                        None
                    }
                };

                // This is a signed integer of size 16, 32, or 64 bits
                let immediate_offset =
                {
                    // Store enough for 64 bits and then just match on output
                    let mut value = [0u8; 8];

                    match size_of_immediate_data
                    {
                        1 =>
                        {
                            value[0] = bytes.next().unwrap();
                            value[1] = bytes.next().unwrap();
                        }

                        2 =>
                        {
                            value[0] = bytes.next().unwrap();
                            value[1] = bytes.next().unwrap();
                            value[2] = bytes.next().unwrap();
                            value[3] = bytes.next().unwrap();
                        }

                        3 =>
                        {
                            value[0] = bytes.next().unwrap();
                            value[1] = bytes.next().unwrap();
                            value[2] = bytes.next().unwrap();
                            value[3] = bytes.next().unwrap();
                            value[4] = bytes.next().unwrap();
                            value[5] = bytes.next().unwrap();
                        }
                        _ => unreachable!()
                    }

                    value
                };

                print!("    {} ", op);

                // Operand 1
                if operand1_is_indirect
                {
                    print!("@");
                }

                let operand1 = Register::from_u8(operand1_value);

                print!("{}", operand1);

                if let Some(value) = op1_x16_index_or_immediate
                {
                    print!("({})", u16::from_le_bytes(value));
                }

                print!(", ");

                // Operand 2
                match size_of_immediate_data
                {
                    1 =>
                    {
                        let mut value = [0u8; 2];
                        for i in 0 .. value.len()
                        {
                            value[i] = immediate_offset[i];
                        }
                        let offset = i16::from_le_bytes(value);
                        print!("{}", if offset < 0 { "-" } else { "+" });
                        print!("{}", offset);
                    }

                    2 =>
                    {
                        let mut value = [0u8; 4];
                        for i in 0 .. value.len()
                        {
                            value[i] = immediate_offset[i];
                        }
                        let offset = i32::from_le_bytes(value);
                        print!("{}", if offset < 0 { "-" } else { "+" });
                        print!("{}", offset);
                    }

                    3 =>
                    {
                        let offset = i64::from_le_bytes(immediate_offset);
                        print!("{}", if offset < 0 { "-" } else { "+" });
                        print!("{}", offset);
                    }

                    _ => unreachable!()
                }

                println!("");
            }

            OpCode::PUSHn =>
            {
                let operand1_index_present = byte0_bits[7];

                let byte1 = bytes.next().expect("Unexpected end of bytes");
                let byte1_bits = bits_rev(byte1);
                let operand1_is_indirect = byte1_bits[3];
                let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);

                let op1_x16_index_or_immediate =
                {
                    if operand1_index_present
                    {
                        let mut value = [0u8; 2];

                        value[0] = bytes.next().unwrap();
                        value[1] = bytes.next().unwrap();

                        Some(value)
                    }
                    else
                    {
                        None
                    }
                };

                print!("    {} ", op);

                // Operand 1
                if operand1_is_indirect
                {
                    print!("@");
                }

                let operand1 = Register::from_u8(operand1_value);

                print!("{}", operand1);

                if let Some(value) = op1_x16_index_or_immediate
                {
                    if operand1_is_indirect
                    {
                        let offset = i16::from_le_bytes(value);
                        print!("{}", if offset < 0 { '-' } else { '+' });
                        print!("({})", offset);
                    }
                    else
                    {
                        let index = u16::from_le_bytes(value);
                        let natural_index = NaturalIndex::from_u16(index);
                        print!("{}", natural_index);
                    }
                }

                println!("");
            }

            OpCode::CALL =>
            {
                let operand1_index_present = byte0_bits[7];
                let operand1_index_is_x64 = byte0_bits[6];

                let byte1 = bytes.next().expect("Unexpected end of bytes");
                let byte1_bits = bits_rev(byte1);
                let is_native_call = byte1_bits[5];
                let is_relative_address = byte1_bits[4];
                let operand1_is_indirect = byte1_bits[3];
                let operand1_value = bits_to_byte_rev(&byte1_bits[0 ..= 2]);

                let op1_x32_index_or_immediate =
                {
                    if operand1_index_present
                    {
                        let mut value = [0u8; 8];

                        if operand1_index_is_x64
                        {
                            for i in 0 .. value.len()
                            {
                                value[i] = bytes.next().unwrap();
                            }
                        }
                        else
                        {
                            value[0] = bytes.next().unwrap();
                            value[1] = bytes.next().unwrap();
                        }

                        Some(value)
                    }
                    else
                    {
                        None
                    }
                };

                print!("    CALL");
                print!("{}", if operand1_index_is_x64 { "64" } else { "32" });

                if is_native_call
                {
                    print!("EX");
                }

                if !is_relative_address
                {
                    print!("a");
                }

                print!(" ");

                // !!!!!!!!!!!!!!!!!!! There are only a finite number of ways to print out instructions
                // !!!!!!!!!!!!!!!!!!! Perhaps there are only a finite number of ways to parse them also?
                // See section 22.7 because it shows you the variants!
                // !!!!!!!!!!!!!!!!!!!
                // !!!!!!!!!!!!!!!!!!!
                // !!!!!!!!!!!!!!!!!!!
                // !!!!!!!!!!!!!!!!!!!
                // !!!!!!!!!!!!!!!!!!!
                // !!!!!!!!!!!!!!!!!!! Please take the time to hand-translate
                // !!!!!!!!!!!!!!!!!!! the bytecode into assembly
                // !!!!!!!!!!!!!!!!!!! It will be worth it
                // !!!!!!!!!!!!!!!!!!!
                // !!!!!!!!!!!!!!!!!!!
                // !!!!!!!!!!!!!!!!!!!


                // Operand 1
                if operand1_is_indirect
                {
                    print!("@");
                }

                let operand1 = Register::from_u8(operand1_value);

                print!("{}", operand1);

                // TODO(pbz): This is the easy one. Always treat it as immed
                if operand1_index_is_x64
                {
                    let value = op1_x32_index_or_immediate.expect(
                        "Expected 64-bit immediate data"
                    );

                    let offset = i64::from_le_bytes(value);
                    print!("{}", if offset < 0 { '-' } else { '+' });
                    print!("offset");
                }
                else
                {
                    if let Some(immediate_offset) = op1_x32_index_or_immediate
                    {
                        let mut value = [0u8; 4];
                        for i in 0 .. value.len()
                        {
                            value[i] = immediate_offset[i];
                        }

                        if operand1_is_indirect
                        {
                            let index = u32::from_le_bytes(value);
                            let natural_index = NaturalIndex::from_u32(index);
                            print!("{}", natural_index);
                        }
                        else
                        {
                            let offset = i32::from_le_bytes(value);
                            print!("{}", if offset < 0 { '-' } else { '+' });
                            print!("({})", offset);
                        }
                    }
                }


                // if let Some(value) = op1_index_or_immediate
                // {
                //     if operand1_index_is_x64
                //     {
                //         let offset = i64::from_le_bytes(value);
                //         print!("{}", if offset < 0 { '-' } else { '+' });
                //         print!("({})", offset);
                //     }

                //     if operand1_is_indirect
                //     {
                //         let offset = i16::from_le_bytes(value);
                //         print!("{}", if offset < 0 { '-' } else { '+' });
                //         print!("({})", offset);
                //     }
                //     else
                //     {
                //         let index = u16::from_le_bytes(value);
                //         let natural_index = NaturalIndex::from_u16(index);
                //         print!("{}", natural_index);
                //     }
                // }

                println!("");
            }

            OpCode::BREAK =>
            {
                return None;  // TODO(pbz): This is just temporary
            }

            _ =>  // TODO(pbz): Remove this once all instructions are covered
            {
                println!("OpCode: {}", op);
            }
        }
        */

        // Some(())
    }
}

// fn bits(byte: u8) -> [bool; 8]
// {
//     let mut bits = [false; 8];

//     for i in 0 .. 8
//     {
//         if byte & 2u8.pow(i) > 0
//         {
//             bits[(bits.len() - 1) - i as usize] = true;
//         }
//     }

//     bits
// }


fn bits_u8(byte: u8) -> [bool; 8]
{
    let mut bits = [false; 8];

    for i in 0 .. 8
    {
        if byte & 2u8.pow(i) > 0
        {
            bits[(bits.len() - 1) - i as usize] = true;
        }
    }

    bits
}

fn bits_to_byte_u8(bits: &[bool]) -> u8
{
    let mut byte = 0;

    for (i, bit) in bits.iter().rev().enumerate()
    {
        if *bit
        {
            // byte += 2u8.pow((bits.len() - 1 - i) as u32);
            byte += 2u8.pow((i) as u32);
        }
    }
    byte
}

fn bits_u16(byte: u16) -> [bool; 16]
{
    let mut bits = [false; 16];

    for i in 0 .. 16
    {
        if byte & 2u16.pow(i) > 0
        {
            bits[(bits.len() - 1) - i as usize] = true;
        }
    }

    bits
}

fn bits_to_byte_u16(bits: &[bool]) -> u16
{
    let mut byte = 0;

    for (i, bit) in bits.iter().rev().enumerate()
    {
        if *bit
        {
            // byte += 2u8.pow((bits.len() - 1 - i) as u32);
            byte += 2u16.pow((i) as u32);
        }
    }
    byte
}

fn bits_u32(byte: u32) -> [bool; 32]
{
    let mut bits = [false; 32];

    for i in 0 .. 32
    {
        if byte & 2u32.pow(i) > 0
        {
            bits[(bits.len() - 1) - i as usize] = true;
        }
    }

    bits
}

fn bits_to_byte_u32(bits: &[bool]) -> u32
{
    let mut byte = 0;

    for (i, bit) in bits.iter().rev().enumerate()
    {
        if *bit
        {
            // byte += 2u8.pow((bits.len() - 1 - i) as u32);
            byte += 2u32.pow((i) as u32);
        }
    }
    byte
}

fn bits_u64(byte: u64) -> [bool; 64]
{
    let mut bits = [false; 64];

    for i in 0 .. 64
    {
        if byte & 2u64.pow(i) > 0
        {
            bits[(bits.len() - 1) - i as usize] = true;
        }
    }

    bits
}

fn bits_to_byte_u64(bits: &[bool]) -> u64
{
    let mut byte = 0;

    for (i, bit) in bits.iter().rev().enumerate()
    {
        if *bit
        {
            // byte += 2u8.pow((bits.len() - 1 - i) as u64);
            byte += 2u64.pow((i) as u32);
        }
    }
    byte
}

/// Returns the bits of a byte in reverse so that indexing works as expected.
fn bits_rev(byte: u8) -> [bool; 8]
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

/// Converts a slice of bits sorted in reverse to a byte.
fn bits_to_byte_rev(bits: &[bool]) -> u8
{
    let mut byte = 0;

    for (i, bit) in bits.iter().enumerate()
    {
        if *bit
        {
            // byte += 2u8.pow((bits.len() - 1 - i) as u32);
            byte += 2u8.pow((i) as u32);
        }
    }
    byte
}

/// Reads in an EFI Bytecode file from STDIN and prints the disassembly.
fn main()
{
    // disassemble_instruction(
    //     "BREAK".yellow().to_string(),
    //     None,
    //     Some(Argument::ImmediateU64(1)),
    //     None,
    //     None
    // );

    // disassemble_instruction(
    //     "CMP32eq".on_blue().italic().to_string(),
    //     Some(Operand::new_general_purpose(1, false)),
    //     Some(Argument::ImmediateU64(1)),
    //     Some(Operand::new_general_purpose(2, true)),
    //     Some(Argument::ImmediateU16(2)),
    // );

    // disassemble_instruction(
    //     "CMP32eq".blue().italic().to_string(),
    //     Some(Operand::new_general_purpose(1, false)),
    //     Some(Argument::ImmediateU64(1)),
    //     Some(Operand::new_general_purpose(2, true)),
    //     Some(Argument::ImmediateU16(2)),
    // );

    // disassemble_instruction(
    //     "ADD32".purple().to_string(),
    //     Some(Operand::new_general_purpose(1, false)),
    //     None,
    //     Some(Operand::new_general_purpose(2, true)),
    //     None,
    // );

    // disassemble_instruction(
    //     "STORESP".red().to_string(),
    //     Some(Operand::new_general_purpose(1, false)),
    //     None,
    //     Some(Operand::new_dedicated(0, false)),
    //     None,
    // );

    // disassemble_instruction(
    //     "ADD64".purple().bold().to_string(),
    //     Some(Operand::new_general_purpose(1, false)),
    //     Some(Argument::Index16(0x1234)),
    //     None,
    //     None,
    // );

    // disassemble_instruction(
    //     "RET".red().bold().to_string(),
    //     None,
    //     None,
    //     None,
    //     None,
    // );

    // disassemble_instruction(
    //     "SUPER LONG INSTRUCTION".truecolor(255, 200, 0).to_string(),
    //     None,
    //     Some(Argument::Index16(8564)),
    //     None,
    //     None,
    // );


    // disassemble_instruction(
    //     "SUPER LONG INSTRUCTION".truecolor(98, 209, 111).to_string(),
    //     None,
    //     Some(Argument::Index32(3352307477)),
    //     Some(Operand::new_general_purpose(1, false)),
    //     Some(Argument::Index64(3458764547375975133)),
    // );

    let mut show_help = true;
    for bytecode_file in std::env::args().skip(1).take(1)
    {
        show_help = false;
        println!("{}", bytecode_file);

        let file = std::fs::File::open(bytecode_file.clone()).expect(
            format!("File {} does not exist", bytecode_file).as_str()
        );
        let mut bytes = file.bytes().map(|b| b.unwrap());
        // let mut instruction = [0; 4];

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

    if show_help
    {
        println!(
            "Spore - Disassembler for UEFI Bytecode\nUsage: spore <FILENAME>"
        );
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_bits_to_byte()
    {
        assert_eq!(bits_to_byte_u8(&[true, false]), 2u8);
        assert_eq!(bits_to_byte_u8(&[false, true, false]), 2u8);
        assert_eq!(bits_to_byte_u8(&[true, false, false]), 4u8);
        assert_eq!(bits_to_byte_u8(&[true, false, false, false]), 8u8);
        assert_eq!(bits_to_byte_u8(&[true, false, false, true]), 9u8);
        assert_eq!(bits_to_byte_u8(&[true, false, true, true]), 11u8);

        assert_eq!(bits_to_byte_u32(&[true, false]), 2u32);
        assert_eq!(bits_to_byte_u32(&[false, true, false]), 2u32);
        assert_eq!(bits_to_byte_u32(&[true, false, false]), 4u32);
        assert_eq!(bits_to_byte_u32(&[true, false, false, false]), 8u32);
        assert_eq!(bits_to_byte_u32(&[true, false, false, true]), 9u32);
        assert_eq!(bits_to_byte_u32(&[true, false, true, true]), 11u32);

        assert_eq!(bits_to_byte_u64(&[true, false]), 2u64);
        assert_eq!(bits_to_byte_u64(&[false, true, false]), 2u64);
        assert_eq!(bits_to_byte_u64(&[true, false, false]), 4u64);
        assert_eq!(bits_to_byte_u64(&[true, false, false, false]), 8u64);
        assert_eq!(bits_to_byte_u64(&[true, false, false, true]), 9u64);
        assert_eq!(bits_to_byte_u64(&[true, false, true, true]), 11u64);
    }

    #[test]
    fn test_bits()
    {
        assert_eq!(
            bits_u8(2u8),
            [false, false, false, false, false, false, true, false]
        );

        assert_eq!(
            bits_u8(4u8),
            [false, false, false, false, false, true, false, false]
        );

        assert_eq!(
            bits_u8(0x32u8),
            [false, false, true, true, false, false, true, false]
        );
    }

    #[test]
    fn test_natural_indexing()
    {
        let index = NaturalIndex::from_u16(4161);
        assert_eq!(index.constant, 16u64);
        assert_eq!(index.natural, 1u64);
        assert_eq!(index.offset, 24i64);

        let index = NaturalIndex::from_u16(4114);
        assert_eq!(index.constant, 4u64);
        assert_eq!(index.natural, 2u64);
        assert_eq!(index.offset, 20i64);

        let index = NaturalIndex::from_u16(8581);
        assert_eq!(index.constant, 24u64);
        assert_eq!(index.natural, 5u64);
        assert_eq!(index.offset, 64i64);

        let index = NaturalIndex::from_u32(805324752);
        assert_eq!(index.constant, 4u64);
        assert_eq!(index.natural, 2000u64);
        assert_eq!(index.offset, 16004i64);

        let index = NaturalIndex::from_u32(111111);
        assert_eq!(index.constant, 111111u64);
        assert_eq!(index.natural, 0u64);
        assert_eq!(index.offset, 111111i64);

        let index = NaturalIndex::from_u64(2305843035428095952);
        assert_eq!(index.constant, 400000u64);
        assert_eq!(index.natural, 2000u64);
        assert_eq!(index.offset, 416000i64);

        let index = NaturalIndex::from_u32(591751049);
        assert_eq!(index.constant, 214375u64);
        assert_eq!(index.natural, 137u64);
        assert_eq!(index.offset, 215471i64);

        let index = NaturalIndex::from_u64(11529215072282871760);
        assert_eq!(index.sign, -1i8);
        assert_eq!(index.constant, 400000u64);
        assert_eq!(index.natural, 2000u64);
        assert_eq!(index.offset, -416000i64);
    }
}
