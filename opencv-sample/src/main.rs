// -*- mode: Rust; coding: utf-8 -*-

use byteorder::{LittleEndian, ReadBytesExt};
use opencv::highgui::*;
use opencv::prelude::*;
use rand::prelude::*;
use std::io::Cursor;

fn create_byte_array(size: &[usize]) -> Vec<u8> {
    let [cols, rows, slices] = size else { todo!(); };
    let total_bytes = slices * rows * cols * std::mem::size_of::<u16>();
    let mut u8_data: Vec<u8> = vec![0; total_bytes];
    rand::thread_rng().fill(&mut u8_data[..]);
    u8_data
}

fn main() {
    let cols = 512;
    let rows = 512;
    let slices = 32;
    let u8_data = create_byte_array(&[cols, rows, slices]);
    let mut cursor = Cursor::new(u8_data);
    let num_of_data = cols * rows * slices;
    let mut u16_data: Vec<u16> = vec![0; num_of_data];
    cursor.read_u16_into::<LittleEndian>(&mut u16_data).unwrap();
    assert_eq!(u16_data.len(), num_of_data);

    let mut stacks: Vec<Mat> = vec![];
    for slice in 0..slices {
        let begin = cols * rows * slice;
        let end = begin + (cols * rows);
        let frame = Mat::from_slice_rows_cols(&u16_data[begin..end], rows, cols).unwrap();
        stacks.push(frame);
    }

    for (index, stack) in stacks.iter().enumerate() {
        println!("index: {} / {}", index + 1, stacks.len());
        imshow("image", &stack).expect("Can't show image");
        let key = wait_key(0).expect("Key handling error");
        if key == ('q' as i32) {
            break;
        }
    }
}
