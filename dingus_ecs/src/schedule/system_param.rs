use super::prelude::*;
use crate::archetype::prelude::*;
use crate::world::World;
use crate::query::prelude::*;
use crate::command::prelude::*;
use crate::resource::prelude::{ResMut, Res};
use crate::resource::ResourceTrait;
/// A parameter to a system function, which can be a query, command buffer, resource, etc.
pub trait SystemParam: Sized {
    type State:'static;//Send + Sync +
    type Item<'w>: SystemParam;
    fn init_state(world: &World) -> Self::State;
    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w World) -> Self::Item<'w>;
    fn component_access(state: &Self::State) -> Access;
    fn flush_commands(state: &mut Self::State, world: &mut World) {
        let _ = (state, world); // default: no-op
    }
}

impl SystemParam for Commands<'_> {
    type State = CommandBuffer;
    type Item<'w> = Commands<'w>;

    fn init_state(_world: &World) -> CommandBuffer { CommandBuffer::new() }

    unsafe fn get_param<'w>(state: &'w mut CommandBuffer, _world: &'w World) -> Commands<'w> {
        Commands { buffer: state }
    }

    fn component_access(_: &CommandBuffer) -> Access { Access::default() }

    fn flush_commands(state: &mut CommandBuffer, world: &mut World) {
        state.flush(world);
    }
}

impl<Q: QueryParam + 'static> SystemParam for Query<'_, Q> {
    type State = QueryState<Q>;
    type Item<'w> = Query<'w, Q>;

    fn init_state(world: &World) -> QueryState<Q> {
        QueryState::new(world) // built once, never updated
    }

    unsafe fn get_param<'w>(state: &'w mut QueryState<Q>, world: &'w World) -> Query<'w, Q> {
        Query::new(world, state)
    }

    fn component_access(state: &QueryState<Q>) -> Access {
        state.component_access.clone()
    }
}

impl<A: ArchetypeMarker, Q: QueryParam + 'static> SystemParam for ArchetypeQuery<'_, A, Q> {
    type State = ();
    type Item<'w> = ArchetypeQuery<'w, A, Q>;

    fn init_state(_world: &World) -> () { () }

    unsafe fn get_param<'w>(_state: &'w mut (), world: &'w World) -> ArchetypeQuery<'w, A, Q> {
        ArchetypeQuery::new(world)
    }

    fn component_access(_state: &()) -> Access {
        // TODO! Build access from A::COMPONENT_SET and Qs write/read intent.
        Access::default()
    }
}

impl <R: ResourceTrait> SystemParam for Res<'_, R> {
    type State = ();
    type Item<'w> = Res<'w, R>;

    fn init_state(_world: &World) -> () { () }

    unsafe fn get_param<'w>(_state: &'w mut (), world: &'w World) -> Res<'w, R> {
        Res { value: world.resources.get::<R>().expect("Resource not found") }
    }

    fn component_access(_state: &()) -> Access { Access::default() }
}
impl <R: ResourceTrait> SystemParam for ResMut<'_, R> {
    type State = ();
    type Item<'w> = ResMut<'w, R>;

    fn init_state(_world: &World) -> () { () }

    unsafe fn get_param<'w>(_state: &'w mut (), world: &'w World) -> ResMut<'w, R> {
        ResMut { value: world.resources.get_mut::<R>().expect("Resource not found") }
    }

    fn component_access(_state: &()) -> Access { Access::default() }
}