use bevy::prelude::*;

use crate::r#move::MoveItem;
use crate::r#move::Position;

#[derive(Component)]
pub struct Collider;

// pub struct CollisionEvent {
//     id_1: Entity,
//     id_2: Entity,
// }

// pub fn collision_check(
//     item_query: Query<(Entity, &Position), With<Collider>>,
//     mut collision_event_writer: EventWriter<CollisionEvent>,
// ) {
//     let items = item_query.into_iter().collect::<Vec<_>>();
//     for i in 0..items.len() {
//         for j in i + 1..items.len() {
//             match items[i].1.distance_cmp(items[j].1.pos, ball::RADIUS) {
//                 utils::DistanceEnum::Lt | utils::DistanceEnum::Zero => {
//                     collision_event_writer.send(CollisionEvent {
//                         id_1: items[i].0,
//                         id_2: items[j].0,
//                     })
//                 }
//                 utils::DistanceEnum::Eq | utils::DistanceEnum::Gt => {
//                     // collide not
//                 }
//             }
//         }
//     }
// }

// pub fn collide(
//     mut item_query: Query<&mut MoveItem, With<Moveable>>,
//     collision_event: EventReader<CollisionEvent>,
// ) {
// }

pub fn collide_with_wall(
    windows: Res<Windows>,
    mut collider: Query<(&Position, &mut MoveItem), With<Collider>>,
) {
    const PADDING: f32 = 10.0;

    let window = windows.get_primary().expect("no primary window");
    let top = window.height();
    let right = window.width();

    for (pos, mut item) in &mut collider {
        let old_velocity = item.v;
        if pos.x <= 0.0 + PADDING {
            item.v.x = (1.0_f32).max(-old_velocity.x);
        } else if pos.x >= right - PADDING {
            item.v.x = (-1.0_f32).min(-old_velocity.x);
        }

        if pos.y <= 0.0 + PADDING {
            item.v.y = (1.0_f32).max(-old_velocity.y)
        } else if pos.y >= top - PADDING {
            item.v.y = (-1.0_f32).min(-old_velocity.y)
        }
    }
}

pub struct CollidePlugin;
impl Plugin for CollidePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collide_with_wall.after(crate::r#move::force));
    }
}
