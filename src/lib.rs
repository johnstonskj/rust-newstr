/*!
This crate provides simple macros that generate String based *new types*. The two primary macros
implement the validity rules for the new type by either 1) providing a predicate that is used by
the `is_valid` associated function, or 2) providing a function to parse and return a string which
is then called by `FromStr::from_str`.

Both of these methods produce a new struct, with the following:

1. An associated predicate function `is_valid` that returns `true` if the string provided would be a
   valid value for the type.
1. This type derives implementations of `Clone`, `Debug`, `PartialEq`, `PartialOrd`, `Eq`, `Ord`,
   and `Hash`.
1. An implementation of `Display` for `T` that simply returns the inner value.
1. An implementation of `From<T>` for `String`.
1. An implementation of `Deref` for `T` with the target type `str`.
1. An implementation of `FromStr`.

Additional user-required traits can also be added to the macro to be derived by the implementation.

# Example

The following example constructs a new string type with the macro
[`is_valid_newstring`](macro.is_valid_newstring.html) that implements an `Identifier` value. This
value must be ASCII, alphanumeric, the '_' character and must not be empty.

```rust
# use newstr::is_valid_newstring;
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

# Dependencies

In the example above you can see the necessary use-statements for the trait implementations the
macros generate. Unless you use `regex_is_valid` there are no crate dependencies; if you do you will
need to add `lazy_static` and `regex` dependencies.

If the macros in this crate take on addition dependencies or provide new implementations the set of
use statements may change which will break consumer builds. To avoid this another macro,
[`use_required`](macro.use_required.html), will add any required use statements the consumer requires.

```rust
# use newstr::{is_valid_newstring, use_required};
use_required!();

is_valid_newstring!(NotEmpty, |s: &str| !s.is_empty());
```

*/

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    // ---------- Unused
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
#[macro_export]
macro_rules! standard_struct {
    ($new_name:ident, $( $other:ident ),*) => {
        #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, $($other),*)]
        pub struct $new_name(String);
    };
    ($new_name:ident serde) => {
        #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
        pub struct $new_name(String);
    };
    ($new_name:ident) => {
        #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
        pub struct $new_name(String);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! standard_impls {
    ($new_name:ident) => {
        impl Display for $new_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<$new_name> for String {
            fn from(v: $new_name) -> Self {
                v.0
            }
        }

        impl Deref for $new_name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! is_valid_inner {
    ($new_name:ident, $closure:expr) => {
        standard_impls! { $new_name }

        impl FromStr for $new_name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if Self::is_valid(s) {
                    Ok(Self(s.to_string()))
                } else {
                    Err(())
                }
            }
        }

        impl $new_name {
            pub fn is_valid(s: &str) -> bool {
                $closure(s)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! from_str_inner {
    ($new_name:ident, $closure:expr, $error:ty) => {
        standard_impls! { $new_name }

        impl FromStr for $new_name {
            type Err = $error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $closure(s).map(|s| Self(s))
            }
        }

        impl $new_name {
            pub fn is_valid(s: &str) -> bool {
                Self::from_str(s).is_ok()
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

///
/// This macro expands to the set of use statements required by a consumer of either
/// [`is_valid_newstring`](macro.is_valid_newstring.html) or
/// [`from_str_newstring`](macro.from_str_newstring.html) macros. This can be valuable over time
/// if the implementationof these  macros take on additional dependencies.
///
/// The macro takes a single, optional, parameter `regex` which will also include the necessary
/// dependencies used by the [`regex_is_valid`](macro.regex_is_valid.html) macro.
///
/// # Examples
///
/// ```rust
/// # use newstr::{is_valid_newstring, use_required};
/// use_required!();
///
/// is_valid_newstring!(NotEmpty, |s: &str| !s.is_empty());
/// ```
///
/// ```rust
/// # use newstr::{regex_is_valid, use_required};
/// use_required!(regex);
///
/// regex_is_valid!(r"[0-9]+", is_valid_integer);
/// ```
///
#[macro_export]
macro_rules! use_required {
    () => {
        use std::fmt::{Display, Formatter};
        use std::ops::Deref;
        use std::str::FromStr;
    };
    (regex) => {
        use lazy_static::lazy_static;
        use regex::Regex;
        use_required!()
    };
}

///
/// This macro takes a new type identifier and a predicate function to produce a new type. The
/// predicate is called by `T::is_valid` and is then used in the implementation of `FromStr` to
/// determine whether to return a new instance or error. As this is simply a boolean value and does
/// not differentiate between reasons for invalidity the error type for `FromStr` is always `()`.
///
/// An optional variadic parameter also allows other trait names to be specified which will be
/// added to the list of traits in the `derive` attribute.
///
/// # Examples
///
/// Create a new string type with a user-defined closure.
///
/// ```rust
/// # use newstr::is_valid_newstring;
/// use std::fmt::{Display, Formatter};
/// use std::str::FromStr;
/// use std::ops::Deref;
///
/// is_valid_newstring!(NotEmpty, |s: &str| !s.is_empty());
///
/// assert!(!NotEmpty::is_valid(""));
/// assert!(NotEmpty::from_str("").is_err());
///
/// assert!(NotEmpty::is_valid("hi"));
/// assert!(NotEmpty::from_str("hi").is_ok());
/// assert_eq!(NotEmpty::from_str("hi").unwrap().len(), 2);
/// ```
///
/// The following creates a new string type using an existing function.
///
/// ```rust
/// # use newstr::is_valid_newstring;
/// # use std::fmt::{Display, Formatter};
/// # use std::str::FromStr;
/// # use std::ops::Deref;
/// is_valid_newstring!(AsciiStr, str::is_ascii);
/// ```
///
/// In the following our new string type also derives serde attributes for serialization.
///
/// ```rust
/// # use newstr::{is_valid_newstring, use_required};
/// use_required!();
/// use serde::{Deserialize, Serialize};
///
/// is_valid_newstring!(NotEmpty, |s: &str| !s.is_empty(), Deserialize, Serialize);
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! is_valid_newstring {
    ($new_name:ident, $closure:expr, $( $other:ident ),*) => {
        standard_struct! { $new_name, $($other),* }

        is_valid_inner! { $new_name, $closure }
    };
    ($new_name:ident, $closure:expr) => {
        standard_struct! { $new_name }

        is_valid_inner! { $new_name, $closure }
    };
}

///
/// This macro takes a string that contains a regular expression will construct a new validity
/// predicate that may be used by the [`is_valid_newstring`](macro.is_valid_newstring.html) macro.
/// An optional second parameter provides a name for the new predicate function, overriding the
/// default `is_valid`.
///
/// The generated function uses `lazy_static` to only compile the regular expression once. You
/// will require a dependency on both the `lazy_static` and `regex` crates, as you see in the
/// example below.
///
/// # Example
///
/// ```rust
/// # use newstr::regex_is_valid;
/// use lazy_static::lazy_static;
/// use regex::Regex;
/// use std::str::FromStr;
///
/// regex_is_valid!(r"[0-9]+", is_valid_integer);
/// ```
///
#[macro_export]
macro_rules! regex_is_valid {
    ($regex:expr) => {
        regex_is_valid! { $regex, is_valid }
    };
    ($regex:expr, $fn_name:ident) => {
        fn $fn_name(s: &str) -> bool {
            lazy_static! {
                static ref VALID_VALUE: Regex = Regex::from_str($regex).unwrap();
            }
            VALID_VALUE.is_match(s)
        }
    };
}

///
/// This macro takes a new type identifier and a *parse function* to produce a new type. The parse
/// function **must** take the form `fn(&str) -> Result<String, Err>`, this is called from within
/// the `FromStr::from_str` function to actually parse the string, and in doing so potentially
/// returning an altered form if necessary.
///
/// In this macro the implementation of `T::is_valid` calls `FromStr::from_str` to perform the
/// validity check.
///
/// An optional parameter for this macro allows the implementation to override the default error
/// type, `()`, used in the implementation of `FromStr` allowing more detail to be provided on the
/// validation failure.
///
/// # Examples
///
/// This creates a new string type which only allows for uppercase characters.
///
/// ```rust
/// # use newstr::from_str_newstring;
/// use std::fmt::{Display, Formatter};
/// use std::str::FromStr;
/// use std::ops::Deref;
///
/// fn parse_uppercase_only(s: &str) -> Result<String, ()> {
///     if s.chars().all(|c|c.is_uppercase()) {
///         Ok(s.to_string())
///     } else {
///         Err(())
///     }
/// }
///
/// from_str_newstring!(OnlyUpperCase, parse_uppercase_only);
///
/// assert!(!OnlyUpperCase::is_valid("hello"));
/// assert!(OnlyUpperCase::from_str("hello").is_err());
///
/// assert!(OnlyUpperCase::is_valid("HELLO"));
/// assert!(OnlyUpperCase::from_str("HELLO").is_ok());
/// assert_eq!(OnlyUpperCase::from_str("HELLO").unwrap().to_string(), String::from("HELLO"));
/// ```
///
/// In the following our new string type also derives serde attributes for serialization.
///
/// ```rust
/// # use newstr::{from_str_newstring, use_required};
/// use_required!();
/// use serde::{Deserialize, Serialize};
///
/// fn parse_uppercase_only(s: &str) -> Result<String, ()> {
///     if s.chars().all(|c|c.is_uppercase()) {
///         Ok(s.to_string())
///     } else {
///         Err(())
///     }
/// }
///
/// from_str_newstring!(OnlyUpperCase, parse_uppercase_only, Deserialize, Serialize);
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! from_str_newstring {
    ($new_name:ident, $closure:expr) => {
        from_str_newstring! { $new_name, $closure, () }
    };
    ($new_name:ident, $closure:expr, $( $other:ident ),*) => {
        from_str_newstring! { $new_name, $closure, (), $($other),* }
    };
    ($new_name:ident, $closure:expr, $error:ty) => {
        standard_struct! { $new_name }

        from_str_inner! { $new_name, $closure, $error }
    };
    ($new_name:ident, $closure:expr, $error:ty, $( $other:ident ),*) => {
        standard_struct! { $new_name, $($other),* }

        from_str_inner! { $new_name, $closure, $error }
    };
}
