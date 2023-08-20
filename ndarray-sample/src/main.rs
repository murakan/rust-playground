// -*- mode: Rust; coding: utf-8 -*-

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use ndarray::prelude::*;
use ndarray_rand::{rand_distr::Uniform, RandomExt};
use std::fs::File;
use tempfile::NamedTempFile;

fn create_raw_image(shape: (usize, usize)) -> Array2<u16> {
    Array::random(shape, Uniform::new(u16::MIN, u16::MAX))
}

fn write_raw_image(file: &mut File, img: &Array2<u16>) -> std::io::Result<()> {
    let [rows, cols] = img.shape() else { panic!("Shape expected [rows, cols] but {:?}", img.shape()) };
    let shape = (rows, cols);
    println!("{:?}", shape);
    let buf = img.clone().into_raw_vec();
    for data in &buf {
        file.write_u16::<LittleEndian>(*data)?;
    }
    Ok(())
}

fn read_raw_image(file: &mut File, shape: (usize, usize)) -> std::io::Result<Array2<u16>> {
    let (rows, cols) = shape;
    let num_of_data = cols * rows;
    let mut buf = vec![u16::default(); num_of_data];
    match file.read_u16_into::<LittleEndian>(&mut buf) {
        Ok(_) => Ok(Array::from_shape_vec(shape, buf).unwrap()),
        Err(e) => Err(e),
    }
}

fn main() {
    let shape = (3, 5);
    let ndarr = create_raw_image(shape);
    println!("Create new ndarray.");
    println!("{:?}", ndarr);

    if let Ok(tmp_file) = NamedTempFile::new() {
        let path = tmp_file.path();
        println!("{:?}", path);

        if let Ok(mut wr_file) = File::create(path) {
            match write_raw_image(&mut wr_file, &ndarr) {
                Ok(_) => {
                    println!("Raw array file created.");
                    println!("{:?}", wr_file);
                }
                Err(e) => println!("{:?}", e),
            }
        }

        if let Ok(mut rd_file) = File::open(path) {
            match read_raw_image(&mut rd_file, shape) {
                Ok(new_ndarr) => {
                    println!("Raw array file read done.");
                    println!("{:?}", new_ndarr);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
