mod component_info;
mod component_registry;
mod util;
mod component_trait;
mod component_storage;

/// Packed position of a component type inside a FastBit bitset.
/// Assigned once at registration time; stable for the lifetime of the process.
pub type ComponentTypeId = u64;

pub const EMPTY_COMPONENT : u64 = u64::MAX;

pub mod utils {
    pub use super::util::type_id_to_component_id;
    pub use super::util::make_drop_fn;
    pub use super::util::const_fnv1a;
}



pub mod prelude {
    pub use super::{ComponentTypeId};
    pub use super::component_info::ComponentInfo;
    pub use super::component_registry::ComponentRegistry;
    pub use super::component_trait::ComponentTrait;
    pub use super::component_storage::ComponentStorage;
}
