use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read, Write},
};

fn main() {
    let mut a_space: u8 = 0b00000010;

    println!("a 8bit space: {} ", a_space.to_owned());

    create_chunk();
}

// chunk
fn create_chunk() {
    let mut a_chunk_space: [u8; 2] = [0b000000010, 0b00000000];
    println!("a a_chunk_space space: {:b} ", a_chunk_space[0]);

    let serialized = serialize(&a_chunk_space).unwrap();
    println!("serialized: {:?}", serialized);

    let exist = File::open("my_data");

    match exist {
        Ok(mut v) => {
            println!("result :  {:?}", v);
            let mut buffer = Vec::new();
            v.read_to_end(&mut buffer).unwrap();
            println!("read :  {:?} ", buffer);
        }
        Err(err) => {
            println!("err :  {:?}", err.kind());
            if err.kind() == io::ErrorKind::NotFound {
                let file = match File::create("my_data") {
                    Ok(mut file) => file.write_all(&serialized),
                    Err(err) => {
                        println!("err: {}", err);
                        return;
                    }
                };
                println!("file content: {:?}", file);
            }
        }
    }
}
