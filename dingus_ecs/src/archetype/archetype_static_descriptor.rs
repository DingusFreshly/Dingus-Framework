/// Runtime descriptor for a statically declared archetype.
/// Produced by include_archetypes! and stored in ALL_ARCHETYPE_DESCRIPTORS.
pub use super::prelude::{ArchetypeId};
pub use crate::component::prelude::{ComponentInfo};
use crate::export::InstanceDef;
pub use crate::fast_bit::FastBit;
pub struct StaticArchetypeDescriptor {
    pub archetype_id: ArchetypeId,
    pub name: &'static str,
    pub component_set: FastBit,
    /// sorted by ComponentTypeId and determines column order in Archetype
    pub component_infos: &'static [ComponentInfo],
    /// Initial capacity to pre allocate for each column
    /// 0 = use ComponentStorage default.
    pub initial_capacity: usize,
    /// Defines how this archetype maps to external scripting systems
    pub instance_def: InstanceDef

}
