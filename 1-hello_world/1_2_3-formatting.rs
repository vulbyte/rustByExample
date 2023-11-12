use std::fmt::{self, Display, Formatter};

//is this a class?
struct City {
    name: &'static str,
    //latitude
    lat: f32,
    //long
    lon: f32,
}

impl Display for City {
    //'f' is a biffer, and this method lmust write the formatted string
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        //determine n/e/s/w
        let lat_c = if self.lat >= 0.0 { "N" } else { "S" };
        let lon_c = if self.lat >= 0.0 { "E" } else { "W" };

        //'write!' is like 'format!', but it will write the format into a buffer on the first
        //arguemnt
        //the ':.3' is a percision identifier, it specifies how many decimal places"
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c,
        ) //<================ colon
          //here rust might hate, just
          //testing
    }
}

//
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ] {
        println! {"{}", city};
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        //switch to use {} once you've added the implemtation for fmt::Display
        println!(
            "RGB (R:{}, G:{}, B:{}) 0x{:0>6x}",
            color.red,
            color.green,
            color.blue,
            (color.red as u32 * 65536 + color.green as u32 * 256 + color.blue as u32)
        );
    }
}

//ACTIVITY:
//Add an implementation of the fmt::Display trait for the Color struct above so that the output displays as:

// RGB (128, 255, 90) 0x80FF5A
// RGB (0, 3, 254) 0x0003FE
// RGB (0, 0, 0) 0x000000

// note1: i added a prefix to each color just to make it extra clear cause i prefer the way it looks
// (:

// note2: in if you want to combine a function, it's akin to the ab syntax, where ab = a*b
// the line on 83 basically is saying: should be 6 chars long, if there's not that replace that
// space with upto 6 '0's before the print
// think of it like < means after, > means before
