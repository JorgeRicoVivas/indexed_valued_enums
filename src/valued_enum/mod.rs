use crate::indexed_enum::Indexed;

pub trait Valued: Indexed {
    type Value: PartialEq;

    const VALUES: &'static [Self::Value];

    fn value_opt(&self) -> Option<Self::Value> {
        let (first_offset, second_offset) = Self::split_discriminants_to_offsets(self.discriminant());
        Some(unsafe { Self::VALUES.as_ptr().offset(first_offset).offset(second_offset).read() })
    }

    fn value(&self) -> Self::Value {
        self.value_opt().unwrap()
    }

    fn value_to_variant_opt(value: &Self::Value) -> Option<Self> {
        let discriminant = Self::VALUES.iter()
            .enumerate()
            .filter(|(_, variant_value)| value.eq(variant_value)).next()
            .map(|(index, _)| index);
        Self::from_discriminant_opt(discriminant?)
    }

    fn value_to_variant(value: &Self::Value) -> Self {
        Self::value_to_variant_opt(value).unwrap()
    }
}