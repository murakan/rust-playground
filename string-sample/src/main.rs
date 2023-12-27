// -*- mode:rust; -*-
//
// for UTF-8
// is_numeric
// is_alphabetic
// is_alphanumeric
// is_whitespace
// is_control
//
// for ascii
// is_ascii
// is_ascii_alphabetic
// is_ascii_digit
// is_ascii_hexdigit
// is_ascii_alphanumetic
// is_ascii_control
// is_ascii_graphic
// is_ascii_uppercase
// is_ascii_lowercase
// is_ascii_punctuation
// is_ascii_whitespace
//

fn main() {
    let text = "1１①ⅠaＡａε 　\n\r\t";
    println!("{:?}", text);
    is_func("is_numeric", |ch| ch.is_numeric(), text);
    is_func("is_alphabetic", |ch| ch.is_alphabetic(), text);
    is_func("is_alphanumeric", |ch| ch.is_alphanumeric(), text);
    is_func("is_whitespace", |ch| ch.is_whitespace(), text);
    is_func("is_control", |ch| ch.is_control(), text);
    is_func("is_ascii", |ch| ch.is_ascii(), text);
    is_func("is_ascii_alphabetic", |ch| ch.is_ascii_alphabetic(), text);
    is_func("is_ascii_digit", |ch| ch.is_ascii_digit(), text);
    to_func("to_digit", |ch| ch.to_digit(16), text);
}

// fn _type_of<T>(_: &T) -> &'static str {
//     std::any::type_name::<T>()
// }

fn is_func<F>(name: &str, f: F, text: &str)
where
    F: Fn(&char) -> bool,
{
    print!("{:20}: ", name);
    text.chars().filter(f).for_each(|ch| print!("{:?} ", ch));
    println!();
}

fn to_func<F>(name: &str, f: F, text: &str)
where
    F: Fn(&char) -> Option<u32>,
{
    print!("{:20}: ", name);
    text.chars().for_each(|ch| {
        if let Some(val) = f(&ch) {
            print!("{:?}->{} ", ch, val);
        }
    });
    println!();
}

// fn slice_bytes(arr: &[char]) {
//     arr.bytes().for_each(|x| print!("{}", x));
// }
