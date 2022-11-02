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

#[derive(Component)]
pub struct MoveItem {
    pub m: f32,
    pub v: Vec2,
    pub a: Vec2,
    pub f: Vec2,
}

#[derive(Component)]
pub struct Moveable;

pub fn movement(mut move_items: Query<(&MoveItem, &mut Position), With<Moveable>>) {
    for (move_item, mut pos) in move_items.iter_mut() {
        pos.pos += move_item.v;
    }
}

pub fn accelerate(mut items: Query<&mut MoveItem, With<Moveable>>) {
    for mut item in &mut items {
        // it seems normal not to multiply time delta
        let increment = item.a;
        item.v += increment;
    }
}

pub fn force(mut items: Query<&mut MoveItem, With<Moveable>>) {
    for mut item in &mut items {
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
