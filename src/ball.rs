
use bevy::prelude::*;

use crate::input::ClickEvent;

pub const RADIUS: f32 = 2.0;

#[derive(Component)]
pub struct Ball{
    
}

pub fn add_ball(
    mut event_reader: EventReader<ClickEvent>
){
    for click_event in event_reader.iter(){
        // only spawn one. Im not sure it's neccessary
        println!("spawn at {}", click_event.pos);
    }
}

pub struct BallPlugin;
impl Plugin for BallPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(add_ball);
    }
}