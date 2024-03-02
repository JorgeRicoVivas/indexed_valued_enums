use crate::indexed_enum::{discriminant_internal, from_discriminant_opt_internal, Indexed, split_usize_to_isizes};

/// Allows to get a value from an enum's variant, where this enum implements [Indexed], for example,
/// having the following implementation:
///
/// ```rust
/// use indexed_valued_enums::{indexed_enum::Indexed, valued_enum::Valued};
///
///
/// enum Number{ First, Second, Third }
///
/// impl Indexed for Number{
///     const VARIANTS: &'static [Self] = &[Number::First, Number::Second, Number::Third];
/// }
///
/// impl Valued for Number{
///     type Value = u16;
///     const VALUES: &'static [Self::Value] = &[100,200,300];
/// }
/// ```
/// Calling [Valued::value] on every enum produces [First->100, Second->200, Third->300]
///
/// Since the type of the values (u16) implements PartialEq, we can also call
/// [Valued::value_to_variant] to get the variants corresponding to the values
/// [100->First, 200->Second, 300->Third]
///
/// Note this documentation it's solely informational, it is dis-recommended to implement this trait
/// manually, but using the derive macro [crate::Valued] or the declarative macro
/// [crate::create_indexed_valued_enum] instead.
pub trait Valued: Indexed {

    /// Type of the values the enumeration resolves to
    type Value;

    /// Values each enumeration resolves to, each value must be stored to match it's corresponding
    /// variant, this means it must be sorted in the same order as [Indexed::VARIANTS]
    ///
    /// This means values must be const
    const VALUES: &'static [Self::Value];

    /// Gives the value corresponding to this variant, this is an O(1) operation as it just gets the
    /// value as a copy from [Valued::VALUES]
    ///
    /// The type of [Valued::Value] doesn't need to implement the [Clone] trait as the array is
    /// treated as a raw pointer whose value is read without cloning through
    /// [core::ptr::read]
    ///
    /// Note that if implemented correctly (ensured by using [crate::create_indexed_valued_enum]),
    /// calling this method will always produce [Option::Some(Value)]
    fn value_opt(&self) -> Option<Self::Value> {
        value_opt_internal(self)
    }

    /// Gives the value corresponding to this variant, this is an O(1) operation as it just gets the
    /// value as a copy from [Valued::VALUES]
    fn value(&self) -> Self::Value {
        self.value_opt().unwrap()
    }

    /// Gives variant corresponding to a value, this is an O(n) operation as it does so by comparing
    /// every single value contained in [Valued::VALUES]
    fn value_to_variant_opt(value: &Self::Value) -> Option<Self> where Self::Value:PartialEq {
        let discriminant = Self::VALUES.iter()
            .enumerate()
            .filter(|(_, variant_value)| value.eq(variant_value)).next()
            .map(|(discriminant, _)| discriminant);
        Self::from_discriminant_opt(discriminant?)
    }

    /// Gives variant corresponding to a value, this is an O(n) operation as it does so by comparing
    /// every single value contained in [Valued::VALUES]
    fn value_to_variant(value: &Self::Value) -> Self where Self::Value:PartialEq {
        Self::value_to_variant_opt(value).unwrap()
    }
}

/// Gives the value corresponding for a variant of an enum marked with #[repr(usize)], this is an
/// O(1) operation as it just gets the value as a copy from [Valued::VALUES]
///
/// The type of [Valued::Value] doesn't need to implement the [Clone] trait as the array is
/// treated as a raw pointer whose value is read without cloning through
/// [core::ptr::read]
///
/// Note that if implemented correctly (ensured by the declarative macro
/// [crate::create_indexed_valued_enum]), calling this method will always produce
/// [Option::Some(Value)]
pub const fn value_opt_internal<ValuedType: Valued>(variant: &ValuedType) -> Option<ValuedType::Value> {
    let discriminant = discriminant_internal(variant);
    if discriminant >= ValuedType::VARIANTS.len() { return None; }
    let (first_offset, second_offset, third_offset) = split_usize_to_isizes(discriminant);
    Some(unsafe { ValuedType::VALUES.as_ptr().offset(first_offset).offset(second_offset).offset(third_offset).read() })
}

/// Gives the value corresponding for a variant of an enum marked with #[repr(usize)], this is an
/// O(1) operation as it just gets the value as a copy from [Valued::VALUES]
///
/// The type of [Valued::Value] doesn't need to implement the [Clone] trait as the array is
/// treated as a raw pointer whose value is read without cloning through
/// [core::ptr::read]
///
/// Note that if implemented correctly (ensured by the declarative macro
/// [crate::create_indexed_valued_enum]), this method should never panic.
pub const fn value_internal<ValuedType: Valued>(variant: &ValuedType) -> ValuedType::Value {
    let discriminant = discriminant_internal(variant);
    if discriminant >= ValuedType::VARIANTS.len() { panic!("Tried to get a variant's value whose index is larger than the amount of Variants") }
    let (first_offset, second_offset, third_offset) = split_usize_to_isizes(discriminant);
    unsafe { ValuedType::VALUES.as_ptr().offset(first_offset).offset(second_offset).offset(third_offset).read() }
}

