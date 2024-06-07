use bevy::{app::{Plugin, Startup}, core_pipeline::core_2d::Camera2dBundle, ecs::system::Commands, math::Vec3, transform::components::Transform};

pub struct LithiumCameraSystem;

impl Plugin for LithiumCameraSystem {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, startup_camera);
    }
}

fn startup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), crate::components::player::PlayerCamera {}));
}

/// Updates the camera 
/// NOTE: Updating the camera must happen immediately when the player is updated.
/// So it's called in the player system. Kept in the camera system for consistency
pub fn update_camera(player_pos: Vec3, mut camera_transform: Transform)
{
    camera_transform.translation = Vec3::new(player_pos.x, player_pos.y, 1.0)
}
