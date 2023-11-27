pub trait Indexed: Sized + 'static {
    const VARIANTS: &'static [Self];

    fn discriminant(&self) -> usize {
        unsafe { *<*const _>::from(self).cast::<usize>() }
    }

    fn split_discriminants_to_offsets(discriminant: usize) -> (isize, isize) {
        match TryInto::<isize>::try_into(discriminant) {
            Ok(discriminant) => { (discriminant, 0) }
            Err(_) => { (isize::MAX, (discriminant - (isize::MAX as usize)) as isize) }
        }
    }

    fn from_discriminant_opt(discriminant: usize) -> Option<Self> {
        let (first_offset, second_offset) = Self::split_discriminants_to_offsets(discriminant);
        Some(unsafe { Self::VARIANTS.as_ptr().offset(first_offset).offset(second_offset).read() })
    }

    fn from_discriminant(discriminant: usize) -> Self {
        Self::from_discriminant_opt(discriminant).unwrap()
    }
}