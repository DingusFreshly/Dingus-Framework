use super::{ComponentIndex, ComponentTypeId};
use std::alloc::Layout;
/// Everything the ECS needs to know about a component type at runtime.
/// Produced by the include_components! macro and registered once at startup.
#[derive(Clone, Debug)]
pub struct ComponentInfo {
    /// Unique numeric identity derived from std::any::TypeId.
    pub type_id: ComponentTypeId,

    /// dense index in [0, MAX_COMPONENTS). Assigned by ComponentRegistry.
    pub index: ComponentIndex,

    /// std::alloc::Layout of a single component value.
    pub layout: Layout,

    /// Optional destructor. None for Copy/trivially-destructible types.
    /// Signature: fn(ptr: *mut u8) , drops the value at ptr in-place.
    pub drop_fn: Option<unsafe fn(*mut u8)>,

    /// Human readable name (from std::any::type_name). Editor / debug only.
    pub name: &'static str,
}
    