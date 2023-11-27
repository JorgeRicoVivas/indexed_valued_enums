pub trait Indexed: Sized + 'static {
    const VARIANTS: &'static [Self];

    fn index(&self) -> usize {
        unsafe { *<*const _>::from(self).cast::<usize>() }
    }

    fn split_index_to_offsets(index: usize) -> (isize, isize) {
        match TryInto::<isize>::try_into(index) {
            Ok(index) => { (index, 0) }
            Err(_) => { (isize::MAX, (index - (isize::MAX as usize)) as isize) }
        }
    }

    fn from_index_opt(index: usize) -> Option<Self> {
        let (first_offset, second_offset) = Self::split_index_to_offsets(index);
        Some(unsafe { Self::VARIANTS.as_ptr().offset(first_offset).offset(second_offset).read() })
    }

    fn from_index(index: usize) -> Self {
        Self::from_index_opt(index).unwrap()
    }
}
