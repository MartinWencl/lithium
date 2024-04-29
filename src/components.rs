use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub f32);

#[derive(Component)]
pub struct MovementDirection(pub Vec2);
