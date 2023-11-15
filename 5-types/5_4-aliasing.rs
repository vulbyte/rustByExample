//the type statement can be used to give a new name to an existing type. Types must have
//UpperCamelCase names, or the compiler will raise a warning. The exception to this rule are the
//primitive types: usize, f32, etc

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    //ns = in = u64 = u64
    //the : is pre declarign the variable as a type that uses x data type
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // not that type aliases *dont* provide any extra type safety, because aliases are *not* new
    // types
    println!(
        "{} nanoseconds + {} inches + {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}

// the main use of aliases is to reduce boilerplate; for example the io::Result<T> is an alias fort
// the Result<T, io::Error> type
//
// see also Atrtributes: https://doc.rust-lang.org/rust-by-example/attribute.html
