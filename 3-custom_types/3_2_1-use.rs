// the 'use' declaration can be used so manual scoping isn't needed:

//an attribute to hide warnings for unused code
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // expicitly 'use' each nam so they are avaible without manual scoping
    use crate::Status::{Poor, Rich};
    // Automatically 'use' each name inside 'work'
    use crate::Work::*;

    //equivalent to 'Status::Poor'
    let status = Poor;
    //equivalent to 'Work::Civilian'
    let work = Civilian;

    match status {
        //note the lack of scoping because of the explicit 'use' above
        Rich => println!("the rich have lots of money!"),
        Poor => println!("the poor have no money..."),
    }

    match work {
        //note the lack of scoping
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

// see also:
// https://doc.rust-lang.org/rust-by-example/flow_control/match.html
// https://doc.rust-lang.org/rust-by-example/mod/use.html
