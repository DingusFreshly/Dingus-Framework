use super::prelude::*;
use std::marker::PhantomData;
use crate::world::World;
use crate::query::prelude::*;
use crate::fast_bit::FASTBIT_WORDS;
/// A System is a unit of logic that runs each frame, accessing components and resources according to its declared access patterns
pub trait System {//Send + Sync
    fn name(&self) -> &'static str;
    fn component_access(&self) -> &Access;//TODO! add resources access
    fn initialize(&mut self, world: &World);
    /// # Safety: caller must ensure access rules
    unsafe fn run_unsafe(&mut self, world: &World);
    /// Flush any CommandBuffers this system owns into the world
    #[allow(unused_variables)]
    fn flush_commands(&mut self, world: &mut World) {}
}

pub struct FunctionSystem<F, Params: SystemParamTuple> {
    name: &'static str,
    func: F,
    param_states: Option<Params::States>,
    cached_access: Access,
    _phantom: PhantomData<Params>,
}
/// A tuple of system params, e.g. (Query<&A>, ResMut<B>, Commands), with associated state and access info
pub trait SystemParamTuple: 'static {
    /// Stores the initializing information for all the params in the tuple
    type States: 'static;// Send + Sync +
    /// tuple of all the output `Item` types in the tuple
    type Items<'w>;
    fn init_states(world: &World) -> Self::States;
    unsafe fn get_params<'w>(states: &'w mut Self::States, world: &'w World) -> Self::Items<'w>;
    fn component_access(states: &Self::States) -> Access;
    //TODO! implement resources access
    #[allow(unused_variables)]
    fn flush_commands(states: &mut Self::States, world: &mut World) {}
}
/// Blanket impl to convert any function with SystemParam arguments into a System
pub trait IntoSystem<Params> {
    fn into_system(self, name: &'static str) -> Box<dyn System>;
}


macro_rules! impl_system_fn {
    ($($P:ident),*) => {
        // SystemParamTuple impl for tuples of SystemParams eg (Query<&A>, ResMut<B>, Commands)
        #[allow(non_snake_case)]
        impl<$($P: SystemParam + 'static),*> SystemParamTuple for ($($P,)*) {
            type States = ($($P ::State,)*);
            type Items<'w> = ($(<$P as SystemParam>::Item<'w>,)*);
            
            fn init_states(world: &World) -> Self::States { ($($P::init_state(world),)*) }
            #[allow(non_snake_case, unused_variables,unsafe_op_in_unsafe_fn)]
            unsafe fn get_params<'w>(states: &'w mut Self::States, world: &'w World) -> Self::Items<'w> {
                let ($($P,)*) = states;
                ($($P ::get_param($P, world),)*)
            }
            fn component_access(states: &Self::States) -> Access {
                let ($($P,)*) = states;
                let mut access = Access::default();
                $({ 
                    let a = $P::component_access($P);
                    for i in 0..FASTBIT_WORDS { 
                        access.reads.words[i] |= a.reads.words[i]; 
                        
                        access.writes.words[i] |= a.writes.words[i];
                    }
                })*
                access
            }
            // Flush CommandBuffers for any Commands param in the tuple.
            #[allow(non_snake_case, unused_variables)]
            fn flush_commands(states: &mut Self::States, world: &mut World) {
                let ($($P,)*) = states;
                $(
                    $P ::flush_commands($P, world);
                )*
            }
        }
        //Into system
        #[allow(non_snake_case)]
        impl<Func, $($P: SystemParam + 'static),*> IntoSystem<($($P,)*)> for Func
        where
            // The function must be callable with the concrete param types...
            Func: Fn($($P),*) + Send + Sync + 'static,
            // ...AND with the Item<'w> version (the actual reference lifetimes).
            Func: for<'w> Fn($(<$P as SystemParam>::Item<'w>),*),
        {
            fn into_system(self, name: &'static str) -> Box<dyn System> {
                Box::new(FunctionSystem::<Func, ($($P,)*)> {
                    name,
                    func: self,
                    param_states: None,
                    cached_access: Access::default(),
                    _phantom: PhantomData,
                })
            }
        }
        
        #[allow(non_snake_case, unsafe_op_in_unsafe_fn)]
        impl<Func, $($P),*> System
        for FunctionSystem<Func, ($($P,)*)>
        where
            $($P: SystemParam + 'static,)*
            Func: Fn($($P),*) +'static,//+ Send + Sync + 
            Func: for<'w> Fn($(<$P as SystemParam>::Item<'w>),*),
        {
            fn name(&self) -> &'static str { self.name }
            fn component_access(&self) -> &Access { &self.cached_access }
            fn initialize(&mut self, world: &World) {
                let s = <($($P,)*)>::init_states(world);
                self.cached_access = <($($P,)*)>::component_access(&s);
                self.param_states = Some(s);
            }
            unsafe fn run_unsafe(&mut self, world: &World) {
                let s = self.param_states.as_mut().unwrap();
                let ($($P,)*) = <($($P,)*)>::get_params(s, world);
                (self.func)($($P),*);
                
            }
            fn flush_commands(&mut self, world: &mut World) {
                if let Some(s) = &mut self.param_states {
                    <($($P,)*)>::flush_commands(s, world);
                }
            }
        }
    };
}

//impl_system_fn!();
impl_system_fn!(A);
impl_system_fn!(A, B);
impl_system_fn!(A, B, C);
impl_system_fn!(A, B, C, D);
impl_system_fn!(A, B, C, D, E);
impl_system_fn!(A, B, C, D, E, F);
impl_system_fn!(A, B, C, D, E, F, G);
impl_system_fn!(A, B, C, D, E, F, G, H);