//! A crate to easily create C-like enums resolving into values
//!
//!
//! This creates an public enum where every Number has an associated value of type NumberDescription
//! ```rust
//! use indexed_valued_enums::create_indexed_valued_enum;
//! use indexed_valued_enums::indexed_enum::Indexed;
//! use indexed_valued_enums::valued_enum::Valued;
//!
//! create_indexed_valued_enum! {
//!     enum Number,
//!     derives: [Eq, PartialEq, Debug],
//!     features: [Clone],
//!     value type: NumberDescription,
//!     Zero, NumberDescription { description: "Zero position", index: 0 },
//!     First, NumberDescription { description: "First position", index: 1 },
//!     Second, NumberDescription { description: "Second position", index: 2 },
//!     Third, NumberDescription { description: "Third position", index: 3 }
//! }
//!
//! #[derive(PartialEq)]
//! struct NumberDescription {
//!     description: &'static str,
//!     index: u16,
//! }
//!
//! #[test]
//! fn test() {
//!     assert_eq!(Number::Zero.discriminant(), 0);
//!     assert_eq!(Number::First.value().description, "First position");
//!     assert_eq!(Number::Second.clone(), Number::Second);
//!     assert_eq!(Number::Third, Number::value_to_variant(
//!         &NumberDescription { description: "Third position", index: 3 }));
//! }
//! ```
//!
//! For more details see the macro create_indexed_valued_enum


pub mod valued_enum;
pub mod indexed_enum;
pub mod macros;
#[cfg(feature = "serde_enums")]
pub mod serde_compatibility;