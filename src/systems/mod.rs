use bevy::app::Plugin;

mod enemies;
mod input;
mod player;
mod camera;
mod ui;

pub struct LithiumSystemsPlugin;

impl Plugin for LithiumSystemsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(input::LithiumInputSystem)
            .add_plugins(player::LithiumPlayerSystem)
            .add_plugins(enemies::LithiumEnemySystem)
            .add_plugins(camera::LithiumCameraSystem)
            .add_plugins(ui::LithiumUISystem);
    }
}