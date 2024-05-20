use bevy::{
    app::{App, Plugin, Startup, Update}, asset::Assets, ecs::{
        entity::Entity, query::With, system::{Commands, Query, Res, ResMut}
    }, math::{primitives::{Cuboid, Triangle2d}, Vec2, Vec3}, render::{color::Color, mesh::Mesh}, sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle}, time::Time, transform::components::Transform
};
use bevy::utils::default;

use crate::{components::{player::{Actions, Player, PlayerAim, PlayerCamera, PlayerCursor, PlayerKeyboardControls}, weapons::Weapon, Health, Position}, systems::camera::update_camera};

pub struct LithiumPlayerSystem;

impl Plugin for LithiumPlayerSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initiliaze_player)
            .add_systems(Update, update_player)
            .add_systems(Update, update_player_aim);
    }
}

/// Initializes the Player Entity
/// Together with:
///      PlayerAim - aiming triangle
///      PlayerCursor - cursor
fn initiliaze_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    dbg!("Initiliazing player");
    
    // Initialize empty PlayerAim entity, so we can query for them in `update_player` consistently
    commands.spawn((crate::components::player::PlayerAim {},));

    // Initialize the PlayerCursor 
    commands.spawn((crate::components::player::PlayerCursor {}, Position { value: Vec3::splat(0.0) },));

    // Inialize the Player
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
        Player,
        Position { value: Vec3::splat(0.0) },
        Health::new(100, 1000),
        PlayerKeyboardControls::default(),
    ));

    // TODO: REMOVE - JUST TO SEE MOVEMENT
    // The test cube
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Cuboid {
                half_size: Vec3::new(10.0, 10.0, 0.0),
            })),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}


/// Updated the player (Cube)
/// Calculates the new position based on what key's are pressed and applies the transform
/// Works of the associated component's (i.e. `Position`, `PlayerKeyboardControls`) insted of getting the data direcly
fn update_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &PlayerKeyboardControls, &mut Position), With<Player>>,
    camera_query: Query<&mut Transform, With<PlayerCamera>>,
) {
    let delta_time = time.delta_seconds();
    let camera_transform = camera_query.single();

    // TODO: Change speed and add some form of drag
    let speed = 1000.0;

    const ERROR_MSG: &str = "At least one of the four movement direction's keyboard keys was not set by the time the player get's updated!";

    for (mut transform, controls, mut position) in &mut query {
        // TODO: CHANGE - just adapted from the first iteration.
        let direction = Vec2::new(
            if controls.dict.get(&Actions::GoRight).expect(ERROR_MSG).is_pressed { 1.0 } else { 0.0 }
                + if controls.dict.get(&Actions::GoLeft).expect(ERROR_MSG).is_pressed { -1.0 } else { 0.0 },
            if controls.dict.get(&Actions::GoUp).expect(ERROR_MSG).is_pressed { 1.0 } else { 0.0 }
                + if controls.dict.get(&Actions::GoDown).expect(ERROR_MSG).is_pressed { -1.0 } else { 0.0 }
        );

        let delta_position = direction * speed * delta_time;

        // Update the player position component
        position.value += Vec3::new(delta_position.x, delta_position.y, 0.0);

        // Apply the transfrom for the new position
        // TODO: Think about implementing z-ordering
        transform.translation += Vec3::new(delta_position.x, delta_position.y, 0.0);

        // Update camera to the player's new position
        update_camera(position.value, *camera_transform);
    }
}

/// Updates the player aiming entity (triangle)
/// Despawns the exising one and spawns a new one
/// Works of the associated component's (i.e. Position on PlayerCursor) insted of getting the data direcly
fn update_player_aim(
    player_query: Query<&Position, With<Player>>,
    cursor_query: Query<&Position, With<PlayerCursor>>,
    entity_query: Query<Entity, With<PlayerAim>>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // HACK: This is a placeholder so that we can use the WeaponStats struct
    // without any weapon systems
    let weapon = Weapon::new(15.0, 20.0, 0.33, 2.0, 500.0, 200.0, 0.0);

    // There should only be one entity, as we clear them before creating new one 
    // TODO: This could be an assert, but currently crashing on multiple is not great, all of them should be cleared if that happens
    let aim_entity = entity_query.single();
    commands.entity(aim_entity).despawn();

    let cursor_pos = cursor_query.single().value;
    let player_pos = player_query.single().value;

    let (p1, p2, p3) = weapon.get_aiming_points(player_pos, cursor_pos);

    // TODO: Don't delete and spawn new entity, just move and rotate the existing one
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Triangle2d::new(p1, p2, p3))),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..Default::default()
        },
        PlayerAim {},
    ));
}
