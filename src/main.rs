use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

mod ball;
mod collide;
mod exclude;
mod input;
mod r#move;
// mod test;
mod utils;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(1.0, 1.0, 1.0)),
        },
        ..Default::default()
    });
}

fn main() {
    App::new()
        // .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_prototype_lyon::prelude::ShapePlugin)
        .add_plugin(exclude::ExcludePlugin)
        .add_plugin(collide::CollidePlugin)
        .add_plugin(r#move::MovePlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(ball::BallPlugin)
        .add_startup_system(setup)
        .run();
}
