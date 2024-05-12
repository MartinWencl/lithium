use bevy::prelude::*;
use lithium::Lithium;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Lithium)
        .run();
}