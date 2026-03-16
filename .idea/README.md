# Dingus Framework
ill  finish ts later

## Tutorial
### Getting started
Create a `World` struct
```rust
let mut world = World::new();
```
Before you can use resources 
Insert resource method:
```rust
world.insert_resource(
    Time {
        time: 0.0,
        delta: 0.0,
        last_frame_time: Instant::now(),
        start_time: Instant::now(),
        }
    );
```
`Time` here is the resource, whatever struct you insert as a resource must be registered in the macro (more about that later)

Spawning an archetyp outside a system:
```rust
 world.spawn::<PlayerArchetype>(PlayerBundle {
    position: Position(0.0, 0.0),
    player: Player{
        last_shot: Instant::now(),
        can_shoot: true,
    },
    velocity: Velocity(0.0, 0.0),
    health: Health(PLAYER_HEALTH),
});
```
The 
`dingus_builtins` contains all the game logic code, this is where most things will be for now.

