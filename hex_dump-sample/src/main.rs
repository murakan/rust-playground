// -*- mode: Rust; coding: utf-8 -*-

use rand::Rng;

fn _create_binary_data(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut v = vec![0; size];
    v.iter_mut().for_each(|x| {
        *x = rng.gen::<u8>();
    });
    v
}

fn main() {
    const BYTES_PER_LINE: usize = 16;
    let v = _create_binary_data(1000);

    let mut position = 0;
    for line in v.chunks(BYTES_PER_LINE) {
        print!("[0x{:08X}] ", position);
        for byte in line {
            print!("{:02x} ", byte);
        }
        for byte in line {
            match byte {
                0x20..=0x7E => print!("{}", *byte as char),
                _ => print!("."),
            }
        }
        println!();
        position += BYTES_PER_LINE;
    }
}
