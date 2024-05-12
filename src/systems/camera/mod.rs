use bevy::{app::{Plugin, Startup, Update}, core_pipeline::core_2d::Camera2dBundle, ecs::{query::With, system::{Commands, Query}}, math::Vec3, transform::components::Transform};

use crate::components::{player::{Player, PlayerCamera}, Position};

pub struct LithiumCameraSystem;

impl Plugin for LithiumCameraSystem {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, startup_camera)
            .add_systems(Update, update_camera);
    }
}

fn startup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), crate::components::player::PlayerCamera {}));
}

fn update_camera(
    player_query: Query<&Position, With<Player>>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
) {
    let player_position = player_query.get_single().unwrap();
    let mut camera_tranform = camera_query.get_single_mut().unwrap();

    camera_tranform.translation = Vec3::new(player_position.value.x, player_position.value.y, 1.0)
}