use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;

use crate::components::MovementDirection;
use crate::components::Position;
use crate::components::Velocity;

use self::player_components::Player;

mod player_components;
mod player_input;

const DRAG: f32 = 5.0;

pub struct LithiumPlayer;

impl Plugin for LithiumPlayer {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initiliaze_player)
            .add_systems(Update, player_input::keyboard_events)
            .add_systems(Update, player_input::mouse_events)
            .add_systems(Update, update_player);
    }
}

fn initiliaze_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player = player_components::Player;

    commands.spawn((
        // Player cube
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Cuboid {
                half_size: Vec3::new(10.0, 10.0, 0.0),
            })),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        player,
        Position(Vec2::splat(0.0)),
        MovementDirection(Vec2::splat(0.0)),
        Velocity(0.0),
    ));

}

fn update_player(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Position,
            &mut MovementDirection,
            &mut Velocity,
            &mut Transform,
        ),
        With<Player>,
    >,
) {
    let (mut position,mut direction, mut velocity, mut transform) = query.get_single_mut().unwrap();

    // calculate new position
    position.0 = position.0 + (direction.0 * velocity.0 * time.delta_seconds());

    // Velocity cannot be negative - full stop if negative
    if velocity.0 - DRAG >= 0.0 {
        velocity.0 -= DRAG;
    } else {
        velocity.0 = 0.0;
        direction.0 = Vec2::splat(0.0);
    }

    println!("v: {:?}, d: {:?}", velocity.0, direction.0);

    *transform = Transform::from_xyz(position.0.x, position.0.y, 0.0)
}
