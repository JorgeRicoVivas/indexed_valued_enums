use crate::indexed_enum::Indexed;
use crate::valued_enum::Valued;

/// Produces an enum implementing the [Indexed] and [Valued] traits, meaning the enum's variants can
/// produce unique numbers of usize to identify each variant through [Indexed::discriminant], and
/// get back those variants through [Indexed::from_discriminant], and similar to it, each variant
/// has a value that can be taken from [Valued::value], where the variant can be taken back* from
/// that  value through [Valued::value_to_variant]
///
/// *Just if the value isn't repeated
/// <br><br>
/// To implement it write:
/// <br><br>
/// create_indexed_valued_enum!{ <br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	***Visibility*** enum ***EnumsName***, <br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	derives: [***Derive1***, ***Derive2***, ...], <br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	features: [***Feature1***, ***Feature2***, ...], <br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	value type: ***TypeOfValue***, <br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	***Variant1***, ***Value1***,<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	***Variant2***, ***Value2***,<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	...<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;	***VariantN***, ***ValueN***<br>
/// }
///
/// such as:
/// ```rust
/// use indexed_valued_enums::create_indexed_valued_enum;
///
/// create_indexed_valued_enum! {
///     enum Number,
///     derives: [Eq, PartialEq, Debug],
///     features: [],
///     value type: &'static str,
///     Zero, "Zero position",
///     First, "First position",
///     Second, "Second position",
///     Third,  "Third position"
/// }
/// ```
///
/// On each of the fields you can indicate different parameters to change the implementation of the
/// enum:
///
/// * *Visibility*: Visibility of the enum
/// * *EnumsName*: Name the enum will have
/// * *Derives*: List of derive macros you want the enum to execute
/// * *TypeOfValue*: type of the values the variant's resolve to
/// * Pairs of *Variant, Value*: Name of the variant's to create along to the name they resolve to
/// * *Features*: List of specific implementations you want your enum to use, they are the following ones:
///     * DerefToValue: The enum implements Deref, making variants to resolve to their value
///                     directly, remember however these values won't mutate as they are constant
///                     references (&'static *TypeOfValue*), this is also the reason why these
///                     values require their life-time to be 'static
///     * Clone: The enum implements clone calling [Indexed::from_discriminant], this way it's not
///              required for the Derive Clone macro to expand to large enums
///     * Delegators: Implements delegator functions over this enum that call on the methods from
///                  [Indexed] and [Valued], this way it is not required to import or use the
///                  indexed_valued_enums crate directly, however, it doesn't delegate the methods
///                  [Valued::value_to_variant] and [Valued::value_to_variant_opt] as they
///                  require the type of value to implement [PartialEq], however, you can delegate
///                  these too with the feature **ValueToVariantDelegators**
///     * ValueToVariantDelegators: Implements delegator functions for [Valued::value_to_variant]
///                                 and [Valued::value_to_variant_opt]
///     * Serialize: Implements serde's Serialize trait where it serializes to an usize that
///                  represents this enum's discriminant
///     * Deserialize: Implements serde's Deserialize trait where it deserializes an enum variant's
///                    from it's enum's discriminant
///     * NanoSerBin: Implements nanoserde's SerBin trait where it serializes to an usize that
///                   represents this enum's discriminant
///     * NanoDeBin: Implements nanoserde's DeBin trait where it deserializes an enum variant's
///                  from it's enum's discriminant
///     * NanoSerJson: Implements nanoserde's SerJson trait where it serializes to an usize that
///                   represents this enum's discriminant
///     * NanoDeJson: Implements nanoserde's DeJson trait where it deserializes an enum variant's
///                  from it's enum's discriminant
#[macro_export]
macro_rules! create_indexed_valued_enum {
    (process features
        [$enum_name:ident, $value_type:ty],
        [Delegators $($other_features:tt)*]
    )=>{
        impl $enum_name {
            #[doc = concat!("Gets the discriminant of this",stringify!($enum_name),", this \
            operation is O(1)")]
            pub fn discriminant(&self) -> usize {
                indexed_valued_enums::indexed_enum::Indexed::discriminant(self)
            }

            #[doc = concat!("Gets the",stringify!($enum_name),"'s variant corresponding to said \
            discriminant, this operation is O(1) as it just gets the discriminant as a copy from \
            [indexed_valued_enums::indexed_enum::Indexed::VARIANTS], meaning this enum doesn't \
            need to implement [Clone]")]
            pub fn from_discriminant_opt(discriminant: usize) -> Option<Self> {
                indexed_valued_enums::indexed_enum::Indexed::from_discriminant_opt(discriminant)
            }

            #[doc = concat!("Gets the",stringify!($enum_name),"'s variant corresponding to said \
            discriminant, this operation is O(1) as it just gets the discriminant as a copy from \
            [indexed_valued_enums::indexed_enum::Indexed::VARIANTS], meaning this enum doesn't \
            need to implement [Clone]")]
            pub fn from_discriminant(discriminant: usize) -> Self {
                indexed_valued_enums::indexed_enum::Indexed::from_discriminant(discriminant)
            }

            #[doc = concat!("Gives the value of type",stringify!($value_type),"corresponding to \
            this", stringify!($enum_name),"'s variant<br><br>This value is always Some(",
            stringify!($value_type),"), so it's recommended to call\
             [",stringify!($enum_name),"::value] instead")]
            pub fn value_opt(&self) -> Option<$value_type> {
                indexed_valued_enums::valued_enum::Valued::value_opt(self)
            }

            #[doc = concat!("Gives the value of type",stringify!($value_type),"corresponding to \
            this", stringify!($enum_name),"'s variant")]
            pub fn value(&self) -> $value_type {
                indexed_valued_enums::valued_enum::Valued::value(self)
            }
        }

        create_indexed_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
    };
        (process features
        [$enum_name:ident, $value_type:ty],
        [ValueToVariantDelegators $($other_features:tt)*]
    )=>{
        impl $enum_name {
            #[doc = concat!()]
            pub fn value_to_variant_opt(value: &$value_type) -> Option<Self> { indexed_valued_enums::valued_enum::Valued::value_to_variant_opt(value) }

            pub fn value_to_variant(value: &$value_type) -> Self { indexed_valued_enums::valued_enum::Valued::value_to_variant(value) }
        }

        create_indexed_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
    };
    (process features
        [$enum_name:ident, $value_type:ty],
        [DerefToValue $($other_features:tt)*]
    )=>{
        impl core::ops::Deref for $enum_name{
            type Target = $value_type;

            fn deref(&self) -> &Self::Target {
                &<Self as indexed_valued_enums::valued_enum::Valued>::VALUES[self.discriminant()]
            }
        }

        create_indexed_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
    };
    (process features
        [$enum_name:ident, $value_type:ty],
        [Clone $($other_features:tt)*]
    )=>{
        impl core::clone::Clone for $enum_name {
            fn clone(&self) -> Self {
                let discriminant = indexed_valued_enums::indexed_enum::Indexed::discriminant(self);
                indexed_valued_enums::indexed_enum::Indexed::from_discriminant(discriminant)
            }
        }

        create_indexed_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
    };
    (process features
        [$enum_name:ident, $value_type:ty],
        [Serialize $($other_features:tt)*]
    )=>{
        impl serde::Serialize for $enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
                serializer.serialize_u128(self.discriminant() as u128)
            }
        }

        create_indexed_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
    };
    (process features
        [$enum_name:ident, $value_type:ty],
        [Deserialize $($other_features:tt)*]
    )=>{
        impl<'de> serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                match deserializer.deserialize_u128(indexed_valued_enums::serde_compatibility::discriminant_visitor::DISCRIMINANT_VISITOR) {
                    Ok(value) => {
                        match $enum_name::from_discriminant_opt(value) {
                            Some(value) => { Ok(value) }
                            None => { Err(serde::de::Error::custom("Deserialized an discriminant that is bigger than the amount of variants")) }
                        }
                    }
                    Err(error) => { Err(error) }
                }
            }
        }

        create_indexed_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
    };
    (process features
        [$enum_name:ident, $value_type:ty],
        [NanoSerBin $($other_features:tt)*]
    )=>{
        impl nanoserde::SerBin for $enum_name {
            fn ser_bin(&self, output: &mut Vec<u8>) {
                self.discriminant().ser_bin(output)
            }
        }

        create_indexed_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
    };
    (process features
        [$enum_name:ident, $value_type:ty],
        [NanoDeBin $($other_features:tt)*]
    )=>{
        impl nanoserde::DeBin for $enum_name {
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

        create_indexed_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
    };

    (process features
        [$enum_name:ident, $value_type:ty],
        [NanoSerJson $($other_features:tt)*]
    )=>{
        impl nanoserde::SerJson for $enum_name {
            fn ser_json(&self, _d: usize, state: &mut nanoserde::SerJsonState) {
                state.out.push_str(&self.discriminant().to_string());
            }
        }

        create_indexed_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
    };
    (process features
        [$enum_name:ident, $value_type:ty],
        [NanoDeJson $($other_features:tt)*]
    )=>{
        impl nanoserde::DeJson for $enum_name {
            fn de_json(state: &mut nanoserde::DeJsonState, input: &mut Chars) -> Result<Self, nanoserde::DeJsonErr> {
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

        create_indexed_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
    };

    (process features [$enum_name:ident, $value_type:ty], [])=>{};

    (
        $visibility:vis enum $enum_name:ident,
        derives: [$($derives:ident),*],
        features: [$($features:tt),*],
        value type: $value_type:ty,
        $($variants:ident, $values:expr),+
    ) => {
        #[derive($($derives),*)]
        #[repr(usize)]
        $visibility enum $enum_name{
            $($variants),+
        }

        impl indexed_valued_enums::indexed_enum::Indexed for $enum_name {
            const VARIANTS: &'static [ Self ] = &[$($enum_name::$variants),+];
        }

        impl indexed_valued_enums::valued_enum::Valued for $enum_name {
            type Value = $value_type;
            const VALUES: &'static [ Self::Value] = & [$($values),+];
        }

        create_indexed_valued_enum !{process features [$enum_name, $value_type], [$($features)*] }
    };
}