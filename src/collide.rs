use bevy::prelude::*;

use crate::r#move::{MoveItem, Moveable};
use crate::utils::{self, DistanceSquare};
use crate::{
    ball::{self, Ball},
    r#move::Position,
    utils::DistanceCmp,
};

#[derive(Component)]
pub struct Collider;

pub struct CollisionEvent {
    id_1: Entity,
    id_2: Entity,
}

pub fn collision_check(
    item_query: Query<(Entity, &Position), With<Collider>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    let items = item_query.into_iter().collect::<Vec<_>>();
    for i in 0..items.len() {
        for j in i + 1..items.len() {
            match items[i].1.distance_cmp(&items[j].1, ball::RADIUS) {
                utils::DistanceEnum::Lt | utils::DistanceEnum::Zero => {
                    collision_event_writer.send(CollisionEvent {
                        id_1: items[i].0,
                        id_2: items[j].0,
                    })
                }
                utils::DistanceEnum::Eq | utils::DistanceEnum::Gt => {
                    // collide not
                }
            }
        }
    }
}

pub fn collide(
    mut item_query: Query<&mut MoveItem, With<Moveable>>,
    collision_event: EventReader<CollisionEvent>,
) {
}
