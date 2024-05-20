use bevy::app::{App, Plugin};
use systems::LithiumSystemsPlugin;

mod systems;
mod components;
mod data;
pub struct Lithium;

impl Plugin for Lithium {
    fn build(&self, app: &mut App) {
        app.add_plugins(LithiumSystemsPlugin);
    }
}
