pub use crate::fast_bit::FastBit;
pub use crate::component::ComponentIndex;
/// represents what components or resources are read and written to by a query
#[derive(Clone, Default)]
pub struct Access {
    pub reads:  FastBit,
    pub writes: FastBit,
}
impl Access {
    pub fn conflicts_with(&self, other: &Access) -> bool {
        !self.writes.is_disjoint(&other.writes)
            || !self.writes.is_disjoint(&other.reads)
            || !self.reads.is_disjoint(&other.writes)
    }
    pub fn add_read(&mut self, index: ComponentIndex)  { self.reads.set(index); }
    pub fn add_write(&mut self, index: ComponentIndex) { self.writes.set(index); }
}