use super::prelude::{ComponentIndex, ComponentTypeId, ComponentInfo};
use std::collections::HashMap;
use linkme::distributed_slice;
use crate::fast_bit::FASTBIT_WORDS;

/// Maximum distinct component types the bitset can track.
/// increase FASTBIT_WORDS to support more components
pub const MAX_COMPONENTS: usize = FASTBIT_WORDS * 64;
/// Represents a set of component types attached to an archetype
/// Global mutable registry, written once at startup by `World`, read-only thereafter.
pub struct ComponentRegistry {
    /// Indexed by ComponentIndex
    infos: Vec<ComponentInfo>,
    /// Maps ComponentTypeId hash to its index in infos, only used during debug
    by_type_id: HashMap<ComponentTypeId, ComponentIndex>,
    /// Used for making sure len(infos) <= MAX_COMPONENTS
    next_index: ComponentIndex,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        ComponentRegistry {
            infos: Vec::new(), by_type_id: HashMap::new(), next_index: 0,
        }
    }

    /// Register a component type and return its assigned ComponentIndex.
    /// Panics if called after simulation begins or if MAX_COMPONENTS is exceeded.
    pub fn register(&mut self, mut info: ComponentInfo) -> ComponentIndex {
        assert!(
            self.next_index < MAX_COMPONENTS as u32,
            "ComponentRegistry: exceeded MAX_COMPONENTS ({})", MAX_COMPONENTS
        );
        if let Some(&existing) = self.by_type_id.get(&info.type_id) {
            //debug_assert!(false, "Component {} registered twice!")
            return existing; // idempotent re-registration
        }
        let index = self.next_index;
        info.index = index;
        self.by_type_id.insert(info.type_id, index);
        self.infos.push(info);
        self.next_index += 1;
        index
    }

    /// Get component info by component hash, only used during debug
    pub fn get_by_type_id(&self, type_id: ComponentTypeId) -> Option<&ComponentInfo> {
        self.by_type_id
            .get(&type_id)
            .map(|&idx| &self.infos[idx as usize])
    }

    pub fn get_by_index(&self, index: ComponentIndex) -> &ComponentInfo {
        &self.infos[index as usize]
    }
}
