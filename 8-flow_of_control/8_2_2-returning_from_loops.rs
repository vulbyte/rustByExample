/*
 * one fo the uses of a loop is to retry an operation until it succeeds. if the operations returns
 * a value though, you might need to pass it to teh rest fo the code: but it after the break and it
 * will be returned by the loop expression
 */

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 1); // because this isn't == the program will panic and quit early

    println!("if the above 'assert_eq!()' is not true, then this shouldn't print");
}
