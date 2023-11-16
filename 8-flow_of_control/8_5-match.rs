/*
 *  rust provides matching via the match keyword, which can be used like a C switch, the first
 *  matching arm is evaluated and all possible values must be covered
 */

fn main() {
    let numeber = 13;

    println!("Tell me about {}", number);

    match number {
        //match a single value
        1 => println!("One!"),

        //match several values
        2 | 3 | 5 | 7 | 11 => println!("this is a prine!"),

        //match an inclusive range
        13..=19 => println!("a teen number"),

        //default case
        _ => println!("aint nothing special"),
    }

    let boolean = true;
    // match is an expression too
    let binary = match boolean {
        // the arms must cover all possible values
        false => 0,
        true => 1,
        //is one is commented out, this will fail
    };

    println!("{} -> {}", boolean, binary);
}
