// HexDump Clone
use std::env;
use std::fs::File;
use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

fn main() -> std::io::Result<()> {
    let arg1 = env::args().nth(1);
    let file_name = arg1.expect("usage: file_view FILENAME");

    let mut f = File::open(&file_name).expect("Unable to open file");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    // Each loop reads up to 16 bytes
    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", pos);

        for byte in &buffer {
            match *byte {
                0x00 => print!(". "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }

        println!();
        pos += BYTES_PER_LINE;
    }

    Ok(())
}
