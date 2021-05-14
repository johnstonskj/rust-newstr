#[macro_use]
extern crate newstr;

use_required!();

#[derive(Clone, Copy)]
struct Boo {}

impl Display for Boo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Boo!")
    }
}

#[test]
fn check_for_from_str() {
    let _: u8 = u8::from_str("16").unwrap();
}

#[test]
fn check_for_deref() {
    let _: &str = String::from("hello").deref();
}
