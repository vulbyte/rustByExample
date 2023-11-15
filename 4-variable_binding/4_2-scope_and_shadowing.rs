// variable bindings have a scope, and are constrained to live in a block. a block is a collection
// of statements enclosed by {}

fn main() {
    // this binding lives in teh main function
    let long_lived_binding = 1;

    //this blcok has a smaller scope than main //no kidding
    {
        //this binding only exists in this code
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // end of block //🙀 no way! i don't believe you

    //if you call short lived binding here it will error
    //long lived binding will work

    //variable shadowing is allowed
    println!("llb before shadow: {}", long_lived_binding);
    {
        let long_lived_binding = "definantly not the #1";

        println!("llb shadowed: {}", long_lived_binding);
    }
    println!("llb after shadow: {}", long_lived_binding);
}
