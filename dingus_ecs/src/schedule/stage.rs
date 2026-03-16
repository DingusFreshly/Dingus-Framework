use crate::schedule::system_trait::IntoSystem;
use super::prelude::*;
use crate::world::World;
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SystemId(usize);
/// A stage is a collection of systems that can be run in parallel if they dont conflict on component access.
pub struct Stage {
    pub name: &'static str,
    systems: Vec<Box<dyn System>>,
    parallel_groups: Vec<Vec<SystemId>>,
    dirty: bool,
}

impl Stage {
    pub fn new(name: &'static str) -> Self {
        Stage { name, systems: Vec::new(), parallel_groups: Vec::new(), dirty: false }
    }

    pub fn with_system<P>(mut self, name: &'static str, sys: impl IntoSystem<P>) -> Self {
        self.systems.push(sys.into_system(name));
        self.dirty = true;
        self
    }

    pub fn initialize(&mut self, world: &World) {
        for s in &mut self.systems { s.initialize(world); }
        self.rebuild_groups();
    }
    /// checks their access patterns so they dont conflict.
    /// only done when systems are added or initialized
    fn rebuild_groups(&mut self) {
        self.parallel_groups.clear();
        for id in 0..self.systems.len() {
            let access = self.systems[id].component_access().clone();
            let mut placed = false;
            'outer: for group in &mut self.parallel_groups {
                for &existing in group.iter() {
                    if self.systems[existing.0].component_access().conflicts_with(&access) {
                        continue 'outer;
                    }
                }
                group.push(SystemId(id));
                placed = true;
                break;
            }
            if !placed { self.parallel_groups.push(vec![SystemId(id)]); }
        }
        self.dirty = false;
    }

    /// Run systems, then flush all CommandBuffers
    pub fn run(&mut self, world: &mut World) {
        if self.dirty { self.rebuild_groups(); }

        for group in &self.parallel_groups {
            if group.len() == 1 {
                unsafe { self.systems[group[0].0].run_unsafe(world); }
            } else {
                // Parallel path (rayon):
                // rayon::scope(|s| { for &sid in group { s.spawn(|_| unsafe { ... }); } });
                // Sequential fallback:
                //TODO! make parallel once resources and component access is guarenteed safe
                for &sid in group {
                    unsafe { self.systems[sid.0].run_unsafe(world); }
                }
            }
        }

        // Flush all command buffers AFTER all systems in this stage have run.
        for s in &mut self.systems {
            s.flush_commands(world);
        }
    }
}

pub struct Schedule {
    stages: Vec<Stage>,
}

impl Schedule {
    pub fn new() -> Self { Schedule { stages: Vec::new() } }
    pub fn add_stage(&mut self, stage: Stage) -> &mut Self { self.stages.push(stage); self }
    pub fn initialize(&mut self, world: &World) { for s in &mut self.stages { s.initialize(world); } }
    pub fn run(&mut self, world: &mut World) { for s in &mut self.stages { s.run(world); } }
}