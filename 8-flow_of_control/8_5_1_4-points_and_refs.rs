/*
 *  for pointers, a distinction needs to be made between destructing and dereferencing as they are
 *  different concepts which re used differently from languages like c/c++
 *
 *  dereferencing uses *
 *  restructuring uses &, ref, and ref mut
 */

fn main() {
    //assign a reference of type 'i32'.
    //the '&' signifies there is a reference being assigned.

    let reference = &4;

    match reference {
        // fi 'reference' is pattern matched against '&val', it will return in a comperison like:
        // '&i32'
        // '&val'
        // ^ we see that if the matchuing '&'s are dropped, then the 'i32' should be assigned 'val'
        &val => println!("got a galue via dereferencing: {:?}", val),
    }

    // to avoid the "'&', you dereference before matching
    match *reference {
        val => println!("got a value via dereferencing: {:?}", val),
    }

    //what if you don't start with a  reference?
    //'referece' was a '&' because the right side was already a reference.
    //this is not a reference because the right side is not one
    let _not_a_reference = 3;

    //rust provides 'ref' for exactly this purpose. it modifies the assignment so that a reference
    //is created for the elementl this reference is assigned
    let ref _is_a_reference = 3;

    // accordingly, by defining 2 values without references, references, can be retrieved via 'ref'
    // and 'ref mut'.
    let value = 5;
    let mut mut_value = 6;

    //use ref keyword to create a reference
    match value {
        ref r => println!("got a ref value form: {:?}", r),
    }

    // use 'ref mut' similarly
    match mut_value {
        ref mut m => {
            //get a reference, gotta dereference it before we can add anything to it
            *m += 10;
            println!("we added 10, 'mut_value': {:?}", m);
        }
    }
}
