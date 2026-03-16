use std::time::Instant;

#[derive(Clone, Copy, Debug)]
pub struct Position(pub f32, pub f32);
#[derive(Clone, Copy, Debug)]
pub struct Velocity(pub f32, pub f32);
#[derive(Clone, Copy, Debug)]
pub struct Health(pub i32);

#[derive(Clone, Copy, Debug)]
pub struct Player{
    pub last_shot: Instant,
    pub can_shoot: bool,
}
#[derive(Clone, Copy, Debug)]
pub struct Size(pub i32);

#[derive(Clone, Copy, Debug)]
pub struct Damage(pub i32);