use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerDirection {
    pub x: f32,
    pub y: f32,
}

impl PlayerDirection {
   pub fn new(x: f32, y: f32) -> Self {
       Self {x, y}
   } 
}

#[derive(Component)]
pub struct PlayerCamera;
