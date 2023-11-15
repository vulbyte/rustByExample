/*
 * the from and into traits are inherently linked, and this is actually part of its implementation.
 * if you are abote to convert type a from type b, then it should be easy to believe that we should
 * be able to do vise versa aswell.
 */

/*
 *  the from trait allowsw for a type to define how to create itself from another type, hence
 *  providing a very simple mechanism for cvonverting between several types. there are numerous
 *  implementations of this trait within the standard library for conversion of primitive and
 *  common types.
 *
 *  as an example, you can easily convert form a str to a string via:
 *  let my_str = "hello";
 *  let my_string = String::from(my_str);
 *
 *  we can do a similar thing for defining a conversion for our own type
 */

use std::convert::From;
use std::convert::Into;

#[allow(dead_code)]
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

fn main() {
    let num = Number::from(32);
    println!("my number is {:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("my number is {:?}", num);
}

/*
 *  the into trait is simply the reciprocal of the from traint, that is, if you have implemented
 *  the from trait for your type, into will call it when necessary.
 *
 *  usignt he into trait will typically require specificationof the type to convert into something
 *  as the compiler is unable ot determine this most of the time. however this is a small trade-off
 *  considering we get the functionality for free.
 */
