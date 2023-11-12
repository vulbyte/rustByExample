/* use std::fmt; 
 * 
 * // define a struct named 'list' containing a 'vec'
 * struct List(Vec<i32>);
 * 
 * impl fmt::Display for List {
 *     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 *         //extract the value using tuple indexing, 
 *         //then create a reference to 'vec'
 *         let vec = &self.0;
 * 
 *         write!(f, "[")?;
 * 
 *         // iterate over 'v' in 'vec' wile enumerating the iteration count
 *         // in 'count'
 *         for (count, v) in vec.iter().enumerate() {
 *             // for every lement except the first, add a comma.
 *             // use the ? operator to return on any errors
 *             if count != 0 { write!(f, ", ")?; }
 *             write!(f, "{}", v)?;
 *         }
 * 
 *         //close the opened bracket and return a fmt::Result value
 *         write!(f, "]")
 *     }
 * }
 * 
 * fn main() {
 *     // write!(f, "{}", value);
 *     let v = List(vec![1, 2, 3, ]);
 *     println!("{}", v);
 * }
 */

//assignment: turn the above functioning code so that the output is:
//[0: 1, 1: 2, 2: 3]

//above to me looks like starting at the value of 0, add one and print, making it look like a map

use std::fmt; 

// define a struct named 'list' containing a 'vec'
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //extract the value using tuple indexing, 
        //then create a reference to 'vec'
        let vec = &self.0;

        write!(f, "[")?;

        // iterate over 'v' in 'vec' wile enumerating the iteration count
        // in 'count'
        for (count, v) in vec.iter().enumerate() {
            // for every lement except the first, add a comma.
            // use the ? operator to return on any errors
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}:{}", count, v)?; //change here, added count and switched ot using {} 
        }

        //close the opened bracket and return a fmt::Result value
        write!(f, "]")
    }
}

fn main() {
    // write!(f, "{}", value);
    let v = List(vec![1, 2, 3, ]);
    println!("{}", v);
}
