/*
 * The for in construct can be used to iterate through an Iterator. One of the easiest ways to create an iterator is to use the range notation a..b. This yields values from a (inclusive) to b (exclusive) in steps of one.

Let's write FizzBuzz using for instead of while.
 */

fn fizz_buzz() {
    for n in 1..=100 {
        let mut print_string = n.to_string() + ": ";

        if n % 3 == 0 {
            print_string += "fizz";
        }
        if n % 5 == 0 {
            print_string += "buzz";
        }
        if n % 7 == 0 {
            print_string += "bazz";
        }

        println!("{}", print_string);
    }
}

// Alternatively, a..=b can be used for a range that is inclusive on both ends.

/*
 *  the for in construct is able to intersact with an iterator in several ways.
 *  as discusses in the section on the iterator(part 16) trait, by default the for loop will apple
 *  the into_inter function to the collection.
 *  however this is not hte only means of converting collections into interators.
 *
 *  into_iter, iter, and iter_mut all handle the conversion of a colelction into an iterator in
 *  different views on the data within
 */

fn iter_func() {
    let names = vec!["bob", "frank", "ferris"];

    for name in names.iter() {
        match name {
            &"ferris" => println!("there is a rustacean amoung us!"),
            // TODO ^ try deleting the & and matchign just "Ferris"
            // //   doing so will throw an error
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
    println!("iter_func completed");
}

// this is be just wanting to try make a better fizzbuzz

fn better_fizz_buzz() {
    let fizz_buzz_values: std::collections::HashMap<&str, u32> =
        [("fizz", 3), ("buzz", 5), ("bazz", 7), ("fuzz", 13)]
            .iter()
            .cloned()
            .collect();

    for i in 1..=100 {
        let mut print_string = i.to_string() + ": ";

        for (name, value) in fizz_buzz_values.iter() {
            if i % value == 0 {
                print_string += name;
            }
        }
        println!("{}", print_string);
    }

    println!("better fizzbuzz completed")
}

/*
 *  into_inter - this consumes the colection so that on each iteration teh exact data is provided.
 *  once the collection has been consumed it is no longer avaible for reuse as it has been 'moved'
 *  within the loop
 */
fn into_iter_loop() {
    let names = vec!["bob", "frank", "ferris"];

    for name in names.into_iter() {
        match name {
            "ferris" => println!("there is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    //println!("names: {:?}", names);
    //the above line will error as ownership has changed
}

/*
 *  iter_mut - this mutably borrows each element of the collection, allowing for the collecitno to
 *  be modified in place
 */

fn iter_mut_loop() {
    let mut names = vec!["bob", "frank", "ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "ferris" => "there is a rustacean amongus!",
            _ => "hello",
        }
    }

    println!("names: {:?}", names);
}

/*
 *  in the above snippets, not teh type of match branch. this is the key difference in teh types of
 *  iteration. the difference in the type then of course implies differing action that are able to
 *  be performed
 */

fn main() {
    //default for x in y
    fizz_buzz();
    better_fizz_buzz();

    iter_func();
    into_iter_loop();
    iter_mut_loop();
}
