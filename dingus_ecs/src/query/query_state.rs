use super::prelude::*;
use crate::archetype::prelude::*;
use std::marker::PhantomData;
use crate::world::World;
pub struct QueryState<Q: QueryParam> {
    /// Immutable after World::new().
    pub matched_archetypes: Vec<ArchetypeId>,//TODO! add resources access before parralel
    pub component_access: Access,
    _phantom: PhantomData<Q>,
}

impl<Q: QueryParam> QueryState<Q> {
    /// Build the state by scanning all archetypes once.
    /// Called at system initialisation, never again.
    pub fn new(world: &World) -> Self {
        let mut matched = Vec::new();
        for (id, archetype) in world.archetypes.iter().enumerate() {
            if Q::matches_archetype(archetype) {
                matched.push(id as ArchetypeId);
            }
        }
        // Build component access by inspecting matched archetypes.
        // (In practice the scheduler derives access from system param types,
        //  not from the matched archetype list, but we store it here for convenience.)
        QueryState {
            matched_archetypes: matched,
            component_access: Access::default(),
            _phantom: PhantomData,
        }
    }
}