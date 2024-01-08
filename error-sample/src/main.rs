// -*- mode: Rust; coding: utf-8 -*-

use anyhow::{anyhow, Context, Result};
use thiserror::Error;

/// simple error
fn simple_error() -> Result<()> {
    Err(anyhow!("Error message by anyhow macro."))
}

/// Conversion error
fn convert_error() -> Result<()> {
    "abcde".parse::<i32>()?;
    Ok(())
}

/// I/O error
fn file_error() -> Result<()> {
    std::fs::File::open("xxxx")?;
    Ok(())
}

/// I/O error with context.
fn file_error_with_context() -> Result<()> {
    std::fs::File::open("xxxx").with_context(|| "Original error message")?;
    Ok(())
}

/// I/O error with context.
fn file_error_with_param(file: &str) -> Result<()> {
    std::fs::File::open(file).with_context(|| format!("Formated error message {file}"))?;
    Ok(())
}

#[derive(Debug, Error)]
enum CustomError {
    #[error("File is not found. {0}")]
    FileNotFound(&'static str),
    #[error("Conversion error. {0}")]
    ConversionError(i32),
}

/// custom file error
fn custom_file_error(file: &'static str) -> Result<()> {
    Err(anyhow!(CustomError::FileNotFound(file)))
}

/// custom conversion error
fn custom_conversion_error(number: i32) -> Result<()> {
    Err(anyhow!(CustomError::ConversionError(number)))
}

fn main() {
    println!("Error sample.");
    let res = simple_error();
    println!("{:?}", res);
    let res = convert_error();
    println!("{:?}", res);
    let res = file_error();
    println!("{:?}", res);
    let res = file_error_with_context();
    println!("{:?}", res);
    let res = file_error_with_param("abcde");
    println!("{:?}", res);
    let res = custom_file_error("abcde");
    println!("{:?}", res);
    let res = custom_conversion_error(123);
    println!("{:?}", res);
}
