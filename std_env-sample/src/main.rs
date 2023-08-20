// -*- mode: Rust; coding: utf-8 -*-
//
// Introduction:
// $ cargo run aaa bbb 123
// Args { inner: ["target/debug/std_env", "aaa", "bbb", "123"] }
//

use std::env;

fn main() {
    let args = env::args();
    println!("{:?}", args);
}
