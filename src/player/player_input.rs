use bevy::{
    ecs::{event::EventReader, query::With, system::Query},
    input::{
        keyboard::{KeyCode, KeyboardInput},
        ButtonState,
    },
};

use crate::components::{MovementDirection, Position, Velocity};

use super::player_components::Player;

const VELOCITY: f32 = 20.0;
const MAX_VELOCITY: f32 = 300.0;

pub fn keyboard_events(
    mut key_event_reader: EventReader<KeyboardInput>,
    mut player_query: Query<(&mut MovementDirection, &mut Velocity), With<Player>>,
) {
    let (mut direction, mut velocity) = player_query.get_single_mut().unwrap();

    for ev in key_event_reader.read() {
        match ev.key_code {
            KeyCode::KeyW => direction.0.y = 1.0,
            KeyCode::KeyS => direction.0.y = -1.0,
            KeyCode::KeyA => direction.0.x = -1.0,
            KeyCode::KeyD => direction.0.x = 1.0,
            _ => (),
        };

        direction.0 = direction.0.normalize_or_zero();

        if velocity.0 + VELOCITY <= MAX_VELOCITY {
            velocity.0 += VELOCITY;
        }
    }
}
