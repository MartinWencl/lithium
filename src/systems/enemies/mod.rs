use bevy::{app::{Plugin, Update}, ecs::{entity::Entity, query::{With, Without}, system::{Commands, Query}}};

use crate::components::{enemies::Enemy, player::Player, Health};

pub struct LithiumEnemySystem;

impl Plugin for LithiumEnemySystem {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, remove_when_no_health);
    }
}

/// Removes all non-player enemies that have reached 0 health
pub fn remove_when_no_health(
    query: Query<(Entity, &mut Health), (With<Enemy>, Without<Player>)>,
    mut commands: Commands,
) {
    for (entity, health) in &query {
        // TODO: Add a call for death animation here or smt
        if health.is_zero() {
            commands.entity(entity).despawn();
        }
    }
}