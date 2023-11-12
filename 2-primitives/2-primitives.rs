fn main() {
    _test();
}

fn _test() {
    //this will tell the compiler to ignore variables that aren't used
    //#![allow(dead_code)]
    //struct SemanticDirection;

    // variables can be type annotated
    let logical: bool = true;

    let a_float: f64 = 3.0; // float of 64 bits
    let an_interger = 5i32; // int defined in line
                            // or default will be used. floats are f64 (double), 32 for intergers

    // steps can be inferred from context
    let mut inferred_type = 12; // inferred to be i64 from line below
    inferred_type = 3415987213907i64; //

    // a mut variable can be changed
    let mut mutable = 12; //mutable i32
    mutable = 21;

    //error, the types cannot be changed!
    // mutable = true;

    // variables can be overwritten when shadowing

    let mutable = true;
}
