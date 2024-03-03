#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

//! [![crates.io](https://img.shields.io/crates/v/indexed_valued_enums.svg)](https://crates.io/crates/indexed_valued_enums)
//! [![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JorgeRicoVivas/indexed_valued_enums/rust.yml)](https://github.com/JorgeRicoVivas/indexed_valued_enums/actions)
//! [![docs.rs](https://img.shields.io/docsrs/indexed_valued_enums)](https://docs.rs/indexed_valued_enums/latest/indexed_valued_enums/)
//! [![GitHub License](https://img.shields.io/github/license/JorgeRicoVivas/indexed_valued_enums)](https://github.com/JorgeRicoVivas/indexed_valued_enums?tab=CC0-1.0-1-ov-file)
//!
//! Create enums resolving into values and get their variants back through their values or their
//! discriminant, inspired by Java's enums.
//!
//! 1 [Motivation and use](#1-motivation-and-use)<br>
//! 2 [Creating a valued enum](#2a1-introductory-example-of-valued-enum-use-via-the-declarative-macro)<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;2.a Via the declarative macro<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;2.a.1 [Introductory example of valued enum use via the declarative macro](#2a1-introductory-example-of-valued-enum-use-via-the-declarative-macro)<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;2.a.2 [How to use the declarative macro](#2a2-how-to-use-the-declarative-macro)<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;2.a.3 [Other examples for the declarative macro](#2a3-other-examples-for-the-declarative-macro)<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;2.a Via the derive macro<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;2.b.1 [Introductory example of valued enum use via the Derive macro](#2b1-introductory-example-of-valued-enum-use-via-the-derive-macro)<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;2.b.2 [How to use the Derive macro](#2b2-how-to-use-the-derive-macro)<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;2.b.3 [Other examples for the derive macro](#2b3-other-examples-for-the-derive-macro)<br>
//! 3 [Extra features](#3-extra-features)<br>
//! 4 [Assumptions this crate does](#4-assumptions-this-crate-does)<br>
//!
//! ## 1 Motivation and use
//! In a few programming languages it is possible to create enums and associate some information
//! at compile time, for example, Java or C# allow to get a variants identifier, and said variant
//! out of that identifier, it also allows applying a constructor to them, making easy to associate
//! constant values to each variant, allowing to define enums like this one:
//!
//! ```java
//! public enum Planet {
//!     Earth(6357.0, 9.807), Mars(3389.5, 3.71), Mercury(2439.7, 3.7);
//!
//!     private Double radius;
//!     private Double gravity;
//!
//!     Planet(Double radius, Double gravity) {
//!         this.radius = radius;
//!         this.gravity = gravity;
//!     }
//!
//!     public Double getRadius() {
//!         return radius;
//!     }
//!
//!     public Double getGravity() {
//!         return gravity;
//!     }
//! }
//! ```
//! <br>
//! To replicate those mechanics two trais have been created:
//!
//! * [Indexed] allows you to get a discriminant / index of said variant through the
//! function 'discriminant' and get this variant back using the function 'from_discriminant'.
//! <br><br>
//! In the example below, Planet::Mars gives discriminant 1, and the
//! discriminant 1 would give Planet::Mars Back.<br><br><br>
//! * [Valued] allows you to associate values to discriminants, giving a function
//! 'value' to return the associated constant with the variant, and 'value_to_variant_opt' to get a
//! possible variant whose constant matches said value.<br><br>
//! In the example below, Planet::Earth gives a value of CelestialBody{ radius: 6357.0,
//! gravity: 9.807 }, and said value would return Planet::Earth back.<br>
//!
//!
//! ```rust
//! use indexed_valued_enums::{Valued, enum_valued_as};
//!
//! #[derive(PartialEq)]
//! pub struct CelestialBody {
//!     radius: f32,
//!     gravity: f32,
//! }
//!
//! #[derive(PartialEq, Debug, Valued)]
//! #[enum_valued_as(CelestialBody)]
//! #[enum_valued_features(DerefToValue, Delegators, ValueToVariantDelegators)]
//! enum Planet {
//!     #[value(CelestialBody{ radius: 6357.0, gravity: 9.807 })]
//!     Earth,
//!     #[value(CelestialBody{ radius: 3389.5, gravity: 3.71 })]
//!     Mars,
//!     #[value(CelestialBody{ radius: 2439.7, gravity: 3.7 })]
//!     Mercury,
//! }
//!
//! #[test]
//! fn example_test(){
//!     //Identifiers mechanics
//!     assert_eq!(Planet::Mars, Planet::from_discriminant(1));
//!     assert_eq!(Planet::Mercury.discriminant(), 2);
//!
//!     //Value mechanics
//!     assert_eq!(Planet::Earth.value().radius, 6357.0);
//!     assert_eq!(Planet::Mars.gravity, 3.71);
//!     assert_eq!(Planet::Mercury, Planet::value_to_variant(&CelestialBody{ radius: 2439.7, gravity: 3.7 }));
//! }
//! ```
//!
//! You can implement this on your enums using one of two macros:
//! * [The declarative macro](#2a1-introductory-example-of-valued-enum-use-via-the-declarative-macro):
//! On this one you write every variant along it's value, being really easy to write and read, and
//! especially useful when creating simple enums without a lot of manipulation, be them short or
//! large however, in case where you need to directly manipulate your enum, it can be quite
//! restrictive and it doesn't support variants with fields, be them named or unnamed, if you find
//! yourself in any of these two scenarios, use the derive macro instead.
//! <br><br>
//! * [The Derive macro](#2b1-introductory-example-of-valued-enum-use-via-the-derive-macro): On this
//! one you only need to add a few attributes to your enum and your variants indicating the values,
//! leaving you to fully control your enum as you please, however, too many variants might produce
//! hard to read code, in these cases, they are usually large enum without any fields, being a
//! perfect scenario for the declarative macro instead. It requires you to add the 'derive' feature
//! on your Cargo.toml, like
//! ```indexed_valued_enums = { version =  "1.0.0", features=["derive", ...] }```.
//!
//! ## 2.a.1 Introductory example of valued enum use via the declarative macro
//! This creates a public enum where every Number has an associated value of type NumberDescription,
//! just like in the introductory Derive example.
//!
//! ```rust
//! use indexed_valued_enums::create_indexed_valued_enum;
//! use indexed_valued_enums::indexed_enum::Indexed;
//! use indexed_valued_enums::valued_enum::Valued;
//!
//! create_indexed_valued_enum! {
//!     #[derive(Eq, PartialEq, Debug)]
//!     //The double octothorpe is intentional
//!     ###[features(Clone)]
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
//! ## 2.a.2 How to use the declarative macro
//!
//! Being a macro by rules, you only need to follow this pattern:
//!
//! create_indexed_valued_enum!{ <br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	**Your metadata** //Like '#[derive(...)]', this is optional <br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	**##**[features(**Feature1**, **Feature2**, ...)] // this is optional, but it needs **two** octothorpes<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	**Visibility** enum **Enum's name** values as **TypeOfValue**; <br><br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	***Variant1's metadata*** //this is optional<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	***Variant1***, ***Value1***,<br><br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	***Variant2's metadata*** //this is optional<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	***Variant2***, ***Value2***,<br><br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	...<br><br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	***VariantN's metadata*** //this is optional<br>
//! &nbsp;&nbsp;&nbsp;&nbsp;	***VariantN***, ***ValueN***<br>
//! }
//!
//! <br>
//!
//! On each of these fields you can indicate different parameters to change the implementation of
//! the enum:
//!
//! * *EnumsName*: Name the enum will have.
//! * *TypeOfValue*: type of the values the variant's resolve to.
//! * Pairs of *Variant, Value*: Name of the variant's to create along to the name they resolve to,
//!   the values must be const and have 'static lifetime.
//! * *Features*: List of specific implementations you want your enum to use, see the section
//!   [extra features](#3-extra-features) for more information about this.
//!
//! Note: You can write metadata (Such as #[derive(...)]) before each pair of *Variant, Value*, and
//! also before the enum, but it is required that the ##[features(...)] is the last of the enum's 
//! declaration metadatas as this is not another metadata (hence the double octothorpe to denote
//! it).
//! <br>
//!
//! ## 2.a.3 Other examples for the declarative macro
//! A simple example could look like:
//!
//! ```rust
//! use indexed_valued_enums::create_indexed_valued_enum;
//!
//! create_indexed_valued_enum! {
//!     //Defines the enum and the value type it resolves to
//!     pub enum MyOtherNumber valued as &'static str;
//!     //Defines every variant and their value, note that values must be const
//!     Zero, "Zero position",
//!     First, "First position",
//!     Second, "Second position",
//!     Third,  "Third position"
//! }
//! ```
//! A more complex example could look like:
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
//!     ###[features(Clone, DerefToValue, Delegators, ValueToVariantDelegators)]
//!     //Defines the enum and the value type it resolves to
//!     pub enum MyOtherNumber valued as &'static str;
//!     //Defines every variant and their value, note that values must be const
//!     Zero, "Zero position",
//!     First, "First position",
//!     Second, "Second position",
//!     Third,  "Third position"
//! }
//! ```
//!
//! ## 2.b.1 Introductory example of valued enum use via the Derive macro
//! This creates a public enum where every Number has an associated value of type NumberDescription,
//! just like in the declarative macro example.
//!
//! ```rust
//! use indexed_valued_enums::{enum_valued_as, Valued};
//!
//! #[derive(Eq, PartialEq, Debug, Valued)]
//! #[enum_valued_as(NumberDescription)]
//! pub enum Number{
//!     #[value(NumberDescription { description: "Zero position", index: 0 })]
//!     Zero,
//!     #[value(NumberDescription { description: "First position", index: 1 })]
//!     First,
//!     #[value(NumberDescription { description: "Second position", index: 2 })]
//!     Second,
//!     #[value(NumberDescription { description: "Third position", index: 3 })]
//!     Third,
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
//!
//! ## 2.b.2 How to use the Derive macro
//!
//! **IMPORTANT**: To use it, the 'derive' feature should be indicated on your Cargo.toml, like
//! ```indexed_valued_enums = { version =  "1.0.0", features=["derive", ...] }```.
//!
//! **Basic implementation**: Add the derive macro [indexed_valued_enums_derive::Valued] and then
//! write the #[enum_valued_as(*Value type*)] attribute indicating the type your variants will
//! resolve to, then on each variant write an attribute #[value(*this variants value*)]. this way:
//! <br><br>
//!
//! ```rust
//! use indexed_valued_enums::{Valued, enum_valued_as};
//!
//! #[derive(Valued)]
//! #[enum_valued_as(u8)]
//! pub enum MyEnum{
//!     #[value(10)]
//!     Variant1,
//!     #[value(20)]
//!     Variant2,
//! }
//! ```
//! <br>
//!
//! **Add extra functionality**: Below the Derive declaration you can write the attribute
//! #[enum_valued_features(*Your desired features*)] which will automatically implement certain
//! traits or functions which will become helpful, you can check these features on the section
//! [extra features](#3-extra-features).<br>
//!
//! ```rust ignore
//! ...
//! /// Adding 'Delegators' allows to call most of functions at
//! /// compile-time, being able to get values and variants easily
//! #[enum_valued_features(DerefToValue, Delegators)]
//! pub enum MyEnum{
//!     ...
//! }
//! ```
//! <br>
//!
//! **Don't repeat yourself**: For variants whose variants values are often repeated or irrelevant
//! you can use the attribute #[unvalued_default(*Your default value*)] which will make all these
//! unvalued variants to resolve into said value.<br>
//!
//! ```rust ignore
//! ...
//! #[unvalued_default(50)]
//! pub enum MyEnum{
//!     /// This variant's value will resolve to 10 as it is specified.
//!     #[value(10)]
//!     Variant1,
//!     /// This variant's value will resolve to the default of 50 as a value it is not specified.
//!     Variant2,
//! }
//! ```
//! <br>
//!
//! **Variant's with fields can be added too!** Unlike the declarative macro, this one is compatible
//! with variants with fields, be them named or unnamed, but they have a downside: since the 
//! [Indexed::from_discriminant] function must return a constant value for each variant, we also
//! need to create those variants with values at compile, when this situation arises you have two 
//! options:
//!
//! * Use the #[variant_initialize_uses(*Your default value*)] attribute: Here you write the default
//! contents for these variants, for example, if one was ```IP{host: &'static str, port: u16}```,
//! you could write: #[variant_initialize_uses(host: "localhost", port: 8080)].<br><br>
//! * If the values of the variant implement [const_default::ConstDefault]: You can simply add
//! const-default in your Cargo.toml like ```const-default = { version =  "1.0.0" }``` and when this
//! variant gets resolved from [Indexed::from_discriminant], it will return it with all fields as
//! specified in [const_default::ConstDefault].
//!
//! ```rust ignore
//! ...
//! pub enum MyEnum{
//!     /// When applying [from::discriminant] to 0, it will return MyEnum::Variant1(23, "Jorge")
//!     #[variant_initialize_uses(23, "Jorge")]
//!     Variant1(u8, &'static str),
//!     /// Since the attribute #[variant_initialize_uses] isn't specified, when applying
//!     /// [from::discriminant] to 1, it will return MyEnum::Variant2{age: 0}, as ConstDefault
//!     /// for u8 returns 0 
//!     Variant2{age:u8},
//! }
//! ```
//! <br>
//!
//! ## 2.b.3 Other examples for the derive macro
//! A simple example could look like this:
//!
//! ```rust
//! use indexed_valued_enums::{Valued, enum_valued_as};
//!
//! #[derive(Valued)]
//! #[enum_valued_as(&'static str)]
//! pub enum Number{
//!     #[value("Zero position")]
//!     Zero,
//!     #[value("First position")]
//!     First,
//!     #[value("Second position")]
//!     Second,
//!     #[value("Third position")]
//!     Third,
//! }
//! ```
//!
//! A more complex example could look like:
//!
//! ```rust
//! use indexed_valued_enums::{Valued, enum_valued_as};
//!
//! #[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
//! #[derive(Valued)]
//! #[enum_valued_as(&'static str)]
//! #[enum_valued_features(Clone, DerefToValue, Delegators, ValueToVariantDelegators)]
//! #[unvalued_default("My default string")]
//! pub enum Number{
//!     /// Zero doesn't have a value, so it's value will resolve to "My default string"
//!     Zero,
//!     #[value("First position")]
//!     First,
//!     /// Second is a variant with fields: u8 and u16, since it's not specified, when calling
//!     /// [Indexed::from_discriminant] the values for both will be 0, which are their default
//!     /// values on [const_default::ConstDefault::DEFAULT]
//!     #[value("Second position")]
//!     Second(u8, u16),
//!     /// Third is a variant with fields: my_age: u8 and my_name:&'static str, as specified,
//!     /// calling [Indexed::from_discriminant] will result in those fields contanining
//!     /// my_age: 23, my_name: "Jorge"
//!     #[variant_initialize_uses(my_age: 23, my_name: "Jorge")]
//!     #[value("Third position")]
//!     Third{my_age: u8, my_name:&'static str},
//! }
//! ```
//!
//! ## 3 Extra features
//!
//! * **DerefToValue**: Implements Deref, dereferencing each variant to a static reference of their
//! value.<br><br>
//! * **Clone**: Implements clone calling 'from_discriminant', avoiding large expansions of the
//! Derive Clone, this however won't clone the fields of your variants if there are some, being
//! rather ideal in the case of large field-less enums.<br>Since it calls 'discriminant' and then
//! 'from_discriminant', this operation is O(1). <br><br>
//! * **Delegators**: Implements **const functions** equivalent to methods from [Indexed] and
//! [Valued], like 'value(&self)' or 'from_discriminant(&self)', note that these delegator functions
//! are not the same as the ones inside the [Indexed] and [Valued] traits, as these delegators
//! **are const** functions.<br>
//! Note it doesn't delegate the methods 'value_to_variant' and 'value_to_variant_opt' as they
//! require the type of value to implement [PartialEq], you can delegate these too with the feature
//! **ValueToVariantDelegators**, but these delegator functions are **not const**.<br><br>
//! * **ValueToVariantDelegators**: Implements delegator functions calling to
//! [Valued::value_to_variant] and [Valued::value_to_variant_opt].<br><br>
//! * De/Serialization features: These allow to serialize and deserialize this enum as just it's
//! discriminant value, this is useful when your enum consists on variants without fields.
//! <br><br>
//! The features **Serialize** and **Deserialize** match the Serialize and DeserializeOwned traits,
//! of serde, to use this, you must add the feature serde_enums on Cargo.toml, like:
//! ``` indexed_valued_enums = { version = "1.0.0", features=["serde_enums"] } ``` <br><br>
//! The features **NanoSerBin**, **NanoDeBin**, **NanoSerJson** and **NanoDeJson** implements the
//! nanoserde's traits SerBin, DeBin, SerJson and DeJson respectively.<br><br>
//! **IMPORTANT**: When using these De/Serialization, it will try to implement them over **your**
//! dependencies, this means indexed_valued_enums won't directly depend on Serde or NanoSerde when
//! implementing these interfaces, so if you want to use the De/Serialization methods of
//! nanoserde, then nanoserde must be a dependency on your Cargo.toml, thanks to this, you always
//! have control over which version of Serde and NanoSerde is being applied.
//!
//!
//! ## 4 Assumptions this crate does
//!
//! * You won't rename this crate's name or any of those used in the
//! [extra features](#3-extra-features), this is because when expanding macros, it will try to
//! target **your** dependencies, by doing this, you avoid longer compile times when this crate and
//! yours use different versions, the dependencies you might need would be: ```serde```,
//! ```nanoserde```, and ```const-default```.<br><br>
//! * The variants of your enum don't have their discriminant manually set-up, this is because
//! values to these variants are stored in an array, where each value is stored in the index
//! corresponding to their variant's position and therefore discriminant, meaning the discriminant
//! as an index.<br><br>
//! * The enums are attributed with #[repr(usize)], you don't need to do this manually, the
//! declarative macro does it by itself, and when using the attribute
//! '#[enum_valued_as(*Your type*)]' it silently adds #[repr(usize)], but if you were to use cargo
//! expand and use the original code, the #[repr(usize)] attribute must remain.<br><br>


#[cfg(feature = "derive")]
extern crate indexed_valued_enums_derive;

#[cfg(feature = "derive")]
pub use indexed_valued_enums_derive::*;

//The following uses are taken for documentation purposes
use crate::indexed_enum::Indexed;
use crate::valued_enum::Valued;

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
#[cfg(feature = "serde")]
pub mod serde_compatibility;

