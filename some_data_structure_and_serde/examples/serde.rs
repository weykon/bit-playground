use serde::{ser, Deserialize, Serialize};
use std::{file, fs::File, io::Write};

fn main() {
    let save_string: String = "ABC".into();

    let mut u8_arr = save_string.as_bytes();

    let mut file = File::create("u8_arr").unwrap();

    file.write_all(&mut u8_arr).unwrap();
}
