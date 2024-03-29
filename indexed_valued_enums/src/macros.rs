//The following uses are taken for documentation purposes
use crate::indexed_enum::Indexed;
use crate::valued_enum::Valued;

/// Produces an enum implementing the [Indexed] and [Valued] traits, meaning the enum's variants can
/// produce unique numbers of usize to identify each variant through [Indexed::discriminant], and
/// get back those variants through [Indexed::from_discriminant], and similar to it, each variant
/// has a value that can be taken from [Valued::value], where the variant can be taken back* from
/// that  value through [Valued::value_to_variant].
///
/// *Just if the value isn't repeated.
/// <br><br>
/// Being a macro by rules, you only need to follow this pattern:
///
/// create_indexed_valued_enum!{ <br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	**Your metadata** //Like '#[derive(...)]', this is optional <br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	**##**[features(**Feature1**, **Feature2**, ...)] // this is optional, but it needs **two** octothorpes<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	**Visibility** enum **Enum's name** values as **TypeOfValue**; <br><br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	***Variant1's metadata*** //this is optional<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	***Variant1***, ***Value1***,<br><br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	***Variant2's metadata*** //this is optional<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	***Variant2***, ***Value2***,<br><br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	...<br><br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	***VariantN's metadata*** //this is optional<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	***VariantN***, ***ValueN***<br>
/// }
///
/// <br>
///
/// On each of these fields you can indicate different parameters to change the implementation of the
/// enum:
///
/// * *EnumsName*: Name the enum will have.
/// * *TypeOfValue*: type of the values the variant's resolve to.
/// * Pairs of *Variant, Value*: Name of the variant's to create along to the name they resolve to,
///   the values must be const and have 'static lifetime.
/// * *Features*: List of specific implementations you want your enum to use, see the section
/// * *Features*: List of specific implementations you want your enum to use, you can find a list of
///               them in the documentation of [crate] -> Section: Extra features.
///
/// Note: You can write metadata (Such as #[derive(...)]) before each pair of *Variant, Value*, and
/// also before the enum, but it is required that the ##[features(...)] is the last of the
/// metadatas as this is not another metadata (hence the double hashtag to denote it)///
/// A simple example would look like:
///
/// ```rust
/// use indexed_valued_enums::create_indexed_valued_enum;
///
/// create_indexed_valued_enum! {
///     //Defines the enum and the value type it resolves to
///     pub enum MyOtherNumber valued as &'static str;
///     //Defines every variant and their value, note that values must be const
///     Zero, "Zero position",
///     First, "First position",
///     Second, "Second position",
///     Third,  "Third position"
/// }
/// ```
/// A more complex example would look like:
///
/// ```rust
/// use indexed_valued_enums::create_indexed_valued_enum;
///
/// create_indexed_valued_enum! {
///     #[doc="This is a custom enum that can get values of &'static str!"]
///     //This enum derives certain traits, although you don't need to write this
///     #[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
///     //Gives a list of features that are decomposed functions for specific behaviours, you have
///     //more details about them down below
///     ###[features(Clone, DerefToValue, Delegators, ValueToVariantDelegators)]
///     //Defines the enum and the value type it resolves to
///     pub enum MyOtherNumber valued as &'static str;
///     //Defines every variant and their value, note that values must be const
///     Zero, "Zero position",
///     First, "First position",
///     Second, "Second position",
///     Third,  "Third position"
/// }
/// ```
#[macro_export]
macro_rules! create_indexed_valued_enum {
    (
        $(#[$metadata:meta])*
        $(##[features($($features:tt),*)])?
        $visibility:vis enum $enum_name:ident valued as $value_type:ty;
        $($(#[$variants_metadata:meta])* $variants:ident, $values:expr
            $(;unnamed_field_initializers $($unnamed_field_initializers:expr),+)?
            $(;named_field_initializers $($named_field_name:ident $named_field_value:expr),+)?
        ),+ $(,)?
    ) => {
        $(#[$metadata])*
        #[repr(usize)]
        $visibility enum $enum_name{
            $($(#[$variants_metadata:meta])* $variants),+,
        }

        indexed_valued_enums::create_indexed_valued_enum !(impl traits $enum_name $value_type; $($variants, $values),+);

        $(indexed_valued_enums::create_indexed_valued_enum !{process features $enum_name, $value_type; $($features);* })?
    };
    (
        impl traits $enum_name:ident $value_type:ty; $($variants:ident, $values:expr
            $(;unnamed_field_initializers $($unnamed_field_initializers:expr),+ ;)?
            $(;named_field_initializers $($named_field_name:ident $(:)? $named_field_value:expr),+ ;)?
        ),+
    )=>{
        impl indexed_valued_enums::indexed_enum::Indexed for $enum_name {
            #[doc = concat!("Array storing all the variants of the [",stringify!($enum_name),"]\
            enum where each variant is stored in ordered by their discriminant")]
            const VARIANTS: &'static [ Self ] = &[$($enum_name::$variants
            $(( $($unnamed_field_initializers), +))?
            $({ $($named_field_name: $named_field_value), +})?

            ),+];
        }

        impl indexed_valued_enums::valued_enum::Valued for $enum_name {
            type Value = $value_type;

            #[doc = concat!("Array storing all the variants values of the \
             [",stringify!($enum_name),"] enum, each value is stored in the same order as the \
            discriminant of the variant they belong to")]
            const VALUES: &'static [ Self::Value] = & [$($values),+];
        }
    };
    (process features $enum_name:ident, $value_type:ty; $($features:tt);*)=>{
        $(indexed_valued_enums::create_indexed_valued_enum !{process feature $enum_name, $value_type; $features })*
    };
    (process feature $enum_name:ident, $value_type:ty; Delegators)
    =>{
        impl $enum_name {
            #[doc = concat!("Gets the discriminant of this",stringify!($enum_name),", this \
            operation is O(1)")]
            pub const fn discriminant(&self) -> usize {
                indexed_valued_enums::indexed_enum::discriminant_internal(self)
            }

            #[doc = concat!("Gets the",stringify!($enum_name),"'s variant corresponding to said \
            discriminant, this operation is O(1) as it just gets the discriminant as a copy from \
            [indexed_valued_enums::indexed_enum::Indexed::VARIANTS], meaning this enum does not \
            need to implement [Clone]")]
            pub const fn from_discriminant_opt(discriminant: usize) -> Option<Self> {
                indexed_valued_enums::indexed_enum::from_discriminant_opt_internal(discriminant)
            }

            #[doc = concat!("Gets the",stringify!($enum_name),"'s variant corresponding to said \
            discriminant, this operation is O(1) as it just gets the discriminant as a copy from \
            [indexed_valued_enums::indexed_enum::Indexed::VARIANTS], meaning this enum does not \
            need to implement [Clone]")]
            pub const fn from_discriminant(discriminant: usize) -> Self {
                indexed_valued_enums::indexed_enum::from_discriminant_internal(discriminant)
            }

            #[doc = concat!("Gives the value of type [",stringify!($value_type),"] corresponding \
            to this [", stringify!($enum_name),"] 's variant, this operation is O(1) as it just \
            gets the discriminant as a copy from \
            [indexed_valued_enums::valued_enum::Valued::VALUES] \
            If you just need a reference to the value, use \
            [",stringify!($enum_name),"::value_ref])] instead, as it doesn't require a read copy)
            <br><br>This always returns [Option::Some], so it's recommended to call\
            [",stringify!($enum_name),"::value] instead")]
            pub const fn value_opt(&self) -> Option<$value_type> {
                indexed_valued_enums::valued_enum::value_opt_internal(self)
            }

            #[doc = concat!("Gives the value of type [",stringify!($value_type),"] corresponding \
            to this [", stringify!($enum_name),"] 's variant, this operation is O(1) as it just \
            gets the discriminant as a copy from \
            [indexed_valued_enums::valued_enum::Valued::VALUES]. <br>\
            If you just need a reference to the value, use \
            [",stringify!($enum_name),"::value_ref])] instead, as it doesn't require a read copy")]
            pub const fn value(&self) -> $value_type {
                indexed_valued_enums::valued_enum::value_internal(self)
            }

            #[doc = concat!("Gives the value of type [",stringify!($value_type),"] corresponding \
            to this [", stringify!($enum_name),"] 's variant, if you need a copy of the value \
            but the value doesn't implement Clone, use [",stringify!($enum_name),"::value_opt]\
            instead, as it performs a read copy \
            <br><br>This always returns [Option::Some], so it's recommended to call\
            [",stringify!($enum_name),"::value] instead")]
            pub const fn value_ref_opt(&self) -> Option<&'static $value_type> {
                indexed_valued_enums::valued_enum::value_ref_opt_internal(self)
            }

            #[doc = concat!("Gives the value of type [",stringify!($value_type),"] corresponding \
            to this [", stringify!($enum_name),"] 's variant, if you need a copy of the value\
            but the value doesn't implement Clone, use [",stringify!($enum_name),"::value] \
            instead as it performs a read copy")]
            pub const fn value_ref(&self) -> &'static $value_type {
                indexed_valued_enums::valued_enum::value_ref_internal(self)
            }
        }
    };
    (process feature $enum_name:ident, $value_type:ty; ValueToVariantDelegators)
    =>{
        impl $enum_name {
            #[doc = concat!("Gives [",stringify!($enum_name),"]'s variant corresponding to this \
            value <br><br> this is an O(n) operation as it does so by comparing every single value \
            contained in [indexed_valued_enums::valued_enum::Valued::VALUES]")]
            pub fn value_to_variant_opt(value: &$value_type) -> Option<Self> {
                indexed_valued_enums::valued_enum::Valued::value_to_variant_opt(value)
            }

            #[doc = concat!("Gives [",stringify!($enum_name),"]'s variant corresponding to this \
            value <br><br> this is an O(n) operation as it does so by comparing every single value \
            contained in [indexed_valued_enums::valued_enum::Valued::VALUES]")]
            pub fn value_to_variant(value: &$value_type) -> Self {
                indexed_valued_enums::valued_enum::Valued::value_to_variant(value)
            }
        }
    };
    (process feature $enum_name:ident, $value_type:ty; DerefToValue)
    =>{
        impl core::ops::Deref for $enum_name{
            type Target = $value_type;

            #[doc = concat!("Gives the value of type [",stringify!($value_type),"] corresponding to \
            this [", stringify!($enum_name),"] 's variant <br><br>Since \
            [indexed_valued_enums::valued_enum::Valued::VALUES] is a constant array, the value will \
            be referenced for 'static")]
            fn deref(&self) -> &'static Self::Target {
                &<Self as indexed_valued_enums::valued_enum::Valued>::VALUES[self.discriminant()]
            }
        }
    };
    (process feature $enum_name:ident, $value_type:ty; Clone)
    =>{
        impl core::clone::Clone for $enum_name {

            #[doc = concat!("Clones this [",stringify!($enum_name),"]'s variant<br><br>This clone \
            is taken from the constant array of\
            [indexed_valued_enums::indexed_enum::Indexed::VARIANTS], meaning this is a copy of that \
            array, and therefore not causing a long macro expansion")]
            fn clone(&self) -> Self {
                let discriminant = indexed_valued_enums::indexed_enum::Indexed::discriminant(self);
                indexed_valued_enums::indexed_enum::Indexed::from_discriminant(discriminant)
            }
        }
    };
    (process feature $enum_name:ident, $value_type:ty; Serialize)
    =>{
        impl serde::Serialize for $enum_name {
            #[doc = concat!("Serializes this [",stringify!($enum_name),"]'s variant as it's \
            discriminant, reducing its serializing complexity")]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
                serializer.serialize_u128(self.discriminant() as u128)
            }
        }
    };
    (process feature $enum_name:ident, $value_type:ty; Deserialize)
    =>{
        impl<'de> serde::Deserialize<'de> for $enum_name {
            #[doc = concat!("Deserializes this [",stringify!($enum_name),"]'s variant from it's \
            discriminant, reducing its deserializing complexity")]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                match deserializer.deserialize_u128(indexed_valued_enums::serde_compatibility::discriminant_visitor::DISCRIMINANT_VISITOR) {
                    Ok(value) => {
                        $enum_name::from_discriminant_opt(value).ok_or_else(|| serde::de::Error::custom(
                            "Deserialized an discriminant that is bigger than the amount of variants",
                        ))
                    }
                    Err(error) => { Err(error) }
                }
            }
        }
    };
    (process feature $enum_name:ident, $value_type:ty; NanoSerBin)
    =>{
        impl nanoserde::SerBin for $enum_name {
            #[doc = concat!("Serializes this [",stringify!($enum_name),"]'s variant as it's \
            discriminant, reducing its serializing complexity")]
            fn ser_bin(&self, output: &mut Vec<u8>) {
                self.discriminant().ser_bin(output)
            }
        }
    };
    (process feature $enum_name:ident, $value_type:ty; NanoDeBin)
    =>{
        impl nanoserde::DeBin for $enum_name {
            #[doc = concat!("Deserializes this [",stringify!($enum_name),"]'s variant from it's \
            discriminant, reducing its deserializing complexity")]
            fn de_bin(offset: &mut usize, bytes: &[u8]) -> core::result::Result<Self, nanoserde::DeBinErr> {
                core::result::Result::Ok(
                    $enum_name::from_discriminant_opt(nanoserde::DeBin::de_bin(offset, bytes)?)
                        .ok_or_else(|| nanoserde::DeBinErr {
                            o: *offset,
                            l: core::mem::size_of::<usize>(),
                            s: bytes.len(),
                        })?)
            }
        }
    };
    (process feature $enum_name:ident, $value_type:ty; SerJson)
    =>{
        impl nanoserde::SerJson for $enum_name {
            #[doc = concat!("Serializes this [",stringify!($enum_name),"]'s variant as it's \
            discriminant, reducing its serializing complexity")]
            fn ser_json(&self, _d: usize, state: &mut nanoserde::SerJsonState) {
                state.out.push_str(&self.discriminant().to_string());
            }
        }
    };
    (process feature $enum_name:ident, $value_type:ty; NanoDeJson)
    =>{
        impl nanoserde::DeJson for $enum_name {
            #[doc = concat!("Deserializes this [",stringify!($enum_name),"]'s variant from it's \
            discriminant, reducing its deserializing complexity")]
            fn de_json(state: &mut nanoserde::DeJsonState, input: &mut core::str::Chars) -> Result<Self, nanoserde::DeJsonErr> {
                let val = state.u64_range(core::u64::MAX as u64)?;
                state.next_tok(input)?;
                let discriminant = val as usize;

                let variant = $enum_name::from_discriminant_opt(discriminant)
                    .ok_or_else(|| nanoserde::DeJsonErr{
                        msg: "Indicated discriminant doesn't not correspond to any variant of this enum".to_string(),
                        line: 0,
                        col: 0,
                    })?;
                return Ok(variant);
            }
        }
    };
}