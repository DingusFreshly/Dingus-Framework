use crate::reg_loc::{DenseIndex, RegisteredLocation};
use super::prelude::{ComponentInfo, ComponentTypeId};

/// Marker trait for types that can be stored as ECS components.
/// Implement via `include_components!`.
pub trait ComponentTrait: RegisteredLocation + Send + Sync + 'static {
    const COMPONENT_TYPE_ID: ComponentTypeId = Self::DENSE_INDEX as ComponentTypeId;
    /// Full runtime metadata
    fn component_info() -> ComponentInfo;
}