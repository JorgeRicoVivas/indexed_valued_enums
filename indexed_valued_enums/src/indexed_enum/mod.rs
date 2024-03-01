/// Allows to get a discriminant from an enum's variant to an usize, and also get the same variant
/// from said discriminant, having the following enum:
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
/// Calling [Indexed::discriminant] on every enum produces [First->0, Second->1, Third->2]
///
/// Calling on [Indexed::from_discriminant] over the enums would produce [0->First, 1->Second, 2->Third]
///
/// Note this documentation it's solely informational, it is dis-recommended to implement this trait
/// manually, but using the [crate::create_indexed_valued_enum] instead
pub trait Indexed: Sized + 'static {
    /// Array storing all the variants of the enum ordered by discriminant
    const VARIANTS: &'static [Self];

    /// Gets the discriminant of this variant, this operation is O(1)
    fn discriminant(&self) -> usize {
        unsafe { *<*const _>::from(self).cast::<usize>() }
    }

    /// Gets the variant corresponding to said discriminant, this operation is O(1) as it just gets
    /// the discriminant as a copy from [Indexed::VARIANTS]
    ///
    /// This enum doesn't need to implement the [Clone] trait as the array is treated as a raw
    /// pointer whose value is read without cloning through [core::ptr::read]
    fn from_discriminant_opt(discriminant: usize) -> Option<Self> {
        if discriminant>=Self::VARIANTS.len(){return None}
        let (first_offset, second_offset) = split_usize_to_isizes(discriminant);
        Some(unsafe { Self::VARIANTS.as_ptr().offset(first_offset).offset(second_offset).read() })
    }

    /// Gets the variant corresponding to said discriminant, this operation is O(1) as it just gets
    /// the discriminant as a copy from [Indexed::VARIANTS]
    ///
    /// This operation will panic when the discriminant parameter is a number larger than
    /// [Indexed::VARIANTS]'s length
    ///
    /// This enum doesn't need to implement the [Clone] trait as the array is treated as a raw
    /// pointer whose value is read without cloning through [core::ptr::read]
    fn from_discriminant(discriminant: usize) -> Self {
        Self::from_discriminant_opt(discriminant).unwrap()
    }
}

///Divides an usize in two isizes whose sums results in the original usize
pub(crate) fn split_usize_to_isizes(usize: usize) -> (isize, isize) {
    match TryInto::<isize>::try_into(usize) {
        Ok(usize) => { (usize, 0) }
        Err(_) => { (isize::MAX, (usize - (isize::MAX as usize)) as isize) }
    }
}