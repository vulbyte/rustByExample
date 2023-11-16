/*
 *  tuples can be destructured in a match as follows:
 */

fn main() {
    let triple = (3, 0, -2, 3, 4);

    println!("tell me about {:?}", triple);

    //match can be used to destructure a tuple
    match triple {
        // destructuer teh second and third elements
        (0, w, x, y, z) => println!("first is 0, w:{}, x:{}, y:{}, z:{}", w, x, y, z),
        (1, ..) => println!("first is 1, and the rest doesn't matter"),
        (.., 2) => println!("the last is 2, the rest don't matter"),
        (3, .., 4) => println!("first is 3, last is 4, rest don't matter"),
        _ => println!("doesn't matter what they are"),
    }
}
