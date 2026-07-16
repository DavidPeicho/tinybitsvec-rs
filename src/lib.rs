use std::num::NonZero;

#[cfg(feature = "rkyv")]
mod rkyv;

#[derive(Debug, Default)]
pub struct BitVec {
    storage: Vec<u32>,
    len: usize,
}

const SIZE_IN_BYTES: usize = std::mem::size_of::<u32>();
const SIZE_IN_BITS: usize = 8 * SIZE_IN_BYTES;

#[inline]
fn align_index(bit_index: usize) -> usize {
    bit_index / SIZE_IN_BITS
}
#[inline]
fn bit_index(index: usize) -> usize {
    index % SIZE_IN_BITS
}
#[inline]
fn align_count(bit_index: usize) -> usize {
    bit_index.div_ceil(SIZE_IN_BITS)
}

impl BitVec {
    pub const ELEMENTS_PER_WORD: usize = SIZE_IN_BITS;

    pub fn empty() -> Self {
        Self {
            storage: Vec::new(),
            len: 0,
        }
    }

    pub fn new(size: NonZero<usize>) -> Self {
        let mut bits = Self::empty();
        bits.grow(size.get());
        bits
    }

    pub fn new_with_value(size: usize, value: bool) -> Self {
        let value = if value { u32::MAX } else { 0 };
        let len = align_count(size);

        let mut bits = Self::empty();
        bits.storage.resize(len, value);
        bits.len += size;
        bits
    }

    pub fn unset_all(&mut self) {
        self.storage.fill(0);
    }

    pub fn set_all(&mut self) {
        self.storage.fill(1);
    }

    pub fn from_bools(booleans: &[bool]) -> Self {
        if booleans.len() == 0 {
            return Self::empty();
        }
        let mut bits = BitVec::new(NonZero::new(booleans.len()).unwrap());
        for i in 0..booleans.len() {
            bits.set_value(i, booleans[i]);
        }
        bits
    }

    pub fn push(&mut self, value: bool) {
        if self.capacity() == 0 {
            self.grow(1);
        }
        self.set_value(self.len, value);
        self.len += 1;
    }

    pub fn set_range(&mut self, range: std::ops::Range<usize>) {
        // TODO: Could be optimized based on alignment
        for i in range {
            self.set(i);
        }
    }

    pub fn grow(&mut self, extra_capacity: usize) {
        let len = align_count(extra_capacity);
        self.storage.resize(self.storage.len() + len, 0);
        self.len += extra_capacity;
    }

    pub fn drain(&mut self, range: std::ops::Range<usize>) {
        for src in range.end..self.len {
            let dst = range.start + (src - range.end);
            let value = self.get_unsafe(src);
            self.set_value(dst, value);
        }
        self.len -= range.len();
    }

    #[inline]
    pub fn set_value(&mut self, index: usize, value: bool) {
        if value {
            self.set(index);
        } else {
            self.unset(index);
        }
    }

    #[inline]
    pub fn set(&mut self, index: usize) {
        let block = align_index(index);
        let bit = bit_index(index);
        self.storage[block] = self.storage[block] | (1 << bit);
    }

    #[inline]
    pub fn unset(&mut self, index: usize) {
        let block = align_index(index);
        let bit = bit_index(index);
        self.storage[block] = self.storage[block] & !(1 << bit);
    }

    // TODO: Should be on a slice type
    #[inline]
    pub fn get_unsafe(&self, index: usize) -> bool {
        let block = align_index(index);
        let bit = bit_index(index);
        self.storage[block] & (1 << bit) != 0
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<bool> {
        if index < self.len {
            Some(self.get_unsafe(index))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn words(&self) -> &[u32] {
        &self.storage
    }

    fn capacity(&self) -> usize {
        let bits = self.storage.len() * SIZE_IN_BITS;
        assert!(bits >= self.len);
        bits - self.len
    }
}

#[cfg(test)]
mod tests {
    use crate::BitVec;

    #[test]
    fn get_set() {
        // Less than a word
        let booleans = [true, false, false, true];
        let bits = BitVec::from_bools(&booleans);
        assert_eq!(bits.len(), 4);
        for i in 0..booleans.len() {
            assert_eq!(bits.get(i), Some(booleans[i]));
        }

        // Exactly a word
        let mut booleans = Vec::new();
        booleans.resize(32, false);
        booleans[1] = true;
        booleans[29] = true;
        booleans[31] = true;
        let bits = BitVec::from_bools(&booleans);
        assert_eq!(bits.len(), 32);
        for i in 0..booleans.len() {
            assert_eq!(bits.get(i), Some(booleans[i]), "bit {}", i);
        }

        // Multi-words
        let mut booleans = Vec::new();
        booleans.resize(33, false);
        booleans[0] = true;
        booleans[29] = true;
        booleans[32] = true;
        let bits = BitVec::from_bools(&booleans);
        assert_eq!(bits.len(), 33);
        for i in 0..booleans.len() {
            assert_eq!(bits.get(i), Some(booleans[i]));
        }
    }

    #[test]
    fn drain() {
        let mut expected = Vec::new();
        expected.resize(33, false);
        expected[0] = true;
        expected[4] = true;
        expected[29] = true;
        expected[32] = true;

        let mut bits = BitVec::from_bools(&expected);

        // Drain by the start
        let range = 0..5;
        bits.drain(range.clone());
        expected.drain(range);
        assert_eq!(bits.len(), expected.len());
        for i in 0..expected.len() {
            assert_eq!(bits.get(i), Some(expected[i]));
        }

        // Drain middle
        let range = 11..17;
        expected[13] = true;
        bits.set(13);
        bits.drain(range.clone());
        expected.drain(range);
        assert_eq!(bits.len(), expected.len());
        for i in 0..expected.len() {
            assert_eq!(bits.get(i), Some(expected[i]));
        }

        // Drain by the end
        let range = (bits.len() - 5)..bits.len();
        bits.drain(range.clone());
        expected.drain(range);
        assert_eq!(bits.len(), expected.len());
        for i in 0..expected.len() {
            assert_eq!(bits.get(i), Some(expected[i]));
        }

        // Drain all
        let range = 0..bits.len();
        bits.drain(range.clone());
        expected.drain(range);
    }
}
