fn main() {
    let _immuitable_binding = 1;
    let mut mutable_binding = 1;

    println!("before mutation: {}", mutable_binding);

    // ok
    mutable_binding += 1;

    println!("After mutation {}", mutable_binding);

    //error, cannot assign new value
    //_immutable_binding += 1;
}
