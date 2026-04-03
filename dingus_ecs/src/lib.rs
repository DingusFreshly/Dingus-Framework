// ============================
// Module declarations
// ============================

mod entity;
mod fast_bit;
mod component;
mod archetype;
mod resource;
mod world;
mod query;
mod command;
mod schedule;
mod generated;
mod reg_loc;
mod export;
// ============================
// Public API
// ============================

pub use world::World;
pub use entity::Entity;


// ============================
// Prelude
// ============================

pub mod prelude {
    pub use crate::{
        Entity,
        World,
    };
    pub use hashed_type_def::HashedTypeDef;
    pub use crate::query::prelude::{ArchetypeQuery, Query, With, Without};
    pub use crate::resource::prelude::{Res, ResMut};
    pub use crate::command::prelude::Commands;
    pub use crate::schedule::prelude::{Schedule, Stage};
    pub use crate::component::prelude::ComponentTrait;

    pub use dingus_macros::{
        include_archetypes,
        include_components,
        include_resources,
        Component,
        archetype
    };
}


// ============================
// Internal API 
// ============================

#[doc(hidden)]
pub mod internal {
    pub use linkme::distributed_slice;
    pub use crate::entity::Entity;

    pub use crate::archetype::prelude::{
        Archetype,
        ArchetypeBundle,
        ArchetypeId,
        ArchetypeMarker,
        StaticArchetypeDescriptor,
    };

    pub use crate::resource::{
        prelude::{ResourceId, ResourceMap},
        ResourceTrait,
    };
    pub use crate::prelude::World;
    
    pub use crate::reg_loc::{DenseIndex, RegisteredLocation, RegisterType};
    pub use crate::component::{
        //ExportPropertyInfo,

        prelude::{
            ComponentInfo,
            ComponentRegistry,
            ComponentTrait,
            ComponentTypeId,
            
        },
        utils::{const_fnv1a, make_drop_fn, type_id_to_component_id},
    };

    pub use crate::command::prelude::{CommandBuffer, Commands};
    pub use crate::export::{DingusPrimitive, InstanceDef, DingusTypeHint,PropertyDef,ScriptError};
    pub use crate::fast_bit::FastBit;
    pub use phf::Map as PhfMap;
    pub use phf::phf_map;

    pub use crate::generated::{
        ALL_ARCHETYPE_DESCRIPTORS,
        ALL_COMPONENTS,
    };
}