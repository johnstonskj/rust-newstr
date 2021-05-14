# Crate newstr

Simple macros for declaring String-base new types.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
[![crates.io](https://img.shields.io/crates/v/newstr.svg)](https://crates.io/crates/newstr)
[![docs.rs](https://docs.rs/newstr/badge.svg)](https://docs.rs/newstr)
![Build](https://github.com/johnstonskj/rust-newstr/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-newstr/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-newstr.svg)](https://github.com/johnstonskj/rust-newstr/stargazers)

This crate provides simple macros that generate String based *new types*. The two primary macros
implement the validity rules for the new type by either 1) providing a predicate that is used by
the `is_valid` associated function, or 2) providing a function to parse and return a string which
is then called by `FromStr::from_str`.

Both of these methods produce a new type, with the following:

1. An associated predicate function `is_valid` that returns `true` if the string provided would be a
   valid value for the type.
1. This type derives implementations of `Clone`, `Debug`, `PartialEq`, `PartialOrd`, `Eq`, `Ord`,
   and `Hash`.
1. An implementation of `Display` for `T` that simply returns the inner value.
1. An implementation of `From<T>` for `String`.
1. An implementation of `Deref` for `T` with the target type `str`.
1. An implementation of `FromStr`.

Additional user-required traits can also be added to the macro to be derived by the implementation.

## Example

The following example constructs a new string type that implements an `Identifier` value. This
value must be ASCII, alphanumeric, the '_' character and must not be empty.

```rust
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::ops::Deref;

fn is_identifier_value(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

is_valid_newstring!(Identifier, is_identifier_value);

assert!(!Identifier::is_valid(""));
assert!(!Identifier::is_valid("hi!"));
assert!(!Identifier::is_valid("hello world"));
assert!(!Identifier::is_valid("9.99"));

assert_eq!(
    Identifier::from_str("hi").unwrap().to_string(),
    String::from("hi")
);
assert_eq!(
    Identifier::from_str("hello_world").unwrap().to_string(),
    String::from("hello_world")
);
```

## Dependencies

In the example above you can see the necessary use-statements for the trait implementations the
macros generate. Unless you use `regex_is_valid` there are no crate dependencies; if you do you will
need to add `lazy_static` and `regex` dependencies.

If the macros in this crate take on addition dependencies or provide new implementations the set of
use statements may change which will break consumer builds. To avoid this another macro,
`use_required`, will add any required use statements the consumer requires.

```rust
# use newstr::{is_valid_newstring, use_required};
use_required!();

is_valid_newstring!(NotEmpty, |s: &str| !s.is_empty());
```

## Changes

**Version 0.1.2**

* Added additional, optional, parameter to both macros that allows additional traits to be added to the `derive`
 attribute on the generated struct.
* Refactored the macros to reduce repetition.
* Added some additional tests.

**Version 0.1.1**

* Added new `use_required` macro.
* Removed unnecessary feature 'regex_is_valid', the build doesn't require this to avoid bloat.
* Made `lazy_static` and `regex` dev dependencies, if you don't use them, you don't need them.
* Added dependency on `cargo-husky` for Git cleanliness.

**Version 0.1.0**

* Initial version.