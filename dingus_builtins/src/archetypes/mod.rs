use dingus_ecs::prelude::ComponentTrait;
//
use mlua::Lua;
use dingus_ecs::internal::{Archetype, ArchetypeBundle, ArchetypeMarker, FastBit};
use crate::components::{Health, Velocity};
use crate::generated::*;
use dingus_ecs::prelude::archetype;



#[archetype(
    name = Test,
    components = [Health, Velocity],
    properties = [
        @Health Name : String = component.0,
        @Health Health : Int = component.0,
        @Velocity X : Float = component.0,
    ],
    readonly = [Name]
)]
pub struct Test();



impl Test {

}