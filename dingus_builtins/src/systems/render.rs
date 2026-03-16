use dingus_ecs::prelude::{Query, ResMut};
use crate::components::{Player, Position,Size};
use crate::consts::{ASTEROID_COLOUR, BULLET_SIZE, PLAYER_COLOUR, PLAYER_SIZE, BULLET_COLOUR};
use crate::resources::Renderer;

pub fn render_system(mut renderer: ResMut<Renderer>, query: Query<(&Position, Option<&Player>, Option<&Size>)>) {
    let renderer = &mut *renderer;

    renderer.clear(0);
    println!("Rendering {} objects", query.iter().count());
    for (pos, player, size) in query.iter() {
        let (color, size) = if let Some(_) = player {
            (PLAYER_COLOUR, PLAYER_SIZE)
        } else {
            if let Some(s) = size {
                (ASTEROID_COLOUR, s.0)
            } else {
                (BULLET_COLOUR, BULLET_SIZE)
            }
        };
        renderer.draw_circle(pos.0 as i32, pos.1 as i32, size, color);
    }
    renderer.update();
}