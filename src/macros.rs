#[macro_export]
macro_rules! indexed_and_valued_enum {
    (process features
        [$enum_name:ident, $value_type:ty],
        [DerefToValue $($other_features:tt)*]
    )=>{
        impl Deref for $enum_name{
            type Target = $value_type;

            fn deref(&self) -> &Self::Target {
                &Self::VALUES[self.index()]
            }
        }

        indexed_and_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
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

        indexed_and_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
    };
    (process features
        [$enum_name:ident, $value_type:ty],
        [Deserialize $($other_features:tt)*]
    )=>{
        impl<'de> serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                match deserializer.deserialize_u128(DISCRIMINANT_VISITOR) {
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


        indexed_and_valued_enum !{process features [$enum_name, $value_type], [$($other_features)*]}
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

        impl Indexed for $enum_name {
            const VARIANTS: &'static [ Self ] = &[$($enum_name::$variants),+];
        }

        impl Valued for $enum_name {
            type Value = $value_type;
            const VALUES: &'static [ Self::Value] = & [$($values),+];
        }

        indexed_and_valued_enum !{process features [$enum_name, $value_type], [$($features)*] }
    };
}