mod renderer;

pub use renderer::Renderer;

use std::time::Instant;
#[derive(Clone, Copy, Debug)]
pub struct Time{
    pub time: f32,
    pub delta: f32,
    pub last_frame_time: Instant,
    pub start_time: Instant,
}