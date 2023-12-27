// -*- mode: Rust; coding: utf-8 -*-
//
// Introduction:
// $ cargo run
//
// Results:
// ObjectA called.
// ObjectB called.
// ObjectA(10)
// ObjectB(200.0)
// Finish successfully.
//

#[derive(Debug)]
struct ObjectA(i32);

#[derive(Debug)]
struct ObjectB(f32);

#[derive(Debug)]
struct Target {
    ivalue: i32,
    fvalue: f32,
}

trait TestTrait<T> {
    fn func(&self) -> T;
}

impl TestTrait<ObjectA> for Target {
    fn func(&self) -> ObjectA {
        println!("ObjectA called.");
        ObjectA(self.ivalue)
    }
}

impl TestTrait<ObjectB> for Target {
    fn func(&self) -> ObjectB {
        println!("ObjectB called.");
        ObjectB(self.fvalue)
    }
}

fn proc() -> std::io::Result<()> {
    let t = Target {
        ivalue: 10,
        fvalue: 200.,
    };
    let a: ObjectA = t.func();
    let b: ObjectB = t.func();
    println!("{:?}", a);
    println!("{:?}", b);
    Ok(())
}

fn main() {
    match proc() {
        Ok(_) => println!("Finish successfully."),
        Err(e) => eprintln!("{:?}", e),
    }
}
