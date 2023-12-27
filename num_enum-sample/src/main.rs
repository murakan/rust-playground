// -*- mode: Rust; coding: utf-8 -*-

use std::convert::TryFrom;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[allow(dead_code)]
pub fn type_of<T>(_: &T) -> String {
    let var_type = std::any::type_name::<T>();
    var_type.to_string()
}

#[derive(Debug, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
enum Test {
    A,
    B,
    C,
}

impl Test {
    fn to_num(&self) -> u32 {
        let a = *self;
        println!("{:?}", a);
        a as u32
    }
    fn from_num(num: u32) -> Test {
        Test::try_from(num).unwrap()
    }
}

#[derive(Debug, PartialEq)]
struct StTest {
    name: String,
    value: i32,
}

impl StTest {
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let enum_val = Test::C;
    let val = enum_val as u32;
    let a = Test::try_from(val).unwrap();
    println!("{:?}", a);

    let mut st = StTest {
        name: String::new(),
        value: 0,
    };
    println!("{:?}", st);
    st.set_name("Test Struct");
    println!("{:?}", st);
}
