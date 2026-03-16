use std::ops::RangeInclusive;

pub const SCREEN_WIDTH: usize = 800;
pub const SCREEN_HEIGHT: usize = 600;

pub const PLAYER_SIZE: i32 = 30;
pub const PLAYER_HEALTH: i32 = 100;
pub const PLAYER_COLOUR : u32 = 0xFF0000;
pub const PLAYER_SPEED : f32 = 90.0;
pub const PLAYER_MAX_SPEED : f32 = 100.0;


pub const BULLET_SPEED : f32 = 40.0;
pub const BULLET_SIZE : i32 = 10;
pub const BULLET_COLOUR : u32 = 0xFFFF00;
pub const FIRE_RATE : f32 = 0.3; 
pub const BULLET_DAMAGE : i32 = 1;

pub const ASTEROID_HEALTH_RANGE : RangeInclusive<i32> = 1..=4;
pub const ASTEROID_SIZE_RANGE : RangeInclusive<i32> = 15 ..=45;
pub const ASTEROID_COLOUR : u32 = 0x00FF00;
pub const ASTEROID_SPEED_RANGE : RangeInclusive<f32> = 40.0..=50.0;
pub const ASTEROID_DAMAGE : i32 = 1;
