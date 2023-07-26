// -*- mode: Rust; coding: utf-8 -*-

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use ndarray::*;
// use opencv::core::*;
// use std::io::Read as StdRead;

pub fn _type_of<T>(_: &T) -> String {
    let var_type = std::any::type_name::<T>();
    var_type.to_string()
}

pub type RealImage = Array2<f32>;
pub type RealImageStack = Array3<f32>;

pub trait Image {
    // fn write_image<T: Dimension>(&mut self, img: T);
    // fn read_image<T: Dimension>(&mut self, shape: (usize, usize, usize)) -> T;
    fn write_image(&mut self, img: &RealImage);
    fn read_image(&mut self, shape: (usize, usize, usize)) -> RealImage;
}

impl Image for std::fs::File {
    fn write_image(&mut self, img: &RealImage) {
        if let [cols, rows] = img.shape() {
            let v = ndarrary_to_vec(img);
            self.write_f32::<LittleEndian>(&v).unwrap();
            println!("{:?}", v);
        }
    }

    fn read_image(&mut self, shape: (usize, usize, usize)) -> RealImage {
        RealImage::zeros((shape.0, shape.1))
    }

    // fn read_image<RealImage: Dimension>(&mut self, shape: (usize, usize, usize)) -> RealImage {
    //     let cols = shape.0;
    //     let rows = shape.1;
    //     let slices = shape.2;

    //     let mut buf = vec![0_f32; cols * rows * slices];
    //     self.read_f32_into::<LittleEndian>(&mut buf).unwrap();
    //     let a = Array::from_shape_vec((rows, cols), buf).unwrap();
    //     println!("{:?}", a);
    //     println!("{}", _type_of(&a));
    //     // let shape = (rows as usize, cols as usize);
    //     RealImage::zeros(2)
    //     // RealImage::zeros(shape.into_shape())
    // }

    // fn read_image<RealImageStack>(&mut self) {
    //     let (cols, rows, slices, bytes) = self.read_header();
    //     assert!(bytes == std::mem::size_of::<f32>());
    //     assert!(2 < slices);

    //     let mut buf = vec![0_f32; cols * rows * slices];
    //     self.read_f32_into::<LittleEndian>(&mut buf).unwrap();
    //     let a: Array<f32, _> = buf.try_into().unwrap();
    //     let b = a.into_shape((slices, cols, rows));
    //     println!("{:?}", b);
    //     println!("{}", type_of(&b));
    // }
}

fn ndarrary_to_vec<T>(img: &Array2<T>) -> Vec<T>
where
    T: Clone,
{
    img.view().t().into_iter().cloned().collect()
}
