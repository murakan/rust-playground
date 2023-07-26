// snippet of code @ 2023-07-24 13:44:20

// === Rust Playground ===
// This snippet is in: ~/.emacs.d/rust-playground/at-2023-07-24-134420/

// Execute the snippet: C-c C-c
// Delete the snippet completely: C-c k
// Toggle between main.rs and Cargo.toml: C-c b

use clap::Parser;

/// This structure is for argument options.
#[derive(Debug, Parser)]
struct Args {
    //    #[arg(required = true)]
    filename: String,
}

fn load_filename() -> String {
    let args = Args::parse();
    args.filename
}

fn main() {
    let fname = load_filename();
    println!("Results: {}", fname);
}
