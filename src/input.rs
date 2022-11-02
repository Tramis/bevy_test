use bevy::prelude::*;

pub struct ClickEvent {
    pub pos: Vec2,
}

// pub fn handle_keyboard(keyboard: Res<Input<KeyCode>>, mut event_writer: EventWriter<ClickEvent>) {}

pub fn handle_cursor(
    mouse_button: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut event_writer: EventWriter<ClickEvent>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        let window = windows.get_primary().expect("no primary window");
        if let Some(cursor_pos) = window.cursor_position() {
            event_writer.send(ClickEvent { pos: cursor_pos })
        }
    }
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickEvent>().add_system(handle_cursor);
    }
}
