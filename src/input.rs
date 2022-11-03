use bevy::prelude::*;

pub enum EventType {
    Press,
    Release,
}
pub struct MouseEvent {
    pub mouse_button: MouseButton,
    pub pos: Vec2,
    pub event_type: EventType,
}

// pub fn handle_keyboard(keyboard: Res<Input<KeyCode>>, mut event_writer: EventWriter<ClickEvent>) {}

pub fn handle_cursor(
    mouse_button: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut event_writer: EventWriter<MouseEvent>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().expect("no primary window");
        if let Some(cursor_pos) = window.cursor_position() {
            if cursor_pos.x > 0.0
                && cursor_pos.y > 0.0
                && cursor_pos.x < window.width()
                && cursor_pos.y < window.height()
            {
                event_writer.send(MouseEvent {
                    pos: cursor_pos,
                    mouse_button: MouseButton::Left,
                    event_type: EventType::Press,
                })
            }
        }
    } else if mouse_button.just_released(MouseButton::Left) {
        let window = windows.get_primary().expect("no primary window");
        if let Some(cursor_pos) = window.cursor_position() {
            if cursor_pos.x > 0.0
                && cursor_pos.y > 0.0
                && cursor_pos.x < window.width()
                && cursor_pos.y < window.height()
            {
                event_writer.send(MouseEvent {
                    pos: cursor_pos,
                    mouse_button: MouseButton::Left,
                    event_type: EventType::Release,
                })
            }
        }
    } else if mouse_button.just_pressed(MouseButton::Right){
        let window = windows.get_primary().expect("no primary window");
        if let Some(cursor_pos) = window.cursor_position() {
            println!("right click: {}", cursor_pos)
        }
    }
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MouseEvent>().add_system(handle_cursor);
    }
}
