// snippet of code @ 2023-09-03 09:40:21

// === Rust Playground ===
// This snippet is in: ~/.emacs.d/rust-playground/at-2023-09-03-094020/

// Execute the snippet: C-c C-c
// Delete the snippet completely: C-c k
// Toggle between main.rs and Cargo.toml: C-c b

use byteorder::{ReadBytesExt, LE};
use opencv::core::Mat;
use std::io::Cursor;

fn main() {
    let cols = 4_usize;
    let rows = 3_usize;
    let mut buff = Vec::<u8>::default();
    for index in 1..=(cols * rows * std::mem::size_of::<u16>()) {
        buff.push(index as u8);
    }
    println!("{:?}", buff);

    // Create Vec<T> from byte slice.
    let data_size = cols * rows;
    println!("{}", data_size);
    let mut data = vec![0_u16; data_size];
    let mut cursor = Cursor::new(&mut buff);
    cursor.read_u16_into::<LE>(&mut data).unwrap();

    // Convert from slice to Mat with shape.
    match Mat::from_slice_rows_cols(data.as_slice(), cols, rows) {
        Ok(mat) => println!("{:?}", mat),
        Err(e) => println!("{:?}", e),
    }
}
