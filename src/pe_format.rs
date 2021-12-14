// Unused as of now 12/7/21. Keeping around for later.
// PE Spec: https://docs.microsoft.com/en-us/windows/win32/debug/pe-format

mod backend;

use std::io::prelude::*;
use std::fs::File;
use std::time::SystemTime;

/// Glop compiler executable
fn main()
{
    println!("Hello Glop Compiler!");

    let pe_stub = std::fs::read("pe-stub.bin").unwrap();
    let mut file = File::create("out.efi").unwrap();

    file.write_all(&pe_stub).unwrap();

    let system_time = SystemTime::now().duration_since(
        SystemTime::UNIX_EPOCH
    ).unwrap().as_secs();

    file.write_all(&u32::to_le_bytes(system_time as u32)).unwrap();
    file.write_all(&u32::to_le_bytes(0)).unwrap();
    file.write_all(&u32::to_le_bytes(0)).unwrap();

    // TODO(pbz): GO AND DETERMINE SIZE OF OPTIONAL HEADER + WRITE IT (2 bytes)
    file.write_all(&u16::to_le_bytes(0)).unwrap();

    const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002;
    const IMAGE_FILE_32BIT_MACHINE: u16 = 0x0100;
    const IMAGE_FILE_DLL: u16 = 0x2000;
    // TODO(pbz): Might not need this since loader strips relocs by default
    // const IMAGE_FILE_RELOCS_STRIPPED: u16 = 0x0200;
    const CHARACTERISTICS: u16 = {
        IMAGE_FILE_EXECUTABLE_IMAGE
        | IMAGE_FILE_32BIT_MACHINE
        | IMAGE_FILE_DLL
    };

    file.write_all(&u16::to_le_bytes(CHARACTERISTICS)).unwrap();
    file.write_all(&[0x0B, 0x02]).unwrap();  // Standard fields magic
    file.write_all(&u8::to_le_bytes(2)).unwrap();
    file.write_all(&u8::to_le_bytes(0)).unwrap();

    // TODO(pbz): Write address of code. Currently \xA0\x10\x00\x00
}
