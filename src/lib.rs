//! A crate to easily create C-like enums resolving into values
//!
//!
//! This creates an public enum where every Number has an associated value of type NumberDescription
//! ```rust
//! indexed_valued_enums::create_indexed_valued_enum! {
//!     pub enum Number,
//!     derives: [Eq, PartialEq, Debug],
//!     features: [DerefToValue, Delegators, Clone],
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
//! ```
//! [valued_enum::Valued]
//!
//! For more details see the macro [indexed_valued_enums::macros::create_indexed_valued_enum!]
//!

pub mod valued_enum;
pub mod indexed_enum;
pub mod macros;
#[cfg(feature = "serde_enums")]
pub mod serde_compatibility;