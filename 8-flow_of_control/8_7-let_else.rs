/*
 *  with let-else, a refutable pattern can match and bind variables in the surrounding scope liek a
 *  normal let, or else diverge (eg, break, return, panic!) when a pattern doesn't match.
 */
use std::str::FromStr;

fn get_count_items(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(sount_str), Some(item)) - (it.next(). it.next()) else {
        panic!("Can't segment count item pair: '{s}'"); 
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("can't parse int: '{count_str}'");
    };
    (count, item)
}   

fn main() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

    /*
     *  the cope of name bnindings is the main thing that makes this different from match or if
     *  let-else expressions. you could preciously approximate these patterns with abn unfortunate
     *  bit of repetition and an outer let
     */

    let (count_str, item) = match (it.next(), it.next()) {
    (Some(count_str), Some(item)) => (count_str, item),
    _ => panic!("Can't segment count item pair: '{s}'"),
    };
    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("Can't parse integer: '{count_str}'");
    };

}   
