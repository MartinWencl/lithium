use bevy::prelude::*;

mod components;
mod player;

const X_EXTENT: f32 = 600.;

pub struct Lithium;

impl Plugin for Lithium {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::LithiumPlayer)
        .add_systems(Startup, startup); 
    }
}

fn startup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
