#[macro_use]
extern crate newstr;

use std::ops::Deref;
use std::str::FromStr;

is_valid_newstring!(AsciiStr, str::is_ascii);
new_unchecked!(pub AsciiStr);

#[test]
fn check_for_from_str() {
    let results = AsciiStr::from_str("hello").unwrap();
    assert_eq!(results, AsciiStr::new_unchecked("hello"));
}

#[test]
fn check_for_as_ref() {
    let results = AsciiStr::from_str("hello").unwrap();
    assert_eq!(results.as_ref(), "hello");
}

#[test]
fn check_for_deref() {
    let results = AsciiStr::from_str("hello").unwrap();
    assert_eq!(results.deref(), "hello");
}
