use std::io::{Write, Result};

fn main() {
    println!("hello");
    let entirety_data = mock_data();
}

fn mock_data() -> Vec<u8> {
    // mock a array of string,

    let string = String::from("abcdefg");

    string.into_bytes()
}

pub trait Serialize {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<() >;
}

struct DataFrame {
    data: u8,
    pos: usize,
}

impl Serialize for DataFrame {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[self.data])?;
        Ok(())
    }
}