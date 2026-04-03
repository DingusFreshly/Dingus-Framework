use super::{ComponentTypeId};
use std::alloc::Layout;

/// Everything the ECS needs to know about a component type at runtime.
/// Produced by the include_components! macro and registered once at startup.
#[derive(Clone, Debug)]
pub struct ComponentInfo {
    
    /// dense index in [0, MAX_COMPONENTS) that represents its id. Assigned by ComponentRegistry.
    pub type_id: ComponentTypeId,

    /// std::alloc::Layout of a single component value.
    pub layout: Layout,

    /// Optional destructor. None for Copy/trivially-destructible types.
    /// Signature: fn(ptr: *mut u8) , drops the value at ptr in-place.
    pub drop_fn: Option<unsafe fn(*mut u8)>,

    /// Human readable name (from std::any::type_name). Editor / debug only.
    pub name: &'static str,
    // Properties of this component that should be exported to the editor. Editor only.
    //pub export_properties : &'static [ExportPropertyInfo],
}
impl ComponentInfo {
    pub fn empty() -> ComponentInfo {
        ComponentInfo {
            type_id : super::EMPTY_COMPONENT,
            layout: Layout::new::<ComponentTypeId>(),
            drop_fn: None,
            name: "EMPTY COMPONENT - THIS INDICATES AN ERROR MOST LIKELY",
            //export_properties: &[],
        }
    }
}
    