/*
 *  similar to if let, while let can make awkward match sequences mroe tolerable. consider the
 *  following squence that increments one
 */

fn example_increment() {
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // Repeatedly try this test.
    loop {
        match optional {
            // If `optional` destructures, evaluate the block.
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ Requires 3 indentations!
            }
            // Quit the loop when the destructure fails:
            _ => {
                break;
            } // ^ Why should this be required? There must be a better way!
        }
    }
}

fn main() {
    example_increment();

    //using while let makes this sequence much nicer:

    let mut optional = Some(0);

    //this reads:
    //while 'let' destructures 'optional' into 'Some(i)', evaluate the block, else break.
    while let Some(i) = optional {
        if i > 9 {
            println!("greater than 9, quit!");
            optional = None;
        } else {
            println!("'i' is '{:?}'. try again.", i);
            optional = Some(i + 1);
        }
        // less rightward drift and doesn't require explicitly handling the failing case
    }
    // * if let had additional option else/else if clauses. while let does not have these.
}
