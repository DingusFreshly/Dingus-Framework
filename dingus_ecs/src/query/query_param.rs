use crate::archetype::prelude::*;
use crate::component::prelude::*;
use crate::world::World;
use std::marker::PhantomData;
use crate::Entity;
///Types that can be queried must implement this trait
pub trait QueryParam {
    type Item<'a>;
    type State;
    fn init_state(world: &World) -> Self::State;
    fn matches_archetype(archetype: &Archetype) -> bool;
    unsafe fn fetch_ptrs(archetype: &Archetype, row: usize) -> Self::Item<'_>;
}

// &T
impl<T: ComponentTrait> QueryParam for &T {
    type Item<'a> = &'a T;
    type State = ();
    fn init_state(_: &World) -> () { () }
    #[inline] fn matches_archetype(a: &Archetype) -> bool { a.has_component_id(T::component_type_id()) }
    #[inline] unsafe fn fetch_ptrs(a: &Archetype, row: usize) -> &T {
        a.columns[a.column_of(T::component_type_id())].get::<T>(row)
    }
}

// &mut T
impl<T: ComponentTrait> QueryParam for &mut T {
    type Item<'a> = &'a mut T;
    type State = ();
    fn init_state(_: &World) -> () { () }
    #[inline] fn matches_archetype(a: &Archetype) -> bool { a.has_component_id(T::component_type_id()) }
    #[inline] unsafe fn fetch_ptrs(a: &Archetype, row: usize) -> &mut T {
        // Safety: scheduler guarantees exclusive access to this column.
        let col = &a.columns[a.column_of(T::component_type_id())] as *const _ as *mut ComponentStorage;
        (*col).get_mut::<T>(row)
    }
}

/// Returns the item only if it exists
impl<T: ComponentTrait> QueryParam for Option<&T> {
    type Item<'a> = Option<&'a T>;
    type State = ();
    fn init_state(_: &World) -> () { () }
    #[inline] fn matches_archetype(_: &Archetype) -> bool { true }
    #[inline] unsafe fn fetch_ptrs(a: &Archetype, row: usize) -> Option<&T> {
        a.column_index.get(&T::component_type_id())
            .map(|&ci| a.columns[ci].get::<T>(row))
    }
}
///returns the entity id also, always guarenteed to exist
impl QueryParam for Entity {
    type Item<'a> = Entity;
    type State = ();
    fn init_state(_: &World) -> () { () }
    #[inline] fn matches_archetype(_: &Archetype) -> bool { true }
    #[inline] unsafe fn fetch_ptrs(a: &Archetype, row: usize) -> Entity {
        a.entities[row]
    }
}
impl<T: ComponentTrait> QueryParam for Option<&mut T> {
    type Item<'a> = Option<&'a mut T>;
    type State = ();
    fn init_state(_: &World) -> () { () }
    #[inline] fn matches_archetype(_: &Archetype) -> bool { true }
    #[inline] unsafe fn fetch_ptrs(a: &Archetype, row: usize) -> Option<&mut T> {
        a.column_index.get(&T::component_type_id())
            .map(|&ci| a.columns[ci].get_mut::<T>(row))
    }
}

// With<T> filter
pub struct With<T: ComponentTrait>(PhantomData<T>);
impl<T: ComponentTrait> QueryParam for With<T> {
    type Item<'a> = ();
    type State = ();
    fn init_state(_: &World) -> () { () }
    #[inline] fn matches_archetype(a: &Archetype) -> bool { a.has_component_id(T::component_type_id()) }
    #[inline] unsafe fn fetch_ptrs(_: &Archetype, _: usize) -> () { () }
}

// Without<T> filter
pub struct Without<T: ComponentTrait>(PhantomData<T>);
impl<T: ComponentTrait> QueryParam for Without<T> {
    type Item<'a> = ();
    type State = ();
    fn init_state(_: &World) -> () { () }
    #[inline] fn matches_archetype(a: &Archetype) -> bool { !a.has_component_id(T::component_type_id()) }
    #[inline] unsafe fn fetch_ptrs(_: &Archetype, _: usize) -> () { () }
}

// Tuple impls
macro_rules! impl_query_param {
    ($($T:ident),*) => {
        impl<$($T: QueryParam),*> QueryParam for ($($T,)*) {
            type Item<'a> = ($($T::Item<'a>,)*);
            type State = ($($T::State,)*);
            fn init_state(world: &World) -> Self::State { ($($T::init_state(world),)*) }
            #[inline] fn matches_archetype(a: &Archetype) -> bool { $($T::matches_archetype(a))&&* }
            #[inline] unsafe fn fetch_ptrs(a: &Archetype, row: usize) -> Self::Item<'_> {
                ($($T::fetch_ptrs(a, row),)*)
            }
        }
    };
}
impl_query_param!(A);
impl_query_param!(A, B);
impl_query_param!(A, B, C);
impl_query_param!(A, B, C, D);
impl_query_param!(A, B, C, D, E);
impl_query_param!(A, B, C, D, E, F);
impl_query_param!(A, B, C, D, E, F, G);
impl_query_param!(A, B, C, D, E, F, G, H);