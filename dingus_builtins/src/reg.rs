
use dingus_ecs::prelude::*;

use crate::resources::{Time, Renderer};
use crate::components::{Position,Health,Player,Velocity, Size};

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

include_archetypes! {
    Asteroid [
        Size,
        Position,
        Velocity,
        Health,
    ]
    Bullet [
        Position,
        Velocity,
    ]
    Player [
        Player,
        Position,
        Velocity,
        Health,
    ]
}