
use dingus_ecs::prelude::*;

use crate::resources::{Time, Renderer};
use crate::components::{Position,Health,Player,Velocity, Size};
use crate::archetypes::Test;

include_resources!{
    Time,
    Renderer,
}


include_components!{
    Position,
    Health,
    Player,
    Velocity,
    Size,
}

//archetypes are either defined by passing in a struct, that implements archetype marker, or a table as shown here, where it will derive it
include_archetypes! {
     AsteroidArchetype [
        Size,
        Position,
        Velocity,
        Health,

    ],

    Test,

    BulletArchetype [
        Position,
        Velocity,
    ]
    PlayerArchetype [
        Player,
        Position,
        Velocity,
        Health,
    ]
}