/*
 *  CONVERTING TO A STRING
 *  to convert any type to a string, it's as simple as implementing the 'ToString' trait for the
 *  type.
 *  you can do so from the fmt::display trait
 */

//use std::fmt;

struct Circle {
    radius: i32,
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    /*
     *  PARSING A STRING
     *  one of the most common types to convert a string in to a number.
     *  the idiomatic approach to this is to use the parse function and either to arrange the type
     *  interface or to specify the type to parse using the 'turbofish' syntax. both alternatives are
     *  shown in the following example.
     *
     *  this will conver the string into the type specified as long as the FromStr trait is
     *  implemented for that type. This is implemented for numerous types within the standard
     *  library. to obtain this functionality on a user defined type simple implement the FromStr
     *  trait for that type
     */
    let parsed: i32 = "5".parse().unwrap();
    // a test to see if it can be done in parts (it cannot)
    //let parsed: i32 = "5";
    //println!("{parsed}");

    //parsed = parsed.parse();
    ////println!("{parsed}");

    //parsed = parsed.parse().unwrap();
    //println!("{parsed}");

    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;

    println!("Sum: {:?}", sum)
}
