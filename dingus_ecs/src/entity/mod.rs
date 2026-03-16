use crate::archetype::ArchetypeId;

/// Dense index into 'World::entity_locations'.
pub type EntityIndex = u32;

//Entity: versioned index.
// Generation counter invalidates handles to despawned entities

/// A versioned handle to a live entity in the World.
/// Cheap to copy, 8 bytes total.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Entity {
    pub index: EntityIndex,
    /// Incremented each time this index slot is recycled.
    pub generation: u32,
}

impl Entity {
    /// Sentinel used internally for "no entity".
    pub const DANGLING: Entity = Entity { index: u32::MAX, generation: 0 };
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EntityLocation {
    pub archetype_id: ArchetypeId,
    pub row: u32,
}