use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerAim;

#[derive(Component)]
pub struct PlayerDirection(pub Vec2);

#[derive(Component)]
pub struct PlayerCamera;
