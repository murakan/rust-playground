// -*- mode: Rust; coding: utf-8 -*-

mod raw_image;

use std::fs::File;
use ndarray::prelude::*;
use crate::raw_image::*;

fn _type_of<T>(_: &T) -> String {
    let var_type = std::any::type_name::<T>();
    var_type.to_string()
}

fn main() {
    let mut args = std::env::args();
    if let Some(filename) = args.nth(1) {
        println!("Results: {}", filename);
        let shape = (16, 10);
        if let Ok(mut f) = File::open(filename) {
            let img: Array2<u8> = f.read_raw_image(shape).unwrap();
            println!("{:?}", img);
            let img: Array2<u16> = f.read_raw_image(shape).unwrap();
            println!("{:?}", img);
            let img: Array2<f32> = f.read_raw_image(shape).unwrap();
            println!("{:?}", img);
        }
    }
}
