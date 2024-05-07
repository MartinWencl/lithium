use bevy::{
    asset::Assets,
    ecs::{
        event::EventReader,
        query::With,
        system::{Commands, Query, ResMut},
    },
    input::{
        keyboard::{KeyCode, KeyboardInput},
        mouse::MouseButtonInput,
    },
    math::{primitives::Triangle2d, Vec2},
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    transform::components::Transform,
    window::{PrimaryWindow, Window},
};

use crate::components::{MovementDirection, Position, Velocity};

use super::player_components::{self, Player, PlayerDirection};

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

pub fn mouse_events(
    mut player_query: Query<(&mut Velocity, &Position), With<Player>>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (mut velocity, position) = player_query.get_single_mut().unwrap();

    let cursor_position = window_query
        .get_single()
        .unwrap()
        .cursor_position()
        .unwrap();

    const CONE_WIDTH: f32 = 20.0;
    let player_aim = player_components::PlayerAim;
    let vertices = vec![
        Vec2::new(cursor_position.x, cursor_position.y + CONE_WIDTH),
        Vec2::new(cursor_position.x, cursor_position.y + CONE_WIDTH),
        Vec2::new(position.0.x, position.0.y),
    ];

    commands.spawn((
        // Player cube
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Triangle2d::new(vertices[0], vertices[1], vertices[2]))),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        player_aim,
    ));
}
