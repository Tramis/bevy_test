use crate::r#move::force;
use crate::{
    r#move::{MoveItem, Moveable, Position},
    utils::DistanceSquare,
};
use bevy::prelude::*;

/// mutually exclude
pub fn exclude_force(mut items: Query<(&Position, &mut MoveItem), With<Moveable>>) {
    const EXCLUSIVE_FORCE: f32 = 1.0;
    const MAX_EXCLUSIVE_FORCE: f32 = 50.0;

    let (pos_vec, mut item_vec): (Vec<&Position>, Vec<Mut<MoveItem>>) = items.iter_mut().unzip();
    for i in 0..pos_vec.len() {
        for j in i + 1..pos_vec.len() {
            let pos_a = pos_vec[i];
            let pos_b = pos_vec[j];

            let distance_square = pos_a.distance_square(&pos_b);

            // recompute force
            if distance_square != 0.0 {
                item_vec[i].f += (pos_a.pos - pos_b.pos)
                    * MAX_EXCLUSIVE_FORCE.min(EXCLUSIVE_FORCE / distance_square);
                item_vec[j].f += (pos_b.pos - pos_a.pos)
                    * MAX_EXCLUSIVE_FORCE.min(EXCLUSIVE_FORCE / distance_square);
            }
        }
    }
}

pub fn exclude_border() {}

pub struct ExcludePlugin;
impl Plugin for ExcludePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(exclude_force.after(force))
            .add_system(exclude_border.after(force));
    }
}
