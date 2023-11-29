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
//!     #[derive(Eq, PartialEq, Debug)]
//!     #[features(Clone)]
//!     enum Number valued as NumberDescription;
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
//! For more details see the macro [crate::create_indexed_valued_enum] to see details and examples
//! on how creating valued enums, it also offers simplified methods to implement serialize and
//! deserialize for the Serde and Nanoserde crates


/// Defines a trait to associate values to an enum
pub mod valued_enum;

/// Defines a trait index enums over an array using it's discriminant
pub mod indexed_enum;

/// Defines the main macro used to expand a list of values to an enum where each variant has an
/// associated values
pub mod macros;

/// Adds compatibility with Serde, this requires indicating the feature 'serde_enums' when adding
/// this library to your Cargo.toml, like
/// ```toml
/// indexed_valued_enums = { version = "0.8.0", features=["serde_enums"] }
/// ```
#[cfg(feature = "serde_enums")]
pub mod serde_compatibility;