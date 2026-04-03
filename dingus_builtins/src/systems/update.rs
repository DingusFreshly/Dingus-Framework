use std::collections::HashSet;
use std::time::{Duration, Instant};
use dingus_ecs::internal::{Commands, Entity};
use dingus_ecs::prelude::{Query, Res, ResMut,ArchetypeQuery};
use crate::components::{Health, Player, Position, Velocity, Size};
use crate::resources::{Renderer, Time};
use minifb::Key;
use crate::consts::*;
use crate::generated::*;
use rand;

pub fn collision_system(
    player: ArchetypeQuery<PlayerArchetype, (&Position, &mut Health, Entity)>,
    asteroids: ArchetypeQuery<AsteroidArchetype, (&Position, &mut Health, &Size, Entity)>,
    mut commands: Commands,
) {
    for (p_pos, p_health, p_entity_id) in player.iter() {
        for (a_pos, a_health, a_size, entity_id) in asteroids.iter() {
            let dx = p_pos.0 - a_pos.0;
            let dy = p_pos.1 - a_pos.1;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance < a_size.0 as f32 {
                println!("Player hit by asteroid!");
                // Handle player-asteroid collision
                p_health.0 -= ASTEROID_DAMAGE;
                commands.despawn(entity_id);
            }
            if a_health.0 <= 0 {
                println!("Asteroid destroyed!");
                commands.despawn(p_entity_id);
            }
        }
        if p_health.0 <= 0 {
            println!("Player died!");
        }
    }
    
}
pub fn asteroid_bullet_collision_system(
    bullets: ArchetypeQuery<BulletArchetype, &Position>,
    asteroids: ArchetypeQuery<AsteroidArchetype, (&Position, &mut Health, &Size, Entity)>,
    mut commands: Commands,
) {
    let mut destroyed = HashSet::new();
    
    for b_pos in bullets.iter() {
        for (a_pos, a_health, a_size, entity_id) in asteroids.iter() {
            let dx = b_pos.0 - a_pos.0;
            let dy = b_pos.1 - a_pos.1;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance < a_size.0 as f32 {
                println!("Bullet hit asteroid!");
                a_health.0 -= BULLET_DAMAGE;
            }
            if a_health .0 <= 0 {
                println!("Asteroid destroyed!");
                if !destroyed.contains(&entity_id) {
                    destroyed.insert(entity_id);
                    commands.despawn(entity_id);
                }
            }
        }
    }
}

pub fn asteroid_spawn_system(mut commands: Commands, time: Res<Time>) {
    println!("Time: {}", time.time);
    if time.time % 3.0 < 0.1 {
        println!("Spawning asteroid");
        //spawn an asteroid that starts at the edge of the screen
        let mut rng = rand::thread_rng();
        let edge = rand::Rng::gen_range(&mut rng, 0..4);
        let (x, y) = match edge {
            0 => (0.0, rand::Rng::gen_range(&mut rng, 0.0..SCREEN_HEIGHT as f32)),
            1 => (SCREEN_WIDTH as f32, rand::Rng::gen_range(&mut rng, 0.0..SCREEN_HEIGHT as f32)),
            2 => (rand::Rng::gen_range(&mut rng, 0.0..SCREEN_WIDTH as f32), 0.0),
            _ => (rand::Rng::gen_range(&mut rng, 0.0..SCREEN_WIDTH as f32), SCREEN_HEIGHT as f32),
        };
        let angle = rand::Rng::gen_range(&mut rng, 0.0..std::f32::consts::TAU);
        let speed = rand::Rng::gen_range(&mut rng, ASTEROID_SPEED_RANGE);
        let vel = (angle.cos() * speed , angle.sin() * speed);
        let size = rand::Rng::gen_range(&mut rng, ASTEROID_SIZE_RANGE);
        let health = rand::Rng::gen_range(&mut rng, ASTEROID_HEALTH_RANGE);
        
        commands.spawn::<AsteroidArchetype>(AsteroidArchetypeBundle {
            position: Position(x, y),
            velocity: Velocity(vel.0, vel.1),
            health: Health(health),
            size: Size(size),
        });
    }
}

pub fn movement_system(
    query: Query<(&mut Position, &mut Velocity, Option<&mut Player>)>,
    time: Res<Time>,
    renderer: Res<Renderer>,
    mut commands: Commands,
) {
    if time.time % 2.0 < 0.1 {
        println!("Second");

    }
    let dt = time.delta;

    println!("{}", query.iter().count());
    
    for (pos, vel,  plr) in query.iter() {
        if let Some( p) = plr {
            if renderer.is_key_down(Key::W) {
                vel.1 -= PLAYER_SPEED  * dt;
            }
            if renderer.is_key_down(Key::S) {
                vel.1 += PLAYER_SPEED  * dt;
            }
            if renderer.is_key_down(Key::A) {
                vel.0 -= PLAYER_SPEED* dt;
            }
            if renderer.is_key_down(Key::D) {
                vel.0 += PLAYER_SPEED  * dt;
            }
            
            if p.last_shot + Duration::from_secs_f32(FIRE_RATE) < Instant::now() {
                p.last_shot = Instant::now();
                
                let mut vel = Velocity(0.0,0.0);
                if renderer.is_key_down(Key::Left) {
                    vel .0 -= BULLET_SPEED;
                } else if renderer.is_key_down(Key::Right) {
                    vel.0 += BULLET_SPEED ;
                } else if renderer.is_key_down(Key::Up) {
                    vel.1 -= BULLET_SPEED ;
                } else if renderer.is_key_down(Key::Down) {
                    vel.1 += BULLET_SPEED  ;
                }
                if vel.0 != 0.0 || vel.1 != 0.0 {
                    commands.spawn::<BulletArchetype>(BulletArchetypeBundle {
                        position: Position(pos.0, pos.1),
                        velocity: Velocity(vel.0 * BULLET_SPEED, vel.1 * BULLET_SPEED),
                    });
                }   
            }
        }
        
        //println!("{:?}", pos);
        pos.0 += vel.0 * dt;
        pos.1 += vel.1 * dt;
    }
}

pub fn update_system(mut res: ResMut<Time>) {
    let now = Instant::now();
    res.delta = (now - res.last_frame_time).as_secs_f32();
    res.time += res.delta;
    res.last_frame_time = now;

    //println!("{}", res.time);
}

