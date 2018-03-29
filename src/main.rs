extern crate specs;
use specs::{Component, VecStorage};

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[macro_use] 
extern crate specs_derive;

#[derive(Debug, Component)]
#[component(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

use specs::{System, Join, WriteStorage, ReadStorage, Entities};

struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem  {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.x += velocity.x;
            position.y += velocity.y;
        }
    }
}

struct RenderSystem;

impl<'a> System<'a> for RenderSystem  {
    type SystemData = (Entities<'a>, ReadStorage<'a, Position>);

    fn run(&mut self, (entities, positions): Self::SystemData) {
        for (entity, position) in (&*entities, &positions).join() {
            println!("Entity {e} at ({x}, {y})", e = entity.id(), x = position.x, y = position.y);
        }
    }
}

use specs::{DispatcherBuilder, World};

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    world.create_entity()
        .with(Position { x: 0., y: 0. })
        .build();
    world.create_entity()
        .with(Position { x: 1., y: 0. })
        .with(Velocity { x: 0.1, y: 0.1 })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .add(PhysicsSystem, "physics", &[])
        .add(RenderSystem, "render", &["physics"])
        .build();
    loop {
        dispatcher.dispatch(&mut  world.res);
    }
}
