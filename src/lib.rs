#![no_std]
#![no_main]

//! A crate to easily create C-like enums resolving into values.
//!
//! ## Example of valued enum use
//! This creates a public enum where every Number has an associated value of type NumberDescription.
//! ```rust
//! use indexed_valued_enums::create_indexed_valued_enum;
//! use indexed_valued_enums::indexed_enum::Indexed;
//! use indexed_valued_enums::valued_enum::Valued;
//!
//! create_indexed_valued_enum! {
//!     #[derive(Eq, PartialEq, Debug)]
//!     #[features(Clone)]
//!     pub enum Number valued as NumberDescription;
//!     Zero, NumberDescription { description: "Zero position", index: 0 },
//!     First, NumberDescription { description: "First position", index: 1 },
//!     Second, NumberDescription { description: "Second position", index: 2 },
//!     Third, NumberDescription { description: "Third position", index: 3 }
//! }
//!
//! #[derive(PartialEq)]
//! pub struct NumberDescription {
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
//! ## Example of creating a valued enum
//! To implement it write:
//! <br><br>
//! create_indexed_valued_enum!{ <br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	#\[doc=**Documentation**] <br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	#[derive(**Derive1**, **Derive2**, ...)] <br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	#[features(**Feature1**, **Feature2**, ...)] <br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	**Visibility** enum **Enum's name** values as **TypeOfValue**; <br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	***Variant1***, ***Value1***,<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	***Variant2***, ***Value2***,<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	...<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	***VariantN***, ***ValueN***<br>
//! }
//!
//! As example:
//!
//! ```rust
//! use indexed_valued_enums::create_indexed_valued_enum;
//!
//! create_indexed_valued_enum! {
//!     #[doc="This is a custom enum that can get values of &'static str!"]
//!     //This enum derives certain traits, although you don't need to write this
//!     #[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
//!     //Gives a list of features that are decomposed functions for specific behaviours, you have
//!     //more details about them down below
//!     #[features(Clone, DerefToValue, Delegators, ValueToVariantDelegators,
//!                Serialize, Deserialize,
//!                NanoDeBin, NanoSerBin, NanoDeJson, NanoSerJson)]
//!     //Defines the enum and the value type it resolves to
//!     pub enum MyOtherNumber valued as &'static str;
//!     //Defines every variant and their value, note that values must constant and have 'static lifetime
//!     Zero, "Zero position",
//!     First, "First position",
//!     Second, "Second position",
//!     Third,  "Third position"
//! }
//! ```
//!
//! On each of the fields you can indicate different parameters to change the implementation of the
//! enum:
//!
//! * *Documentation*: Documentation of the enum.
//! * *Derives*: List of derive macros you want the enum to execute (Optional).
//! * *Visibility*: Visibility of the enum (Optional).
//! * *EnumsName*: Name the enum will have.
//! * *TypeOfValue*: type of the values the variant's resolve to.
//! * Pairs of *Variant, Value*: Name of the variant's to create along to the name they resolve to,
//!                              the values must be const and have 'static lifetime.
//! * *Features*: List of specific implementations you want your enum to use, they are the following ones:
//!     * DerefToValue: The enum implements Deref, making variants to resolve to their value
//!                     directly, remember however these values won't mutate as they are constant
//!                     references (&'static *TypeOfValue*), this is also the reason why these
//!                     values require their life-time to be 'static.
//!     * Clone: The enum implements clone calling [Indexed::from_discriminant], this way it's not
//!              required for the Derive Clone macro to expand to large enums.
//!     * Delegators: Implements delegator functions over this enum that call on the methods from
//!                  [Indexed] and [Valued], this way it is not required to import or use the
//!                  indexed_valued_enums crate directly, however, it doesn't delegate the methods
//!                  [Valued::value_to_variant] and [Valued::value_to_variant_opt] as they
//!                  require the type of value to implement [PartialEq], however, you can delegate
//!                  these too with the feature **ValueToVariantDelegators**.
//!     * ValueToVariantDelegators: Implements delegator functions for [Valued::value_to_variant]
//!                                 and [Valued::value_to_variant_opt].
//!     * Serialize: Implements serde's Serialize trait where it serializes to an usize that
//!                  represents this enum's discriminant. <br>
//!                  Requires the "serde_enums" feature.
//!     * Deserialize: Implements serde's Deserialize trait where it deserializes an enum variant's
//!                    from it's enum's discriminant. <br>
//!                    Requires the "serde_enums" feature.
//!     * NanoSerBin: Implements nanoserde's SerBin trait where it serializes to an usize that
//!                   represents this enum's discriminant.
//!     * NanoDeBin: Implements nanoserde's DeBin trait where it deserializes an enum variant's
//!                  from it's enum's discriminant.
//!     * NanoSerJson: Implements nanoserde's SerJson trait where it serializes to an usize that
//!                   represents this enum's discriminant.
//!     * NanoDeJson: Implements nanoserde's DeJson trait where it deserializes an enum variant's
//!                  from it's enum's discriminant.

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

//The following uses are taken for documentation purposes
use crate::indexed_enum::Indexed;
use crate::valued_enum::Valued;