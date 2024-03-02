/// Allows to get a discriminant from an enum's variant to an usize, and also get the same variant
/// from said discriminant, for example, having the following implementation:
///
/// ```rust
/// use indexed_valued_enums::indexed_enum::Indexed;
///
///
/// enum Number{ First, Second, Third }
///
/// impl Indexed for Number{
///     const VARIANTS: &'static [Self] = &[Number::First, Number::Second, Number::Third];
/// }
/// ```
/// Calling [Indexed::discriminant] on every enum produces [First->0, Second->1, Third->2].
///
/// Calling on [Indexed::from_discriminant] over the enums would produce [0->First, 1->Second, 2->Third].
///
/// Note this documentation it's solely informational, it is dis-recommended to implement this trait
/// manually, but using the derive macro [crate::Valued] or the declarative macro
/// [crate::create_indexed_valued_enum] instead.
pub trait Indexed: Sized + 'static {
    /// Array storing all the variants of the enum ordered by discriminant.
    const VARIANTS: &'static [Self];

    /// Gets the discriminant of this variant, this operation is O(1).
    fn discriminant(&self) -> usize {
        discriminant_internal(self)
    }

    /// Gets the variant corresponding to said discriminant, this operation is O(1) as it just gets
    /// the discriminant as a read-copy from [Indexed::VARIANTS].
    ///
    /// This enum doesn't need to implement the [Clone] trait as the array is treated as a raw
    /// pointer whose value is read without cloning through [core::ptr::read].
    fn from_discriminant_opt(discriminant: usize) -> Option<Self> {
        from_discriminant_opt_internal(discriminant)
    }

    /// Gets the variant corresponding to said discriminant, this operation is O(1) as it just gets
    /// the discriminant as a copy from [Indexed::VARIANTS].
    ///
    /// This operation will panic when the discriminant parameter is a number larger than
    /// [Indexed::VARIANTS]'s length.
    ///
    /// This enum doesn't need to implement the [Clone] trait as the array is treated as a raw
    /// pointer whose value is read without cloning through [core::ptr::read].
    fn from_discriminant(discriminant: usize) -> Self {
        from_discriminant_opt_internal(discriminant).unwrap()
    }
}

/// Gets the discriminant for a variant of an enum marked with #[repr(usize)], this operation is O(1).
///
/// This internal function is used when using 'Delegators'.
pub const fn discriminant_internal<T>(variant: &T) -> usize {
    unsafe { *(variant as *const T).cast::<usize>() }
}

/// Gets the variant corresponding to said discriminant, this operation is O(1) as it just gets
/// the discriminant as a read-copy from [Indexed::VARIANTS].
///
/// The enum bust be marked with #[repr(usize)].
///
/// This enum doesn't need to implement the [Clone] trait as the array is treated as a raw
/// pointer whose value is read without cloning through [core::ptr::read].
pub const fn from_discriminant_internal<TIndexed: Indexed>(discriminant: usize) -> TIndexed {
    if discriminant >= TIndexed::VARIANTS.len() { panic!("Tried to get a variant whose index is larger than the amount of Variants") }
    let (first_offset, second_offset, third_offset) = split_usize_to_isizes(discriminant);
    unsafe { TIndexed::VARIANTS.as_ptr().offset(first_offset).offset(second_offset).offset(third_offset).read() }
}

/// Gets the variant corresponding to said discriminant, this operation is O(1) as it just gets
/// the discriminant as a read-copy from [Indexed::VARIANTS].
///
/// The enum bust be marked with #[repr(usize)] and it doesn't need to implement the [Clone] trait
/// as the array is treated as a raw pointer whose value is read without cloning through
/// [core::ptr::read].
///
/// This internal function is used when using 'Delegators'.
pub const fn from_discriminant_opt_internal<TIndexed: Indexed>(discriminant: usize) -> Option<TIndexed> {
    if discriminant >= TIndexed::VARIANTS.len() { return None }
    let (first_offset, second_offset, third_offset) = split_usize_to_isizes(discriminant);
    Some(unsafe { TIndexed::VARIANTS.as_ptr().offset(first_offset).offset(second_offset).offset(third_offset).read() })
}


/// Divides an usize in three isizes whose sums results in the original usize, used to point on the
/// arrays of [Indexed::VARIANTS] and [super::Valued::VALUES] .
pub(crate) const fn split_usize_to_isizes(usize: usize) -> (isize, isize, isize) {
    match usize.checked_sub(isize::MAX as usize) {
        Some(first_usize) => {
            if first_usize > isize::MAX as usize {
                (isize::MAX, isize::MAX, 1)
            } else {
                (isize::MAX, first_usize as isize, 0)
            }
        }
        None => (usize as isize, 0, 0),
    }
}