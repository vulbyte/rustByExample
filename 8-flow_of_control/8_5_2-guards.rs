/*
 *  a match guard can be added to filter the arm
 */

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30C", t),
        // the if conditions is a guard
        Temperature::Celsius(t) => println!("{}C is below 30C", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is hotter then 86F", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86F", t),
    }

    /*
     *  note that the compiler won't take guard conditions into account when checking if all
     *  patterns are covered by the match expression
     */
}
