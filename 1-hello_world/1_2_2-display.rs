//optionally import with use <lib>::<name>;, ie use std::fmt
use std::fmt;

fn main(){
    //define an int32 
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            //write structly the first elem in
            //steam: 'f' returns 'fmt::Result' which indicates success or fail
            write!(f, "{}", self.0)
        }
    }
}
