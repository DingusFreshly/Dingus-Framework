use std::time::Instant;
use dingus_ecs::prelude::Component;

#[derive(Clone, Copy, Debug, Component)]
pub struct Position(
    #[dingus_export(readonly)]
    pub f32, pub f32
);
#[derive(Clone, Copy, Debug, Component)]
pub struct Velocity(pub f32, pub f32);
#[derive(Clone, Copy, Debug, Component)]
pub struct Health(pub i32);

#[derive(Clone, Copy, Debug, Component)]
pub struct Player{
    pub last_shot: Instant,
    pub can_shoot: bool,
}
#[derive(Clone, Copy, Debug, Component)]
pub struct Size(pub i32);

