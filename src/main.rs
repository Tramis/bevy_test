use bevy::prelude::*;

mod ball;
mod collide;
mod exclude;
mod input;
mod r#move;
mod utils;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(exclude::ExcludePlugin)
        .add_plugin(r#move::MovePlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(ball::BallPlugin)
        .run();
}
