use bevy::prelude::*;

mod components;
mod player;

pub struct Lithium;

impl Plugin for Lithium {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::LithiumPlayer)
        .add_systems(Startup, startup); 
    }
}

// TODO: Change camera to follow player - move to it's own module
fn startup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
