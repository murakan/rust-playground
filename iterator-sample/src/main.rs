// -*- mode: Rust; coding: utf-8 -*-

fn _type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    println!("Iterator samples");

    // 0 1 2 3 4 5 6 7 8 9
    (0..10).for_each(|v| print!("{} ", v));
    println!();
    // 0 1 2 3 4 5 6 7 8 9 10
    (0..=10).for_each(|v| print!("{} ", v));
    println!();
    // 3 4 5
    (3..).take(3).for_each(|v| print!("{} ", v));
    println!();
    // 1 3 5 6 9
    (1..).step_by(2).take(5).for_each(|v| print!("{} ", v));
    println!();

    // 1 1 1 1 1 1 1 1 1 1
    std::iter::from_fn(|| Some(1))
        .take(10)
        .for_each(|v| print!("{} ", v));
    println!();
    // (0, 11) (1, 12) (2, 13) (3, 14)
    let zero = 11_i32;
    std::iter::successors(Some(zero), |&v| (Some(v + 1)))
        .take(4)
        .enumerate()
        .for_each(|v| print!("{:?} ", v));
    println!();
    // 5
    std::iter::once(5).take(3).for_each(|v| print!("{} ", v));
    println!();
    // 3 3 3 3 3
    std::iter::repeat(3).take(5).for_each(|v| print!("{} ", v));
    println!();

    let vec: Vec<_> = (1..).step_by(3).take(5).collect();
    // [1, 4, 7, 10, 13]
    println!("{:?}", vec);
    // i32(1) i32(4) i32(7) i32(10) i32(13)
    vec.into_iter()
        .for_each(|v| print!("{}({}) ", _type_of(&v), v));
    println!();

    let vec: Vec<_> = (1..).step_by(2).take(7).collect();
    // &i32(1) &i32(3) &i32(5) &i32(7) &i32(9) &i32(11) &i32(13)
    vec.iter().for_each(|v| print!("{}({}) ", _type_of(&v), v));
    println!();
    // [1, 3, 5, 7, 9, 11, 13]
    println!("{:?}", vec);

    let vec: Vec<_> = (1..).step_by(2).take(7).collect();
    // &i32(1) &i32(3) &i32(5) &i32(7) &i32(9) &i32(11) &i32(13)
    (&vec)
        .into_iter()
        .for_each(|v| print!("{}({}) ", _type_of(&v), v));
    println!();
    // [1, 3, 5, 7, 9, 11, 13]
    println!("{:?}", vec);

    let mut vec: Vec<_> = (1..).step_by(5).take(3).collect();
    // &mut i32(1) &mut i32(6) &mut i32(11)
    vec.iter_mut().for_each(|v| {
        print!("{}({}) ", _type_of(&v), v);
        *v += 1;
    });
    println!();
    // [1, 6, 11]
    println!("{:?}", vec);

    let mut vec: Vec<_> = (1..).step_by(5).take(3).collect();
    // &mut i32(1) &mut i32(6) &mut i32(11)
    (&mut vec).into_iter().for_each(|v| {
        print!("{}({}) ", _type_of(&v), v);
        *v += 1;
    });
    println!();
    // [1, 6, 11]
    println!("{:?}", vec);

    let mut vec: Vec<_> = (1..).step_by(3).take(5).collect();
    // 0x7ffce165bc78 [1, 4, 7, 10, 13]
    println!("{:p} {:?}", &vec, vec);
    vec.drain(1..=3);
    // 0x7ffce165bc78 [1, 13]
    println!("{:p} {:?}", &vec, vec);

    // 10
    Some(10).iter().for_each(|v| print!("{:?} ", v));
    println!();

    //
    let text = " ponies \n    giraffes\n     iguanas \nsquid   ".to_string();
    text.lines().map(str::trim).for_each(|v| print!("{} ", v));
    println!();
}
