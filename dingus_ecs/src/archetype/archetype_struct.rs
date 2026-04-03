use std::collections::HashMap;
pub use crate::component::prelude::{ComponentTypeId, ComponentStorage,ComponentInfo};
use crate::entity::Entity;
use crate::fast_bit::FastBit;

pub use super::prelude::StaticArchetypeDescriptor;

pub struct Archetype {
    pub component_set: FastBit,

    pub columns: Vec<ComponentStorage>,

    pub column_index: HashMap<ComponentTypeId, usize>,

    pub entities: Vec<Entity>,

    pub len: usize,
    pub capacity: usize,
}

impl Archetype {
    /// Create storage for a known archetype
    /// `infos` must be sorted by ComponentTypeId (matches column order)
    pub fn from_descriptor(desc: &StaticArchetypeDescriptor) -> Self {
        let infos = &desc.component_infos;
        let mut columns = Vec::with_capacity(infos.len());
        let mut column_index = HashMap::new();
        
        for (i, info) in infos.iter().enumerate() {
            column_index.insert(info.type_id, i);
            let mut col = ComponentStorage::new(info.type_id, info.layout);
            if desc.initial_capacity > 0 {
                // Pre-grow to avoid early reallocations.
                unsafe { col.reserve(desc.initial_capacity); }
            }
            columns.push(col);
        }
        debug_assert!(columns.len() == infos.len(), "Mismatch between component infos and columns");
        Archetype {
            component_set: desc.component_set,
            columns,
            column_index,
            entities: Vec::new(),
            len: 0,
            capacity: 0,
        }
    }

    #[inline]
    pub fn has_component_id(&self, type_id: ComponentTypeId) -> bool {
        self.column_index.contains_key(&type_id)
    }

    /// Does this archetype satisfy a querys required + forbidden sets?
    #[inline]
    pub fn matches(&self, required: &FastBit, forbidden: &FastBit) -> bool {
        self.component_set.is_superset_of(required)
            && self.component_set.is_disjoint(forbidden)
    }

    #[inline]
    pub fn column_of(&self, type_id: ComponentTypeId) -> usize {
        *self.column_index.get(&type_id)
            .expect("column_of: type not in archetype")
    }

    /// Reserve a row slot for a new entity. Returns the row index
    /// Caller is responsible for writing component values before the row is used
    pub fn alloc_row(&mut self, entity: Entity) -> usize {
        let row = self.len;

        for column in &mut self.columns {
            if column.len == column.capacity {
                unsafe { column.grow(); }
            }
            column.len += 1;
        }
        self.entities.push(entity);

        self.len += 1;
        row
    }

    /// Remove row by swap_remove. Returns the entity swapped into `row` (if any)
    pub fn swap_remove_row(
        &mut self,
        row: usize,
        component_infos: &[ComponentInfo],
    ) -> Option<Entity> {
        for (i, col) in self.columns.iter_mut().enumerate() {
            unsafe { col.swap_remove(row, component_infos[i].drop_fn); }
        }
        self.entities.swap_remove(row);
        self.len -= 1;
        if row < self.len { Some(self.entities[row]) } else { None }
    }
}

