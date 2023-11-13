use crate::List::*;

enum List {
    //cons: tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    //Nil: A node that signified the end of the linked list
    Nil,
}

// methods can be attached to an enum
impl List {
    //create an empt list
    fn new() -> List {
        //Nil has a type list
        Nil
    }

    //consume  a list and return the same list with a new element at it's front
    fn prepend(self, elem: u32) -> List {
        //'cons' also has a type list
        Cons(elem, Box::new(self))
    }

    //return the lendth of the list
    fn len(&self) -> u32 {
        //self has to be matched because the behavior depends on teh vairant of self
        //self has type &list, and *self has type list, matching on a concrete type 'T' is prefer
        //over match a ref '&T'.
        //after rust 2018 you can use self here and tail (with no ref), below we will infer &s and
        //ref tail.
        //see https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            //can't take ownership of the tail, because 'self ' is borrowed;
            //// instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            //Base Case: an empty list has 0 length
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // format! is similar to print! but returns a heap allocated string instead of
                // printing to the console.
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    //create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    //show the finale state of the list
    println!("linked list has a length: {}", list.len());
    println!("{}", list.stringify());
}

// see also:
// box: https://doc.rust-lang.org/rust-by-example/std/box.html
// methods: https://doc.rust-lang.org/rust-by-example/fn/methods.html
