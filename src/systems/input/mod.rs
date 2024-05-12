use bevy::{input::ButtonState, prelude::*, window::PrimaryWindow};

use crate::components::{player::{PlayerCamera, PlayerCursor}, Position};

pub struct LithiumInputSystem;

impl Plugin for LithiumInputSystem {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, keyboard_events_update)
            .add_systems(Update, player_cursor_update);
    }
}

pub fn keyboard_events_update(
    mut keyboard_events: EventReader<bevy::input::keyboard::KeyboardInput>,
    mut player_query: Query<
        &mut crate::components::player::PlayerKeyboardControls,
        With<crate::components::player::Player>,
    >,
) {
    let mut controls = player_query.single_mut();

    for event in keyboard_events.read() {
        if let Some(k) = controls
            .dict
            .values_mut()
            .find(|k| k.key_code == event.key_code)
        {
            k.is_pressed = match event.state {
                ButtonState::Pressed => true,
                ButtonState::Released => false,
            }
        }
    }
}

pub fn player_cursor_update(
    mut cursor_query: Query<&mut Position, With<PlayerCursor>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let mut cursor_pos = cursor_query.single_mut();

    let current_cursor_pos = window_query
        .get_single()
        .expect("Didn't find a Primary Window when updating the cursor position!")
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor));

    if let Some(pos) = current_cursor_pos {
        cursor_pos.value = Vec3::new(pos.x, pos.y, 0.0) ;
    }
}
