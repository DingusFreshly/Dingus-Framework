use crate::component::component_registry::ComponentRegistry;

mod component_info;
mod component_registry;
mod util;
mod component_trait;
mod component_storage;

/// Numeric identity for a component type
/// Computed once per type via hash of TypeId
pub type ComponentTypeId = u64;

/// Packed position of a component type inside a FastBit bitset.
/// Assigned once at registration time; stable for the lifetime of the process.
pub type ComponentIndex = u32;

pub mod utils {
    pub use super::util::type_id_to_component_id;
    pub use super::util::make_drop_fn;
    pub use super::util::const_fnv1a;
}



pub mod prelude {
    pub use super::{ComponentIndex, ComponentTypeId};
    pub use super::component_info::ComponentInfo;
    pub use super::component_registry::ComponentRegistry;
    pub use super::component_trait::ComponentTrait;
    pub use super::component_storage::ComponentStorage;
}
