use bevy::app::{App, Plugin};
use data::LithiumDataPlugin;
use systems::LithiumSystemsPlugin;

mod components;
mod data;
mod systems;
pub struct Lithium;

impl Plugin for Lithium {
    fn build(&self, app: &mut App) {
        app.add_plugins(LithiumSystemsPlugin)
            .add_plugins(LithiumDataPlugin);
    }
}
