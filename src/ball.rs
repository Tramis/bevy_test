use bevy::prelude::*;

use crate::{
    input::{EventType, MouseEvent},
    r#move::{self, Position},
    utils::random_color,
};

pub const RADIUS: f32 = 12.0;
pub const DEVIDER: f32 = 2.0;

use bevy_prototype_lyon::prelude as lyon;

#[derive(Component)]
pub struct Ball;

pub fn add_ball(
    mut commands: Commands,
    mut event_reader: EventReader<MouseEvent>,
    mut ball_shooter: ResMut<BallShooter>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().expect("no primary window");
    let v_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    for click_event in event_reader.iter() {
        println!("spawn at {}", click_event.pos);

        if let EventType::Press = click_event.event_type {
            ball_shooter.start_from(click_event.pos);
        } else {
            let velocity = match ball_shooter.shoot(click_event.pos) {
                Some(v) => v,
                None => break,
            };

            let shape = lyon::shapes::Circle {
                center: default(),
                radius: RADIUS,
            };

            commands
                .spawn_bundle(lyon::GeometryBuilder::build_as(
                    &shape,
                    lyon::DrawMode::Fill(lyon::FillMode::color(random_color())),
                    Transform {
                        translation: (window.cursor_position().unwrap() - v_center).extend(0.0),
                        ..Transform::default()
                    },
                ))
                .insert(r#move::MoveItem {
                    m: 0.1,
                    v: velocity / DEVIDER,
                    ..Default::default()
                })
                .insert(Position {
                    pos: click_event.pos,
                })
                .insert(Ball)
                .insert(crate::collide::Collider)
                .insert(crate::exclude::Exclusion);
        }

        // only spawn one. Im not sure it's neccessary
        break;
    }
}

pub fn move_ball(mut balls: Query<(&mut Transform, &Position), With<Ball>>, windows: Res<Windows>) {
    let window = windows.get_primary().expect("no primary window");
    let v_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    for (mut ball, ball_pos) in &mut balls {
        *ball = ball.with_translation((ball_pos.pos - v_center).extend(0.0));
    }
}

pub struct BallShooter {
    pub start_pos: Option<Vec2>,
}

impl BallShooter {
    pub fn start_from(&mut self, start_pos: Vec2) {
        self.start_pos = Some(start_pos)
    }

    pub fn shoot(&mut self, end_pos: Vec2) -> Option<Vec2> {
        self.start_pos.take().map(|start_pos| end_pos - start_pos)
    }
}

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BallShooter { start_pos: None })
            .add_system(add_ball)
            .add_system(move_ball);
    }
}
