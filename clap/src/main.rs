// -*- mode: Rust; coding: utf-8 -*-
//
// Introduction:
// $ cargo run 123
// Results: 123
//

use clap::Parser;

/// This structure is for argument options.
#[derive(Debug, Parser)]
struct Args {
    option: String,
}

fn main() {
    let args = Args::parse();
    let opt = args.option;
    println!("Results: {}", opt);
}
