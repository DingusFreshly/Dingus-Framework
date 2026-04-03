use super::prelude::{ComponentTypeId, ComponentInfo};
use crate::fast_bit::FASTBIT_WORDS;

/// Maximum distinct component types the bitset can track.
/// increase FASTBIT_WORDS to support more components
pub const MAX_COMPONENTS: usize = FASTBIT_WORDS * 64;
/// Represents a set of component types attached to an archetype
/// Global mutable registry, written once at startup by `World`, read-only thereafter.
pub struct ComponentRegistry {
    /// Indexed by ComponentIndex
    pub(crate) infos: Vec<ComponentInfo>,
    // Maps ComponentTypeId hash to its index in infos, only used during debug
    //by_type_id: HashMap<ComponentTypeId, ComponentIndex>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        ComponentRegistry {
            infos: Vec::new(),
        }
    }

    /// Register a component type and return its assigned ComponentIndex.
    /// Panics if called after simulation begins or if MAX_COMPONENTS is exceeded.
    pub fn register(&mut self, info: ComponentInfo) -> ComponentTypeId {
        assert!(
            self.infos.len() < MAX_COMPONENTS,
            "ComponentRegistry: exceeded MAX_COMPONENTS ({}), increase `FastBit::FASTBIT_WORDS` to add more", MAX_COMPONENTS
        );
        
        let index = info.type_id;

        if self.infos.len() <= index as usize {
            self.infos.resize(index as usize + 1,  ComponentInfo::empty());
        }
        
        self.infos[index as usize ] = info ;
        index
    }
    pub fn sort_infos(&mut self) {
        self.infos.sort_by_key(|c| c.type_id)
    }
    pub fn assert_info_order(&self) {
        for index in self.infos.iter() {
            println!("{}", index.type_id);
        }
        for (i, d) in self.infos.iter().enumerate() {
            assert_eq!(d.type_id as usize, i,
                       "Component IDs must be dense starting at 0. Got {} at position {}",
                       d.type_id, i);
        }
    }
    
    pub fn get_by_index(&self, index: ComponentTypeId) -> &ComponentInfo {
        &self.infos[index as usize]
    }
}
