mod system_param;
mod system_trait;
mod stage;

pub mod prelude {
    pub use super::system_param::SystemParam;
    pub use super::system_trait::{System,};
    pub use super::stage::{Schedule, Stage};
}