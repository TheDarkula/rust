// run-pass
#![feature(box_syntax)]

use std::ops::FnMut;

 fn make_adder(x: i32) -> Box<FnMut(i32)->i32+'static> {
    (box move |y: i32| -> i32 { x + y }) as
        Box<FnMut(i32)->i32+'static>
}

pub fn main() {
    let mut adder = make_adder(3);
    let z = adder(2);
    println!("{}", z);
    assert_eq!(z, 5);
}
