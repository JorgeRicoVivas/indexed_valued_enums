//! A crate to easily create C-like enums resolving into values

pub mod valued_enum;
pub mod indexed_enum;
pub mod macros;
#[cfg(feature = "serde_enums")]
pub mod serde_compatibility;