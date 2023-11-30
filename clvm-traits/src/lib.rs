//! # CLVM Traits
//! This is a library for encoding and decoding Rust values using a CLVM allocator.
//! It provides implementations for every fixed-width signed and unsigned integer type,
//! as well as many other values in the standard library that would be common to encode.
//!
//! As well as the built-in implementations, this library exposes two derive macros
//! for implementing the `ToClvm` and `FromClvm` traits on structs and enums. They be
//! marked with the following representations:
//!
//! * `#[clvm(tuple)]` for unterminated lists such as `(A . (B . C))`.
//! * `#[clvm(list)]` for proper lists such as `(A B C)`, or in other words `(A . (B . (C . ())))`.
//! * `#[clvm(curry)]` for curried arguments such as `(c (q . A) (c (q . B) (c (q . C) 1)))`.
//!
//! Additionally, you can use `#[clvm(untagged)]` on enums which don't have a numeric discriminant value.
//!
//! The `#[repr(int_type)]` attribute can be used to specify the discriminant type for enums.

#![cfg_attr(
    feature = "derive",
    doc = r#"
## Derive Example

```rust
use clvmr::Allocator;
use clvm_traits::{ToClvm, FromClvm};

#[derive(Debug, PartialEq, Eq, ToClvm, FromClvm)]
#[clvm(tuple)]
struct Point {
    x: i32,
    y: i32,
}

let a = &mut Allocator::new();

let point = Point { x: 5, y: 2 };
let ptr = point.to_clvm(a).unwrap();

assert_eq!(Point::from_clvm(a, ptr).unwrap(), point);
```
"#
)]

#[cfg(feature = "derive")]
pub use clvm_derive::*;

mod clvm_decoder;
mod clvm_encoder;
mod error;
mod from_clvm;
mod macros;
mod match_byte;
mod to_clvm;
mod wrappers;

pub use clvm_decoder::*;
pub use clvm_encoder::*;
pub use error::*;
pub use from_clvm::*;
pub use match_byte::*;
pub use to_clvm::*;
pub use wrappers::*;

#[cfg(test)]
#[cfg(feature = "derive")]
mod tests {
    extern crate self as clvm_traits;

    use std::fmt;

    use clvmr::{allocator::NodePtr, serde::node_to_bytes, Allocator};

    use super::*;

    fn check<T>(value: T, expected: &str)
    where
        T: fmt::Debug + PartialEq + ToClvm<NodePtr> + FromClvm<NodePtr>,
    {
        let a = &mut Allocator::new();

        let ptr = value.to_clvm(a).unwrap();
        let round_trip = T::from_clvm(a, ptr).unwrap();
        assert_eq!(value, round_trip);

        let bytes = node_to_bytes(a, ptr).unwrap();
        let actual = hex::encode(bytes);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tuple() {
        #[derive(Debug, ToClvm, FromClvm, PartialEq, Eq)]
        #[clvm(tuple)]
        struct TupleStruct {
            a: u64,
            b: i32,
        }

        check(TupleStruct { a: 52, b: -32 }, "ff3481e0");
    }

    #[test]
    fn test_list() {
        #[derive(Debug, ToClvm, FromClvm, PartialEq, Eq)]
        #[clvm(list)]
        struct ListStruct {
            a: u64,
            b: i32,
        }

        check(ListStruct { a: 52, b: -32 }, "ff34ff81e080");
    }

    #[test]
    fn test_curry() {
        #[derive(Debug, ToClvm, FromClvm, PartialEq, Eq)]
        #[clvm(curry)]
        struct CurryStruct {
            a: u64,
            b: i32,
        }

        check(
            CurryStruct { a: 52, b: -32 },
            "ff04ffff0134ffff04ffff0181e0ff018080",
        );
    }

    #[test]
    fn test_unnamed() {
        #[derive(Debug, ToClvm, FromClvm, PartialEq, Eq)]
        #[clvm(tuple)]
        struct UnnamedStruct(String, String);

        check(UnnamedStruct("A".to_string(), "B".to_string()), "ff4142");
    }

    #[test]
    fn test_newtype() {
        #[derive(Debug, ToClvm, FromClvm, PartialEq, Eq)]
        #[clvm(tuple)]
        struct NewTypeStruct(String);

        check(NewTypeStruct("XYZ".to_string()), "8358595a");
    }

    #[test]
    fn test_enum() {
        #[derive(Debug, ToClvm, FromClvm, PartialEq, Eq)]
        #[clvm(tuple)]
        enum Enum {
            A(i32),
            B { x: i32 },
            C,
        }

        check(Enum::A(32), "ff8020");
        check(Enum::B { x: -72 }, "ff0181b8");
        check(Enum::C, "ff0280");
    }

    #[test]
    fn test_explicit_enum() {
        #[derive(Debug, ToClvm, FromClvm, PartialEq, Eq)]
        #[clvm(tuple)]
        #[repr(u8)]
        enum Enum {
            A(i32) = 42,
            B { x: i32 } = 34,
            C = 11,
        }

        check(Enum::A(32), "ff2a20");
        check(Enum::B { x: -72 }, "ff2281b8");
        check(Enum::C, "ff0b80");
    }

    #[test]
    fn test_untagged_enum() {
        #[derive(Debug, ToClvm, FromClvm, PartialEq, Eq)]
        #[clvm(tuple, untagged)]
        enum Enum {
            A(i32),

            #[clvm(list)]
            B {
                x: i32,
                y: i32,
            },

            #[clvm(curry)]
            C {
                curried_value: String,
            },
        }

        check(Enum::A(32), "20");
        check(Enum::B { x: -72, y: 94 }, "ff81b8ff5e80");
        check(
            Enum::C {
                curried_value: "Hello".to_string(),
            },
            "ff04ffff018548656c6c6fff0180",
        );
    }
}
