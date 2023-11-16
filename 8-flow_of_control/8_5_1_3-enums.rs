/*
 *  an enum is destructed similarly
 */

#[allow(dead_code)]

enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);

    println!("what color is it?");
    // an 'enum' can be destructred using a match
    match color {
        Color::Red => println!("the color is red!"),
        Color::Green => println!("the color is green!"),
        Color::Blue => println!("the color is blue!"),

        Color::RGB(r, g, b) => println!("r:{}, g:{}, b: {}", r, g, b),
        Color::HSV(h, s, v) => println!("h:{}, s:{}, v:{}", h, s, v),
        Color::HSL(h, s, l) => println!("h:{}, s:{}, l:{}", h, s, l),
        Color::CMY(c, m, y) => println!("c:{}, m:{}, y:{}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("c:{}, m:{}, y:{}, k:{}", c, m, y, k),
        //default not needed cause all options have been covered
    }
}
