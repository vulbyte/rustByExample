#![allow(unreachable_code, unused_labels)]

fn main() {
    'outer: loop {
        println!("you have entered the outer loop");

        'inner: loop {
            println!("you have entered the inner loop");

            //  this will only break the inner loop
            // break;

            // this breaks the outer loop
            break 'outer;

            // this^ is the coolest thing i've seen in programming in a while
        }

        println!("this point will never be reached if ''break 'outer'' is above this");
    }

    println!("exited the outer loop");
}
