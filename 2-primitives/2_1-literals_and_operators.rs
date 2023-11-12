//integers can be expressed via hexi-decimal, octal, or binary as long as prefixed with the
//respective: 0x, 0o, or 0b
//underscores can be inserted to improve readibility. 1000000 and 1_000_000 are both valid
//scientific notation is also valid, ie 7.6e-4

fn main() {
    // integer addition
    println!("1 - 2 = {}", 1i32 - 2);
    //1i32 and 1u32 are both very important distincitons here

    //scientific nottion
    println!("1e4 if {}, -2.5-3 is {}", 1e4, -2.5e-3);

    //short circuting boolean logic
    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("not true is {}", !true);

    //bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // use underscores to improve readability
    println!("one milion is written as {}", 1_000_000);
}
