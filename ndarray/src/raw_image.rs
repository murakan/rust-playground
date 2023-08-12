// -*- mode: Rust; coding: utf-8 -*-

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use ndarray::prelude::*;
use std::io::{Read, Write};

pub trait ReadRawImage<T>: Read {
    fn read_raw_image(&mut self, shape: (usize, usize)) -> Result<Array2<T>, &'static str>;
    fn read_raw_stack(&mut self, shape: (usize, usize, usize)) -> Result<Array3<T>, &'static str>;
}

impl ReadRawImage<u8> for std::fs::File {
    fn read_raw_image(&mut self, shape: (usize, usize)) -> Result<Array2<u8>, &'static str> {
        let num_of_data = shape.0 * shape.1;
        let mut buf: Vec<u8> = vec![u8::default(); num_of_data];
        match self.read_exact(&mut buf) {
            Ok(_) => Ok(Array::from_shape_vec(shape, buf).unwrap()),
            Err(_) => Err("raw u8 image loading failed")
        }
    }
    fn read_raw_stack(&mut self, shape: (usize, usize, usize)) -> Result<Array3<u8>, &'static str> {
        let num_of_data = shape.0 * shape.1 * shape.2;
        let mut buf: Vec<u8> = vec![u8::default(); num_of_data];
        match self.read_exact(&mut buf) {
            Ok(_) => Ok(Array::from_shape_vec(shape, buf).unwrap()),
            Err(_) => Err("raw u8 image loading failed")
        }
    }
}

impl ReadRawImage<u16> for std::fs::File {
    fn read_raw_image(&mut self, shape: (usize, usize)) -> Result<Array2<u16>, &'static str> {
        let num_of_data = shape.0 * shape.1;
        let mut buf: Vec<u16> = vec![u16::default(); num_of_data];
        match self.read_u16_into::<LittleEndian>(&mut buf) {
            Ok(_) => Ok(Array::from_shape_vec(shape, buf).unwrap()),
            Err(_) => Err("raw u8 image loading failed")
        }
    }
    fn read_raw_stack(&mut self, shape: (usize, usize, usize)) -> Result<Array3<u16>, &'static str> {
        let num_of_data = shape.0 * shape.1 * shape.2;
        let mut buf: Vec<u16> = vec![u16::default(); num_of_data];
        match self.read_u16_into::<LittleEndian>(&mut buf) {
            Ok(_) => Ok(Array::from_shape_vec(shape, buf).unwrap()),
            Err(_) => Err("raw u8 image loading failed")
        }
    }
}

impl ReadRawImage<u32> for std::fs::File {
    fn read_raw_image(&mut self, shape: (usize, usize)) -> Result<Array2<u32>, &'static str> {
        let num_of_data = shape.0 * shape.1;
        let mut buf: Vec<u32> = vec![u32::default(); num_of_data];
        match self.read_u32_into::<LittleEndian>(&mut buf) {
            Ok(_) => Ok(Array::from_shape_vec(shape, buf).unwrap()),
            Err(_) => Err("raw u8 image loading failed")
        }
    }
    fn read_raw_stack(&mut self, shape: (usize, usize, usize)) -> Result<Array3<u32>, &'static str> {
        let num_of_data = shape.0 * shape.1 * shape.2;
        let mut buf: Vec<u32> = vec![u32::default(); num_of_data];
        match self.read_u32_into::<LittleEndian>(&mut buf) {
            Ok(_) => Ok(Array::from_shape_vec(shape, buf).unwrap()),
            Err(_) => Err("raw u8 image loading failed")
        }
    }
}

impl ReadRawImage<f32> for std::fs::File {
    fn read_raw_image(&mut self, shape: (usize, usize)) -> Result<Array2<f32>, &'static str> {
        let num_of_data = shape.0 * shape.1;
        let mut buf: Vec<f32> = vec![f32::default(); num_of_data];
        match self.read_f32_into::<LittleEndian>(&mut buf) {
            Ok(_) => Ok(Array::from_shape_vec(shape, buf).unwrap()),
            Err(_) => Err("raw u8 image loading failed")
        }
    }
    fn read_raw_stack(&mut self, shape: (usize, usize, usize)) -> Result<Array3<f32>, &'static str> {
        let num_of_data = shape.0 * shape.1 * shape.2;
        let mut buf: Vec<f32> = vec![f32::default(); num_of_data];
        match self.read_f32_into::<LittleEndian>(&mut buf) {
            Ok(_) => Ok(Array::from_shape_vec(shape, buf).unwrap()),
            Err(_) => Err("raw u8 image loading failed")
        }
    }
}
