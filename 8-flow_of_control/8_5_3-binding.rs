/*
 *  indirectly accessing a variable makes it impossible to branch and use that variable without
 *  rebinding.
 *  match provides the @ signal for binding values to names:
 */

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("tell me what type of person you are");
    match age() {
        0 => println!("i haven't celebreated my first birthday yet"),
        //could 'atch' 1 ..=12 directly, but then what age would the cild be?
        //instead bind to n for the sequence of 1..=12. now the age can be reported
        n @ 1..=12 => println!("i am a child of age {:?}", n),
        n @ 13..=19 => println!("i am a teen of age {:?}", n),
        //nothing bound, return the result
        n => println!("i'm an old person of age {:?}", n),
    }

    /*
     *  you can also use binding to "destructure" enum variats such as Option:
     */
    match some_number() {
        //got 'some' variant, match if its value, bound t 'n', is = to 42
        Some(n @ 42) => println!("the answer: {}!", n),
        //match any other number
        Some(n) => println!("not interesting... {}", n),
        //match anything else
        _ => (),
    }
}
