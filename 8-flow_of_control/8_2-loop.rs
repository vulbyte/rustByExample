/*
 *  rust provides a loop keyword to indicate an infinate loop.
 *  teh break statement can be used to exit a loop at anytime,
 *  whereas the continue statement can be used to skip the rest of the iteration and start a new
 *  one.
 */

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // infinate loop
    loop {
        count += 1;

        if count == 3 {
            println!("three!");

            //skip the rest
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("ok, i'm bored now");

            //exit the loop
            break;
        }
    }

    //  none of the below works, it needs a loop statement to function
    //
    // println!("this is a test to see if labeled \{\} can take advantage of a continue statement");
    // {
    //     println!("start of if");
    //     if count >= 1 {
    //         println!("condition met, should continue");
    //         // continue; // this doesn't work
    //         // break; // nor does this
    //     }
    //     println!("end of if");

    //     println!("end of if with continue after this print");
    //     // continue; // this doesn't work
    //     // break; // nor does this
    //     println!("after continue with no logic {}");
    // }
}
