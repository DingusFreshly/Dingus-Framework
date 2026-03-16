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
        World,
        Entity,
    };

    pub use crate::query::prelude::{Query, ArchetypeQuery, With, Without};
    pub use crate::resource::prelude::{Res, ResMut};
    pub use crate::command::prelude::Commands;
    pub use crate::schedule::prelude::{Schedule, Stage};

    pub use dingus_macros::{
        include_resources,
        include_archetypes,
        include_components,
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
        ResourceTrait,
        prelude::{ResourceId, ResourceMap},
    };

    pub use crate::component::{
        prelude::{
            ComponentInfo,
            ComponentIndex,
            ComponentRegistry,
            ComponentTrait,
            ComponentTypeId,
        },
        utils::{make_drop_fn, type_id_to_component_id},
    };

    pub use crate::command::prelude::{Commands, CommandBuffer};

    pub use crate::fast_bit::FastBit;

    pub use crate::generated::{
        ALL_ARCHETYPE_DESCRIPTORS,
        ALL_COMPONENTS,
    };
}