//rust
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let f = File::open("test.txt")?;

    for byte in f.bytes() {
        match byte.unwrap() {
            t @ 49...57 => println!("Número {}", t as char),
            t @ _ => println!("Letra {}", t as char),
        };
    }
    Ok(())
}
