/*
 *  the while keyword can be used to run a loop while  acondition is true.
 *  let's write the infamous fizzbuzz using a while loop
 */

fn main() {
    let max_number = 100;
    let mut i = 0;

    while i < max_number {
        i += 1;

        let mut print_string = i.to_string() + ": ";

        if i % 3 == 0 {
            print_string += "fizz";
        }
        if i % 5 == 0 {
            print_string += "buzz";
        }
        if i % 7 == 0 {
            print_string += "bazz";
        }

        println!("{} ", print_string);
    }

    println!("fizzbuzz completed");
}
