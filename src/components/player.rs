use bevy::{ecs::component::Component, input::keyboard::KeyCode, utils::HashMap};

#[derive(Component)]
pub struct Equipped;

/// Marking the Player
#[derive(Component)]
pub struct Player;

/// Marking the Player cursor (mouse)
#[derive(Component)]
pub struct PlayerCursor;

/// Marking the aiming entity of the Player
#[derive(Component)]
pub struct PlayerAim;

/// Marking the Camera following the Player
#[derive(Component)]
pub struct PlayerCamera;

#[derive(Clone)]
pub struct PlayerKey {
    pub is_pressed: bool,
    pub key_code: KeyCode,
}

impl PlayerKey {
    pub fn new(key_code: KeyCode) -> Self {
        Self {
            is_pressed: false,
            key_code,
        }
    }
}

#[derive(Hash, Clone)]
#[derive(Eq, PartialEq)]
pub enum Actions {
    GoUp,
    GoLeft,
    GoDown,
    GoRight,
}

#[derive(Component, Clone)]
pub struct PlayerKeyboardControls {
    pub dict: HashMap<Actions, PlayerKey>,
}

impl PlayerKeyboardControls {
    pub fn default() -> Self {
        let mut dict = HashMap::<Actions, PlayerKey>::new();
        dict.insert(Actions::GoUp, PlayerKey::new(KeyCode::KeyW));
        dict.insert(Actions::GoLeft, PlayerKey::new(KeyCode::KeyA));
        dict.insert(Actions::GoDown, PlayerKey::new(KeyCode::KeyS));
        dict.insert(Actions::GoRight, PlayerKey::new(KeyCode::KeyD));

        Self {
           dict,
        }
    }
}
