// -*- mode: Rust; coding: utf-8 -*-

use anyhow::Result;
use clap::Parser;
use std::fs::File;
use tiff::decoder::{Decoder, DecodingResult};
use tiff::encoder::{colortype, TiffEncoder};

/// Create a multi-page TIF from multiple TIF images.
/// The pixel type of each image is u16.
fn create_multipage_tif(src_files: &[String], dst_file: &str) -> Result<()> {
    let mut writer = TiffEncoder::new(File::create(dst_file)?)?;
    for src_file in src_files {
        let mut decoder = Decoder::new(File::open(src_file)?)?;
        if let DecodingResult::U16(pixels) = decoder.read_image()? {
            let (cols, rows) = decoder.dimensions()?;
            let img = writer.new_image::<colortype::Gray16>(cols, rows)?;
            img.write_data(&pixels)?;
        }
    }
    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output: String,
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();
    match create_multipage_tif(&args.files, args.output.as_str()) {
        Err(e) => println!("{e:?}"),
        _ => println!("Finished successfuly."),
    }
}
