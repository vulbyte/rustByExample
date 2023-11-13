// rust has two different types of constants which cna be ceclared in any scope including global.
// both require explicit type annotation:
//
// const: an unchangeable value (the common case)
// static: a possibly mutable variable wich 'static lifetime. the static lifetime is infererd and
// does not have to be specified. Access or modify a mutable static var is 'unsafe'

//Globals are decalred oujtside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    //access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    //access constant in the mani thread
    println!("This is {} code", LANGUAGE);
    println!("the threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    //error  cannot modify a 'const'
    //THRESHOLD = 5;
    // FIXME ^ comment out this line
}

//see also:
//
// the const/static RFC: https://github.com/rust-lang/rfcs/blob/master/text/0246-const-vs-static.md
// 'static lifetime: https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html
