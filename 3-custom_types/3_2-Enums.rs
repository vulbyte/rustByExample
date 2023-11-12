// the enum keyboard alows the creation of a type which may be one of a few figgerent vairants, any
// variant which is valid as a struct is also a valid enum

// create an enum to classify a web event.
// not how both names and type information together specify the variant:
// 'PageLoad != PageUnload' and 'KeyPress(char) != Paste(String)'.
// each is different and independant

//an attribute to hide warnings for unused code.
#![allow(dead_code)]
// an attribute to allow unused vars
#![allow(unused)]

//==============================// THINGS FOR ENUMS
enum WebEvent {
    // an 'enum' variant may either e 'unit like',
    PageLoad,
    PageUnload,
    //like tuple structs,
    KeyPress(char),
    Paste(String),
    //or c-like structures
    Click { x: i64, y: i64 },
}

//a functino which takes a 'WebEvent' enum as an argument and returns nothing
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // destructure 'c' from inside the 'enum' variant
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s), //demo to how escaping
        // destructure 'click' into 'x' and 'y'
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

//==============================// THINGS FOR ALIASES

enum VeryVerboseEnumOfThingToDoWithNumbers {
    Add,
    Subtract,
}

// creates a type alias
type Operations = VeryVerboseEnumOfThingToDoWithNumbers;

// the most common place you'll see this in impl blocks using the Self alias
impl VeryVerboseEnumOfThingToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

//==============================// PUT IT ALL TOGETHER
fn main() {
    let pressed = WebEvent::KeyPress('x');
    // 'to_owned()' creates an owned 'String' from a string slice
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // we can refer to each variant via its alias, not it's long and inconvenient name
    let x = Operations::Add;
}
