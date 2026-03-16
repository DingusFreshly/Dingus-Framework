mod archetype_marker;
mod archetype_bundle;
mod archetype_static_descriptor;
mod archetype_struct;

/// INdex into 'World::archetypes'
pub type ArchetypeId = u32;

/// entities in the empty archetype (freshly allocated, not yet written).
pub const EMPTY_ARCHETYPE_ID: ArchetypeId = u32::MAX;


pub mod prelude {
    pub use super::ArchetypeId;
    pub use super::archetype_marker::ArchetypeMarker;
    pub use super::archetype_bundle::ArchetypeBundle;
    pub use super::archetype_static_descriptor::StaticArchetypeDescriptor;
    pub use super::archetype_struct::Archetype;
    pub use super::EMPTY_ARCHETYPE_ID;
    
    //pub use super::archetype_struct::ALL_ARCHETYPE_DESCRIPTORS;
}