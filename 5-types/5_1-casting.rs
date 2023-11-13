//rust provides no implicit type conversion (coercion) between primitive types.
//but explicit type conversion (casing) can be preformed using the 'as' keyword.
//
//rules for converting between integral types follow C conventions generally,
//except in cases where C has undefined behavior.
//
//the behavior of all casts between integral types is well defined in Rust.

//surpress all warning from casts which overflow
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    //error no implicit conversion
    //let interger: u8 = decimal;

    //explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    //error! there are limitations in conversion rules.
    //a float cannot be directly convered into a char
    //let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    //when casting any value to an unsigned type, T,
    //T::MAX +1 is added or subtracted until value fits into new type

    //1000 already fits a u16, so it wouldn't be casted
    //1000 doesn't fit a u8 however, so it would be reduced until it fits.
    //ie: 1000-256-256-256 = 232
    //under the hood the first 8 least significant bits(LSB) are kept, while the rest towards the
    //most significant bit (MSB) are tuncated
    //aka, THIS IS NOT A PROPER CONVERSION, is more similar to bitshifting until it fits then
    //converting.
    println!("1000 as a u8 is: {}", 1000 as u8);
    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    //for positive numbers, this is the same as modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    //when casting to a signed type, the (bitwise) result is the same as first casting to the
    //corrosponding unsigned type.
    //if the most significant bit of that value is 1, then the value is negative
    //
    //unless it already fits of course
    println!("128 as a i16 is: {}", 128 as i16);

    //in boundary case 128 in 8-bit two's complement representation of is -128
    println!("128 as a i8 is: {}", 128 as i8);

    //since rust 1.45, the 'as' keyword preforms a *saturating cast* when casting from a float to an int.
    //if the floating point value exceeds the upper bound or is less than the lower bound, the
    //retunred value will be equal to the bound crossed

    //300.0 as u8 is 255
    println!("300 as u8 is: {}", 300.0_f32 as u8);
    // -100 as a u8 is 0
    // nan as a u8 is 0

    //this behavior incurs a small run time cost and can be avoided with unsafe methods.  however
    //the results might overflow and retunr **unsound values**. use these methods wisely:

    unsafe {
        println!("unsafe start");
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!(
            "-100.0 as u8 is : {}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());

        println!("unsafe end");
    }
}
