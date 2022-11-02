use std::ops::Deref;

use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub pos: Vec2,
}

impl Deref for Position {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

#[derive(Component, Debug)]
pub struct MoveItem {
    pub m: f32,
    pub v: Vec2,
    pub a: Vec2,
    pub f: Vec2,
}

impl Default for MoveItem {
    fn default() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self {
            m: 1.0,
            v: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
            a: Vec2::new(0.0, 0.0),
            f: Vec2::new(0.0, 0.0),
        }
    }
}

pub fn movement(mut move_items: Query<(&MoveItem, &mut Position)>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for (move_item, mut pos) in move_items.iter_mut() {
        pos.pos += move_item.v * delta;
    }
}

pub fn accelerate(mut items: Query<&mut MoveItem>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for mut item in &mut items {
        // it seems normal not to multiply time delta
        let increment = item.a;
        item.v += increment * delta;
    }
}

pub fn force(mut items: Query<(Entity, &mut MoveItem)>) {
    for (_id, mut item) in &mut items {
        // println!("id: {}: {item:?}", _id.id());
        let increment = item.f / item.m;
        item.a += increment;
        // recompute force every gap
        item.f = Vec2::new(0.0, 0.0);
    }
}

pub struct MovePlugin;
impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement.before(accelerate))
            .add_system(accelerate.before(force))
            .add_system(force);
    }
}
