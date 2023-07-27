// -*- mode: Rust; coding: utf-8 -*-

mod image;

use byteorder::{ByteOrder, LittleEndian};
use image::*;
use ndarray::*;
// use opencv::core::*;
// use opencv::highgui;
// use opencv::prelude::*;
use rand::Rng;
// use std::convert::TryInto;
use std::fs::File;
use std::io::{Read, Write};

// type RealImage = Array2<f32>;
// type RealImageStack = Array3<f32>;

fn new_image(shape: (usize, usize)) -> RealImage {
    let img = RealImage::zeros(shape.into_shape());
    let mut rng = rand::thread_rng();
    img.mapv(|_| -> f32 { rng.gen::<f32>() })
}

fn _create_binary_file(filename: &str) {
    println!("{}", filename);
    let mut rng = rand::thread_rng();
    let mut v = Vec::<u16>::new();
    for _ in 0..256 {
        v.push(rng.gen());
    }
    println!("{:?}", v);
    if let Ok(mut f) = File::create(filename) {
        let mut buf: Vec<u8> = vec![0; v.len() * 2];
        LittleEndian::write_u16_into(&v, &mut buf);
        f.write(&buf);
    }
}

fn _load_binary_file(filename: &str) {
    println!("{}", filename);
    let mut v = Vec::<u16>::new();
    if let Ok(mut f) = File::open(filename) {
        let mut buf = Vec::<u8>::new();
        f.read_to_end(&mut buf);
        v.resize(buf.len() / 2, 0);
        LittleEndian::read_u16_into(&buf, &mut v);
    }
    println!("{:?}", &mut v);
}

fn save_image(filename: &str, img: &RealImage) {
    if let Ok(mut f) = File::create(filename) {
        f.write_image(img);
    }
}

fn main() {
    println!("Results:");
    let img = new_image((5, 3));
    save_image("test.bin", &img);
    // println!("{:?} {}", img, image::_type_of(&img));
    // // let v = img.into_raw_vec();
    // let shape = img.shape();
    // println!("{:?} {}", shape, image::_type_of(&shape));
    // if let [cols, rows] = shape {
    //     println!("{}x{}", cols, rows);
    // }
    // let [rows, cols] = shape;
    // let img_slice = img.as_slice().unwrap();
    // let mat = Mat::from_slice_rows_cols(img_slice, img.nrows(), img.ncols()).unwrap();
    // println!("{:?}", mat);
    // highgui::imshow("image", &mat).unwrap();
    // highgui::wait_key(0).unwrap();
    // create_binary_file("test.bin");
    // load_binary_file("test.bin");

    // let v: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    // let a = array![[0, 1, 2, 3, 4], [5, 6, 7, 8, 9]];
    // let b = Array::from_shape_vec((3, 2, 2), v).unwrap();
    // // println!("{:?}", v);
    // println!("{:?}", a);
    // println!("{:?}", b);
}
