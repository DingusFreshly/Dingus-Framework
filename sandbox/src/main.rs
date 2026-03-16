use std::thread::Thread;
use dingus_ecs::prelude::*;
use linkme;


use dingus_builtins::{systems::*};
use dingus_builtins::resources::{Renderer, Time};
use dingus_builtins::components::{Position, Velocity, Player, Size, Health};
use dingus_builtins::generated::*;
use dingus_ecs::prelude::*;
use std::time::Instant;
use dingus_builtins::consts::*;

fn main() {
    let mut world = World::new();
    world.insert_resource(
        Time {
            time: 0.0,
            delta: 0.0,
            last_frame_time: Instant::now(),
            start_time: Instant::now(),
        }
    );
    
    world.insert_resource(
        Renderer::new("Dingus", SCREEN_WIDTH, SCREEN_HEIGHT)
    );
    
    world.spawn::<PlayerArchetype>(PlayerBundle {
        position: Position(0.0, 0.0),
        player: Player{
            last_shot: Instant::now(),
            can_shoot: true,
        },
        velocity: Velocity(0.0, 0.0),
        health: Health(PLAYER_HEALTH),
    });
    
    let mut schedule = Schedule::new();
    let stage1 = Stage::new("Spawn")
        .with_system("asteroid_spawn", asteroid_spawn_system)
        .with_system("collision", collision_system);

    let stage2 = Stage::new("Update")
        .with_system("movement", movement_system)
        .with_system("update", update_system)
        .with_system("asteroid_bullet_collision", asteroid_bullet_collision_system);
    let stage3 = Stage::new("Render")
        .with_system("render", render_system);


    schedule.add_stage(
        stage1
    );
    schedule.add_stage(
        stage2
    );
    schedule.add_stage(
        stage3
    );
    schedule.initialize(&world);

    loop {
        schedule.run(&mut world);
        
    }
    
    println!("Hello, world!");
}
