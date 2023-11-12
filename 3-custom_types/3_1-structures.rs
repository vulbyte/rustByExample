//there are 3 types of structs ("structs") that can be created
//tuple structs, which are basically named tuples
//the classic c structs (https://en.wikipedia.org/wiki/Struct_(C_programming_language))
//unit structs, which are field-less, are useful for generics

//an attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8, // predefining the size
}

// a unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// a tuple struct
struct Point {
    x: f32,
    y: f32,
}

//structs can be reused as fields of another struct
struct Rectangle {
    //a rectangle can be specified by where the top-left, and botton-right cornders are in space
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Point { x: x1, y: y1 } = self.top_left;
        let Point { x: x2, y: y2 } = self.bottom_right;

        let width = x2 - x1;
        let height = y2 - y1;
        width * height
    }
}

fn main() {
    //create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    //print debug struct
    println!("{:?}", peter);

    // instantiate a "point"
    let point: Point = Point { x: 10.3, y: 0.4 };

    // access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // make a new point by using struct update syntax to use the fields of your other one
    let bottom_right = Point { x: 5.2, ..point };

    // 'bottom_right.y' will be the same as 'point.y' because we used that field from point
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // destructre the point using a 'let' binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // structure instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // instantiate a unit struct
    let _unit = Unit;

    // instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

// Activity

//    Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
//    Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32.
