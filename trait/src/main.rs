// -*- mode: Rust; coding: utf-8 -*-

pub fn _type_of<T>(_: &T) -> String {
    let var_type = std::any::type_name::<T>();
    var_type.to_string()
}

#[derive(Debug)]
struct Integer {
    val: i32,
}

#[derive(Debug)]
struct Float {
    val: f32,
}

trait ReadData<T> {
    fn read(&self) -> T;
}

impl ReadData<i32> for Integer {
    fn read(&self) -> i32 {
        self.val
    }
}

impl ReadData<f32> for Float {
    fn read(&self) -> f32 {
        self.val
    }
}

fn show<T: std::fmt::Display, U: ReadData<T>>(obj: U) {
    println!("{} {}", _type_of(&obj), obj.read());
}

fn main() {
    println!("Rust playground!");
    let i = Integer { val: 100 };
    let f = Float { val: 8.2 };
    show(i);
    show(f);
}