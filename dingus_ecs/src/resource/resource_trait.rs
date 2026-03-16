use crate::resource::ResourceId;
/// trait for resources, res must implement this to be used as a resource in the world
pub trait ResourceTrait: {
        /// Dense index in [0, MAX_RESOURCES).
        /// Assigned by include_resources! and stable for the entire process lifetime.
        const RESOURCE_INDEX: ResourceId;
}