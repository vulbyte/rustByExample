/*
 *  similar to from and into, tryfrom and tryinto are generic traits fro converting between types.
 *  unline from/into, the tryfrom/trointo traits are usef for fallible conversions, and such return
 *  Result's
 */

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto with 8
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    println!(
        "the size of the 8 input is: {:?}, and the byte size is: {:?}",
        result,
        std::mem::size_of_val(&result)
    );

    //TryInto with 5
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
    println!(
        "the size of the 5 input is: {:?}, and the byte size is: {:?}",
        result,
        std::mem::size_of_val(&result)
    );

    /*
     *  tldr from the TryInto comment to here, is that result will return "Ok(EvenNumber(8))" if
     *  even, while an odd number will return Err(())"
     *
     *  i guess this is to pass around conditional logic?
     */
}
