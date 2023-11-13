// the type inference engine will look at how the var is used after it's declared to infer its
// type. here's an example

fn main() {
    //because the annotation, the compiler will set elem to be a u8
    let elem = 5u8;

    //create an empty vector (a growable array).
    let mut vec = Vec::new();
    // the compiler has no idea the type of vec yet

    vec.push(elem);
    //now the compiler knows the vec is composed os u8's
    //if comment this out, program will error

    println!("{:?}", vec);
}
