use crate::archetype::EMPTY_ARCHETYPE_ID;
use crate::component::prelude::{ComponentRegistry, ComponentInfo,ComponentTrait};
use crate::entity::{Entity, EntityIndex, EntityLocation};
use crate::resource::prelude::ResourceMap;
use crate::archetype::prelude::{Archetype, ArchetypeBundle, StaticArchetypeDescriptor, ArchetypeMarker};
use crate::internal::{ALL_ARCHETYPE_DESCRIPTORS, ALL_COMPONENTS};
use crate::resource::ResourceTrait;
/// central struct for storing, creating and destroying ecs data
pub struct World {
    /// readonly after World::new() One slot per declared archetype
    pub archetypes: Vec<Archetype>,

    /// Cached ComponentInfo slices for each archetype (same index as archetypes).
    /// Used by swap_remove_row to call drop functions
    archetype_infos: Vec<Vec<ComponentInfo>>,

    /// Dense entity to (archetype_id, row) table
    pub entity_locations: Vec<EntityLocation>,
    entity_freelist:  Vec<EntityIndex>,
    entity_generation: Vec<u32>,
    entity_count: u32,

    pub resources: ResourceMap,

    /// Readonly after World::new()
    pub component_registry: ComponentRegistry,
}


impl World {
    pub fn new() -> Self {
        let mut registry = ComponentRegistry::new();

        // 1;Register all components via linkme slice
        for register_fn in ALL_COMPONENTS {
            //calls `ComponentRegistry::register()` on all the components
            register_fn(&mut registry);
            
        }
        registry.sort_infos();
        registry.assert_info_order();
        
        //2: Collect archetype descriptors via linkme slice
        let mut raw_descs: Vec<StaticArchetypeDescriptor> = Vec::new();
        for register_fn in ALL_ARCHETYPE_DESCRIPTORS {
            register_fn(&mut raw_descs);
        }
        // Sort by archetype_id so Vec index == ArchetypeId.
        raw_descs.sort_by_key(|d| d.archetype_id);
        //Verify IDs are dense [0, N), this shouldnt break but its good in case
        for (i, d) in raw_descs.iter().enumerate() {
            assert_eq!(d.archetype_id as usize, i,
                       "Archetype IDs must be dense starting at 0. Got {} at position {}",
                       d.archetype_id, i);
        }

        // 3:Build ComponentInfo slices for each archetype.
        let mut archetypes = Vec::with_capacity(raw_descs.len());
        let mut archetype_infos = Vec::with_capacity(raw_descs.len());

        for desc in &raw_descs {
            // Gather component infos for this archetypes components
            // sorted by ComponentTypeId to match column order
            let mut infos: Vec<ComponentInfo> = desc.component_set
                .iter_set()
                .map(|comp_index| registry.get_by_index(comp_index).clone())
                .collect();
            infos.sort_by_key(|i| i.type_id);

            archetypes.push(Archetype::from_descriptor(desc, &infos));
            archetype_infos.push(infos);
        }

        World {
            archetypes,
            archetype_infos,
            entity_locations: Vec::new(),
            entity_freelist: Vec::new(),
            entity_generation: Vec::new(),
            entity_count: 0,
            resources: ResourceMap::new(),
            component_registry: registry,
        }
    }
}

//entity allocation block
impl World {
    fn alloc_entity(&mut self) -> Entity {
        if let Some(index) = self.entity_freelist.pop() {
            Entity { index, generation: self.entity_generation[index as usize] }
        } else {
            let index = self.entity_locations.len() as EntityIndex;
            
            self.entity_locations.push(EntityLocation {
                
                archetype_id: EMPTY_ARCHETYPE_ID,
                row: 0,
            });
            //println!("rahh");
            self.entity_generation.push(0);
            self.entity_count += 1;
            Entity { index, generation: 0 }
        }
    }
    // checks if the entity ids generation matches the most recent one for that Entity's id
    pub fn is_alive(&self, entity: Entity) -> bool {
        let idx = entity.index as usize;
        idx < self.entity_generation.len()
            && self.entity_generation[idx] == entity.generation
    }
}
//spawn block
impl World {
    /// spawn an entity into archetype A with the given bundle.
    /// All component values are written in a single inline call to
    /// ArchetypeBundle::write_into_archetype 
    pub fn spawn<A: ArchetypeMarker>(&mut self, bundle: A::Bundle) -> Entity {
        let entity = self.alloc_entity();
        let arch_id = A::ARCHETYPE_ID as usize;
        
        // 
        let row = self.archetypes[arch_id].alloc_row(entity);
        unsafe { bundle.write_into_archetype(&mut self.archetypes[arch_id], row); }

        self.entity_locations[entity.index as usize] = EntityLocation {
            archetype_id: A::ARCHETYPE_ID,
            row: row as u32,
        };
        entity
    }
    /// despawns an entity with id: Entity
    /// entity must NOT already exist
    pub fn despawn(&mut self, entity: Entity) -> Result<(), (Entity, String)> {
        if !self.is_alive(entity) {
            return Err((entity, format!("Entity {:?} does not exist or is already dead", entity)));
        }

        let loc = self.entity_locations[entity.index as usize];
        let infos = self.archetype_infos[loc.archetype_id as usize].clone();

        let moved = self.archetypes[loc.archetype_id as usize]
            .swap_remove_row(loc.row as usize, &infos);

        if let Some(moved_entity) = moved {
            self.entity_locations[moved_entity.index as usize].row = loc.row;
        }
        
        // reuse ids
        self.entity_generation[entity.index as usize] += 1;
        self.entity_freelist.push(entity.index);
        Ok(())
    }

}

//component block
impl World {
    /// Read a component from a live entity
    /// Panics in debug if the entity's archetype does not contain T
    pub fn get<T: ComponentTrait>(&self, entity: Entity) -> Option<&T> {//TODO! proper error handling here instead of panicking
        if !self.is_alive(entity) { return None; }
        let loc = self.entity_locations[entity.index as usize];
        let arch = &self.archetypes[loc.archetype_id as usize];
        let &col_idx = arch.column_index.get(&T::component_type_id())?;
        Some(unsafe { arch.columns[col_idx].get::<T>(loc.row as usize) })
    }
    
    pub fn get_mut<T: ComponentTrait>(&mut self, entity: Entity) -> Option<&mut T> {
        if !self.is_alive(entity) { return None; }
        let loc = self.entity_locations[entity.index as usize];
        let arch = &self.archetypes[loc.archetype_id as usize];
        let &col_idx = arch.column_index.get(&T::component_type_id())?;
        Some(unsafe { arch.columns[col_idx].get_mut::<T>(loc.row as usize) })
    }

    /// Set a single component value
    /// Only valid for components in the entitys archetype, panics otherwise.
    pub fn set<T: ComponentTrait>(&mut self, entity: Entity, value: T) {//TODO! proper error handling here instead of panicking
        let loc = self.entity_locations[entity.index as usize];
        let arch = &mut self.archetypes[loc.archetype_id as usize];
        let col_idx = arch.column_of(T::component_type_id());
        unsafe { *arch.columns[col_idx].get_mut::<T>(loc.row as usize) = value; }
    }
}

//resource impl
impl World {
    
    pub fn insert_resource<T: ResourceTrait +'static>(&mut self, value: T) {// Send + Sync + 
        self.resources.insert(value);
    }
    pub fn get_resource<T: ResourceTrait + 'static>(&self) -> Option<&T> { self.resources.get::<T>() }
    pub fn get_resource_mut<T: ResourceTrait + 'static>(&mut self) -> Option<&mut T> { self.resources.get_mut::<T>() }
}