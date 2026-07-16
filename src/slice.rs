use std::ops::Range;

const SIZE_IN_BYTES: usize = std::mem::size_of::<u32>();
pub(crate) const SIZE_IN_BITS: usize = 8 * SIZE_IN_BYTES;

#[inline]
pub(crate) const fn block_index(bit_index: usize) -> usize {
    bit_index / SIZE_IN_BITS
}
#[inline]
pub(crate) const fn bit_index(index: usize) -> usize {
    index % SIZE_IN_BITS
}
#[inline]
pub(crate) const fn align_count(bit_index: usize) -> usize {
    bit_index.div_ceil(SIZE_IN_BITS)
}
#[inline]
pub(crate) const fn storage_range(range: std::ops::Range<usize>) -> std::ops::Range<usize> {
    block_index(range.start)..align_count(range.end)
}

#[derive(Debug, Clone)]
pub struct Slice<'a> {
    pub(crate) storage: &'a [u32],
    pub(crate) range: Range<usize>,
}

impl Slice<'_> {
    pub fn slice(&self, range: Range<usize>) -> Self {
        let start = self.range.start + range.start;
        let end = self.range.start + range.end;
        Self {
            storage: &self.storage[storage_range(start..end)],
            range: bit_index(start)..(bit_index(start) + range.len()),
        }
    }
}

impl_slice!(Slice<'a>);
impl_index!(Slice<'_>);
