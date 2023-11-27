#[macro_export]
macro_rules! create_indexed_valued_enum {
    (process features
        [$enum_name:ident, $value_type:ty],
        [Delegators $($other_features:tt)*]
    )=>{
        impl $enum_name {
            pub fn index(&self) -> usize { indexed_valued_enums::indexed_enum::Indexed::index(self) }

            pub fn from_index_opt(index: usize) -> Option<Self> { indexed_valued_enums::indexed_enum::Indexed::from_index_opt(index) }

            pub fn from_index(index: usize) -> Self { indexed_valued_enums::indexed_enum::Indexed::from_index(index) }

            pub fn value_opt(&self) -> Option<$value_type> { indexed_valued_enums::valued_enum::Valued::value_opt(self) }

            pub fn value(&self) -> $value_type { indexed_valued_enums::valued_enum::Valued::value(self) }

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
                &<Self as indexed_valued_enums::valued_enum::Valued>::VALUES[self.index()]
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
                serializer.serialize_u128(self.index() as u128)
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
                        match $enum_name::from_index_opt(value) {
                            Some(value) => { Ok(value) }
                            None => { Err(serde::de::Error::custom("Deserialized an index that is bigger than the amount of variants")) }
                        }
                    }
                    Err(error) => { Err(error) }
                }
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