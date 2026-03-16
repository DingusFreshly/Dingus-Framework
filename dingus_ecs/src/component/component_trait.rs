use super::prelude::{ComponentInfo, ComponentIndex, ComponentTypeId};
use super::util::{const_fnv1a, type_id_to_component_id};

/// Marker trait for types that can be stored as ECS components.
/// Implement via `include_components!`.
pub trait ComponentTrait: Send + Sync + 'static {
    /// Dense index in [0, MAX_COMPONENTS)
    /// Assigned by include_components! and stable for the entire process lifetime.
    const COMPONENT_INDEX: ComponentIndex;

    /// used in static initialisers.
    fn component_type_id() -> ComponentTypeId;
    /// Full runtime metadata
    fn component_info() -> ComponentInfo;
}