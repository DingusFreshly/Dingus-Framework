use super::prelude::*;
use crate::archetype::prelude::*;
use std::marker::PhantomData;
use crate::world::World;
/// A query that iterates exactly one statically-known archetype.
/// Compiles to a single pointer-striding loop over one array.
///
/// Use when the system logically targets a specific archetype.
/// Prefer Query<Q> when the same system must handle multiple archetypes.
pub struct ArchetypeQuery<'w, A: ArchetypeMarker, Q: QueryParam> {
    archetype: &'w Archetype,
    _phantom: PhantomData<(A, Q)>,
}

impl<'w, A: ArchetypeMarker, Q: QueryParam> ArchetypeQuery<'w, A, Q> {
    pub fn new(world: &'w World) -> Self {
        let arch = &world.archetypes[A::ARCHETYPE_ID as usize];
        debug_assert!(
            true,//Q::matches_archetype(arch),
            "ArchetypeQuery: archetype {} does not satisfy query", A::NAME
        );
        ArchetypeQuery { archetype: arch, _phantom: PhantomData }
    }

    #[inline]
    pub fn iter(&self) -> ArchetypeQueryIter<'_, Q> {
        ArchetypeQueryIter { archetype: self.archetype, row: 0, _phantom: PhantomData }
    }

    pub fn len(&self) -> usize { self.archetype.len }
    pub fn is_empty(&self) -> bool { self.archetype.len == 0 }
}
/// query components that belong to one archetype
/// archetype must have those components
pub struct ArchetypeQueryIter<'w, Q: QueryParam> {
    archetype: &'w Archetype,
    row: usize,
    _phantom: PhantomData<Q>,
}

impl<'w, Q: QueryParam> Iterator for ArchetypeQueryIter<'w, Q> {
    type Item = Q::Item<'w>;

    /// Innermost loop. Single archetype, single pointer stride
    /// no branching so its like, very fast hopefully
    #[inline(always)]
    fn next(&mut self) -> Option<Q::Item<'w>> {
        //if self.archetype.entities.len() <= self.row { return None; }
        //println!("row : {}, len : {}", self.row, self.archetype.len);
        if self.row >= self.archetype.len { return None; }
        let item = unsafe { Q::fetch_ptrs(self.archetype, self.row) };
        self.row += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.archetype.len - self.row;
        (n, Some(n))
    }
}
impl<'w, Q: QueryParam> ExactSizeIterator for ArchetypeQueryIter<'w, Q> {}