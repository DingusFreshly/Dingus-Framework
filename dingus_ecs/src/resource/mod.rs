mod resource_map;
mod res;
mod resource_trait;

pub type ResourceId = u32;
pub use resource_trait::ResourceTrait;
pub mod prelude {
    pub use super::ResourceId;
    pub use crate::resource::res::*;
    pub use crate::resource::resource_map::*;
}
