pub mod many;
pub mod many1;
pub mod sep_by;

use super::*;

#[test]
fn test_many() {
    let number = digit().many1(|iter| Some(iter.fold(0, |n, d| 10 * n + d)));
    let mut input = "123abc";
    assert_eq!(number.parse(&mut input), Some(123));
    assert_eq!(input, "abc");
}

#[test]
fn test_many_mut() {
    let mut stack = vec![2, 3, 5, 7, 11];
    let parser = from_fn_mut(|_| stack.pop()).collect_many();
    let mut input = "";
    assert_eq!(parser.parse_once(&mut input), Some(vec![11, 7, 5, 3, 2]));
    assert_eq!(input, "");
}
