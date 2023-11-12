use std::mem; // .w.

//this function browwows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("Frist element of slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    //fixed size array (type sig is superfluous)
    //all elements can be initialized to the same value
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    //all elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    //indexing starts at 0.
    println!("first element of the array: {}", xs[0]);
    //you know the syntax from their

    //'len()' returns the count of the elemetns in the array.
    println!("number of elements in array: {}", xs.len());

    //Arrays are stack allocated
    println!("Array occupied {} bytes", mem::size_of_val(&xs));

    //arrays can be automatically borrowed as slices/
    println!("borrow the whole array as a slice.");
    analyze_slice(&xs);

    //spices can point to a cection of an array.
    // they are of the form [starting index..ending index].
    // ending_index is one more than the last position in the slice.
    println!("borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    //example of empty slice '&[]';
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); //same as above but more verbose

    //arrays can be safely accessed using '.get', which returns an 'Option'.
    //this can be matched as shown below,
    //or used with  '.expect()' if you would like the program ot exit with a nice message instead of happily continue
    for i in 0..xs.len() {
        //oops, one element too far! (they had a +1 in the example)
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("slow down! {} is too far!", i),
        }
    }
    // out of bound indexing on an array causes a compile time error.
    // println!("{}", xs[5]);
    // out of bound indexing on a slice causes a runtime error.
    // println!("{}", xs[..][5]);
}
