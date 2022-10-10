#[macro_use]
extern crate newstr;

use serde::{Deserialize, Serialize};

is_valid_newstring!(Foo, |_| true; Deserialize, Serialize);

from_str_newstring!(Boo, |s: &str| Ok(s.to_string()); Deserialize, Serialize);

#[test]
fn check_is_valid() {
    use std::str::FromStr;
    let _ = Foo::from_str("hello");
}

#[test]
fn check_from_str() {
    use std::str::FromStr;
    let _ = Boo::from_str("hello");
}
