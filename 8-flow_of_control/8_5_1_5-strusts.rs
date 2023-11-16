/*
 *  similarly, a struct can be destructed as shown:
 */

fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    //if oyu want, try changin the values in the struct to see what happens // i'm good thx
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };

    //you do not need a match block to destructure structs:
    let Foo { x: x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");
}
