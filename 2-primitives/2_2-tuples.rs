//for activity v
use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MATRIX: \n ({}, {}) \n ({}, {}) \n",
            self.0, self.1, self.2, self.3
        )
    }
}

// a tuple is a collection of values of different data types

//tuples can be used as functino arguements and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 'let' can be used ot bind the members of a tuple to vars
    let (int_param, bool_param) = pair;

    //this is the return
    (bool_param, int_param)
}

// the fololwing struct is for the activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // a tuple with abunch of different types:
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // values cna be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    //tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    //touples can be printerd unless they're over 12 elements //why?
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("Pair is {:?}", reverse(pair));

    // creating a tuple with just one element, the () and , is required
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", 5u32);

    //touples can be deconstructed to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("Matrix with :?: \n {:?}, {:?}, {:?}, {:?}", a, b, c, d);

    //adding a :? in the {} will remove any/all fmt'ing
    fn transpose(input: &Matrix) -> Matrix {
        let transposed = Matrix(input.0, input.2, input.3, input.1);
        transposed
    }

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("matrix: \n{}", matrix);
    println!("transposed matrix: \n{}", transpose(&matrix));
}

// Activity
//
//1     Recap: Add the fmt::Display trait to the Matrix struct in the above example, so that if you switch from printing the debug format {:?} to the display format {}, you see the following output:
//
// ( 1.1 1.2 )
// ( 2.1 2.2 )
//
// You may want to refer back to the example for print display.

//2     Add a transpose function using the reverse function as a template, which accepts a matrix as an argument, and returns a matrix in which two elements have been swapped. For example:
//
// println!("Matrix:\n{}", matrix);
// println!("Transpose:\n{}", transpose(matrix));
//
// Results in the output:
//
// Matrix:
// ( 1.1 1.2 )
// ( 2.1 2.2 )
// Transpose:
// ( 1.1 2.1 )
// ( 1.2 2.2 )
