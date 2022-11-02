use crate::r#move::force;
use crate::r#move::{MoveItem, Position};
use bevy::prelude::*;

#[derive(Component)]
pub struct Exclusion;

/// mutually exclude
pub fn exclude_force(mut items: Query<(&Position, &mut MoveItem), With<Exclusion>>) {
    const EXCLUSIVE_FORCE: f32 = 1.0;
    const MAX_EXCLUSIVE_FORCE: f32 = 50.0;

    let (pos_vec, mut item_vec): (Vec<&Position>, Vec<Mut<MoveItem>>) = items.iter_mut().unzip();
    for i in 0..pos_vec.len() {
        for j in i + 1..pos_vec.len() {
            let pos_a = pos_vec[i];
            let pos_b = pos_vec[j];

            let m_a = item_vec[i].m;
            let m_b = item_vec[j].m;

            let distance_square = pos_a.distance_squared(pos_b.pos);

            // recompute force
            if distance_square != 0.0 {
                item_vec[i].f += (pos_a.pos - pos_b.pos)
                    * MAX_EXCLUSIVE_FORCE.min(EXCLUSIVE_FORCE / distance_square)
                    * m_a;
                item_vec[j].f += (pos_b.pos - pos_a.pos)
                    * MAX_EXCLUSIVE_FORCE.min(EXCLUSIVE_FORCE / distance_square)
                    * m_b;
            }
        }
    }
}

pub fn exclude_border(
    windows: Res<Windows>,
    mut items: Query<(&Position, &mut MoveItem), With<Exclusion>>,
) {
    const DIVIDER: f32 = 50000.0;
    let window = windows.get_primary().expect("no primary window");
    let top = window.height();
    let right = window.width();

    let top_mid = top / 2.0;
    let right_mid = right / 2.0;

    for (pos, mut item) in &mut items {
        let v_x = (right_mid - pos.x) / DIVIDER;
        let v_y = (top_mid - pos.y) / DIVIDER;

        // multiply the value remaining direction
        item.f += Vec2::new(v_x, v_y);
    }
}

pub struct ExcludePlugin;
impl Plugin for ExcludePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(exclude_force.after(force))
            .add_system(exclude_border.after(force));
    }
}
