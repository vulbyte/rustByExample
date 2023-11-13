// numeric literals can be type annotated by adding teh type as a suffix. as an example, to specify
// that the literal 42 should have the type i32 write 42i32
//
//the type of unsuffixed nuymeric literals will depend on how they are used if no constants exists,
//the compiler will use i32 for integers, and f64 for floating point numbers

fn main() {
    // suffixed literals
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // 'size_of_val returns the size of a var in bytes
    println!("size of 'x' in bytes: {}", std::mem::size_of_val(&x));
    println!("size of 'y' in bytes: {}", std::mem::size_of_val(&y));
    println!("size of 'z' in bytes: {}", std::mem::size_of_val(&z));
    println!("size of 'i' in bytes: {}", std::mem::size_of_val(&i));
    println!("size of 'g' in bytes: {}", std::mem::size_of_val(&f));
}
