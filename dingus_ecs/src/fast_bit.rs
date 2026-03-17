/// Fast bitset library with const support
/// Use case: const MY_SET: FastBit = FastBit::EMPTY
///    .with(Position::COMPONENT_INDEX)
///    .with(Velocity::COMPONENT_INDEX);


/// * 64 slots
/// 

pub use crate::component::prelude::{ComponentTypeId};

pub const FASTBIT_WORDS: usize = 4; // 256 component slots

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
pub struct FastBit {
    pub words: [u64; FASTBIT_WORDS],
}

impl FastBit {
    pub const EMPTY: FastBit = FastBit { words: [0; FASTBIT_WORDS] };

    // ── const methods (usable in const expressions) ───────────────────────────

    /// Returns a new FastBit with bit `index` set. Used in const contexts.
    #[inline]
    pub const fn with(mut self, index: ComponentTypeId) -> FastBit {
        let word = (index / 64) as usize;
        let bit  = index % 64;
        self.words[word] |= 1u64 << bit;
        self
    }

    /// Const superset check.
    #[inline]
    pub const fn is_superset_of_const(&self, other: &FastBit) -> bool {
        let mut i = 0;
        while i < FASTBIT_WORDS {
            if (self.words[i] & other.words[i]) != other.words[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Const disjoint check.
    #[inline]
    pub const fn is_disjoint_const(&self, other: &FastBit) -> bool {
        let mut i = 0;
        while i < FASTBIT_WORDS {
            if self.words[i] & other.words[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    // ── runtime methods (for non-const use) ──────────────────────────────────

    #[inline] pub fn set(&mut self, index: ComponentTypeId) {
        self.words[(index / 64) as usize] |= 1u64 << (index % 64);
    }
    #[inline] pub fn unset(&mut self, index: ComponentTypeId) {
        self.words[(index / 64) as usize] &= !(1u64 << (index % 64));
    }
    #[inline] pub fn contains(&self, index: ComponentTypeId) -> bool {
        (self.words[(index / 64) as usize] >> (index % 64)) & 1 == 1
    }
    #[inline] pub fn is_superset_of(&self, other: &FastBit) -> bool {
        self.is_superset_of_const(other)
    }
    #[inline] pub fn is_disjoint(&self, other: &FastBit) -> bool {
        self.is_disjoint_const(other)
    }
    pub fn is_empty(&self) -> bool { self.words.iter().all(|&w| w == 0) }

    pub fn iter_set(&self) -> FastBitIter {
        FastBitIter { bits: *self, word: 0, word_bits: self.words[0] }
    }
}

pub struct FastBitIter {
    bits: FastBit,
    word: usize,
    word_bits: u64,
}
impl Iterator for FastBitIter {
    type Item = ComponentTypeId;
    fn next(&mut self) -> Option<ComponentTypeId> {
        while self.word_bits == 0 {
            self.word += 1;
            if self.word >= FASTBIT_WORDS { return None; }
            self.word_bits = self.bits.words[self.word];
        }
        let bit = self.word_bits.trailing_zeros() as u64;
        self.word_bits &= self.word_bits - 1;
        Some(self.word as ComponentTypeId * 64 + bit)
    }
}