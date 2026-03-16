use crate::internal::StaticArchetypeDescriptor;
use crate::component::prelude::ComponentRegistry;

#[linkme::distributed_slice]
pub static ALL_ARCHETYPE_DESCRIPTORS: [fn(&mut Vec<StaticArchetypeDescriptor>)] = [..];

#[linkme::distributed_slice]
pub static ALL_COMPONENTS: [fn(&mut ComponentRegistry)] = [..];