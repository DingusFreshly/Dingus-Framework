use crate::archetype::prelude::*;
use crate::world::World;
use crate::entity::Entity;
/// An action to be applied to the World after a system stage completes.
enum Command {
    Despawn(Entity),
    SpawnFn(Box<dyn FnOnce(&mut World) + Send>),
}

/// Accumulates World-mutating commands from systems that hold only `&World`.
/// Flushed between stages by the Scheduler.

pub struct CommandBuffer {
    commands: Vec<Command>,
}

impl CommandBuffer {
    pub fn new() -> Self { CommandBuffer { commands: Vec::new() } }

    pub fn despawn(&mut self, entity: Entity) {
        self.commands.push(Command::Despawn(entity));
    }

    /// Queue a spawn of archetype A with the given bundle.
    pub fn spawn<A: ArchetypeMarker>(&mut self, bundle: A::Bundle) {
        self.commands.push(Command::SpawnFn(Box::new(move |world: &mut World| {
            world.spawn::<A>(bundle);
        })));
    }

    /// Apply all queued commands and clear the buffer.
    pub fn flush(&mut self, world: &mut World) {
        for cmd in self.commands.drain(..) {
            match cmd {
                Command::Despawn(e)  => { 
                    let err = world.despawn(e); 
                    if let Err((entity, msg)) = err {
                        panic!(
                            "Failed to despawn entity : {:?} \n Reason : {}",
                            entity,
                            msg,
                        );
                    }
                },
                Command::SpawnFn(f)  => f(world),
            }
        }
    }
}