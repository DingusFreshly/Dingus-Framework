use super::prelude::*;
use crate::archetype::prelude::*;
use crate::entity::Entity;
/// System parameter providing access to a per-system CommandBuffer.
/// Commands are buffered during the run and flushed after the stage.
pub struct Commands<'w> {
    pub buffer: &'w mut CommandBuffer,
}

impl<'w> Commands<'w> {
    pub fn spawn<A: ArchetypeMarker>(&mut self, bundle: A::Bundle) {
        self.buffer.spawn::<A>(bundle);
    }
    pub fn despawn(&mut self, entity: Entity) {
        self.buffer.despawn(entity);
    }
}