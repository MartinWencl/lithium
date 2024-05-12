use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Enemy;

pub enum EnemyAlertLevel {
    NotAlerted,
    Wary,
    Alerted,
}

#[derive(Component)]
pub struct AlertLevel {
    value: EnemyAlertLevel,
}