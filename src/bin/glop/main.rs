use std::io::prelude::*;
use std::fs::File;

pub trait FileWriter
{
    fn write_bytes(&mut self, data: &[u8]);
}

impl FileWriter for File
{
    fn write_bytes(&mut self, data: &[u8])
    {
        let mut pos = 0;

        while pos < data.len()
        {
            pos += self.write(&data[pos..]).unwrap();
        }
    }
}

/// Glop compiler executable
fn main()
{
    println!("Hello Glop Compiler!");
    let data = b"Some Bytes";
    let mut file = File::create("out.efi").unwrap();

    file.write_bytes(data);
}
