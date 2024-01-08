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
    parameter: String,
    #[clap(short, long)]
    option: Option<String>,
}

fn main() {
    let args = Args::parse();
    let param = args.parameter;
    let opt = args.option;
    println!("Results: {} {:?}", param, opt);
}
