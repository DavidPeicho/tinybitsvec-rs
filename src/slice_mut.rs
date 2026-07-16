use crate::{bit_index, slice::storage_range};
use std::ops::Range;

#[derive(Debug)]
pub struct SliceMut<'a> {
    pub(crate) storage: &'a mut [u32],
    pub(crate) range: std::ops::Range<usize>,
}

impl<'a> SliceMut<'a> {
    pub fn set_range(&mut self, range: std::ops::Range<usize>) {
        // TODO: Could be optimized based on alignment
        for i in range {
            self.set(i);
        }
    }

    #[inline]
    pub fn unset_all(&mut self) {
        self.storage.fill(0);
    }

    #[inline]
    pub fn set_all(&mut self) {
        self.storage.fill(u32::MAX);
    }

    pub fn set_value(&mut self, index: usize, value: bool) {
        let index = self.relative_index(index);
        if value {
            bit_set!(self.storage, index);
        } else {
            bit_unset!(self.storage, index);
        }
    }

    #[inline]
    pub fn set(&mut self, index: usize) {
        let index = self.relative_index(index);
        bit_set!(self.storage, index);
    }

    #[inline]
    pub fn unset(&mut self, index: usize) {
        let index = self.relative_index(index);
        bit_unset!(self.storage, index);
    }

    pub fn slice(self, range: Range<usize>) -> Self {
        let start = self.range.start + range.start;
        let end = self.range.start + range.end;
        Self {
            storage: &mut self.storage[storage_range(start..end)],
            range: bit_index(start)..(bit_index(start) + range.len()),
        }
    }

    #[inline]
    fn relative_index(&self, index: usize) -> usize {
        self.range.start + index
    }
}

impl_slice!(SliceMut<'a>);
impl_index!(SliceMut<'_>);
