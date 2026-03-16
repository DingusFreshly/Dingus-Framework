use super::prelude::*;
use crate::archetype::prelude::*;
use std::marker::PhantomData;
use crate::world::World;
use crate::entity::Entity;
/// A query over a world, returning items of type Q::Item for each matching entity
pub struct Query<'w, Q: QueryParam> {
    world: &'w World,
    state: &'w QueryState<Q>,
}

impl<'w, Q: QueryParam> Query<'w, Q> {
    pub fn new(world: &'w World, state: &'w QueryState<Q>) -> Self {
        Query { world, state }
    }

    #[inline]
    pub fn iter(&self) -> QueryIter<'_, Q> {
        QueryIter::new(self.world, &self.state.matched_archetypes)
    }
    // gets the queried components for a specific entity, if it matches the query TODO! guarentee that the entity has it by return an error otherwise
    pub fn get(&self, entity: Entity) -> Option<Q::Item<'_>> {
        if !self.world.is_alive(entity) { return None; }
        let loc = self.world.entity_locations[entity.index as usize];
        let arch = &self.world.archetypes[loc.archetype_id as usize];
        if !Q::matches_archetype(arch) { return None; }
        Some(unsafe { Q::fetch_ptrs(arch, loc.row as usize) })
    }
}

pub struct QueryIter<'w, Q: QueryParam> {
    world: &'w World,
    archetype_ids: &'w [ArchetypeId],
    arch_index: usize,
    row: usize,
    current_len: usize,
    _phantom: PhantomData<Q>,
}

impl<'w, Q: QueryParam> QueryIter<'w, Q> {
    fn new(world: &'w World, ids: &'w [ArchetypeId]) -> Self {
        let current_len = ids.first().map(|&id| world.archetypes[id as usize].len).unwrap_or(0);
        QueryIter { world, archetype_ids: ids, arch_index: 0, row: 0, current_len, _phantom: PhantomData }
    }

    #[inline(always)]
    fn advance_archetype(&mut self) -> bool {
        loop {
            self.arch_index += 1;
            if self.arch_index >= self.archetype_ids.len() { return false; }
            let len = self.world.archetypes[self.archetype_ids[self.arch_index] as usize].len;
            if len > 0 { self.current_len = len; self.row = 0; return true; }
        }
    }
}

impl<'w, Q: QueryParam> Iterator for QueryIter<'w, Q> {
    type Item = Q::Item<'w>;

    #[inline(always)]
    fn next(&mut self) -> Option<Q::Item<'w>> {
        loop {
            if self.row < self.current_len {
                let arch = &self.world.archetypes[self.archetype_ids[self.arch_index] as usize];
                let item = unsafe { Q::fetch_ptrs(arch, self.row) };
                self.row += 1;
                return Some(item);
            }
            if !self.advance_archetype() { return None; }
        }
    }
}