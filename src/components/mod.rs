use bevy::{
    ecs::component::Component,
    math::Vec3,
};

pub mod player;
pub mod enemies;
pub mod ui;
pub mod weapons;

#[derive(Component)]
pub struct Position {
    pub value: Vec3,
}

#[derive(Component)]
pub struct Health {
    value: u32,
    max_value: u32,
}

impl Health {
    pub fn take_damage<V>(&mut self, value: V)
    where
        u32: std::ops::SubAssign<V>,
        u32: std::ops::Sub<V>,
        <u32 as std::ops::Sub<V>>::Output: PartialOrd<i32>,
        V: Clone,
    {
        // Checking for negative health
        if (self.value - value.clone()) < 0 {
            self.value = 0;
            return;
        }
        self.value -= value;
    }

    pub fn heal<V>(&mut self, value: V)
    where
        u32: std::ops::AddAssign<V>,
    {
        self.value += value;

        // Capping to max health
        if self.value > self.max_value {
            self.value = self.max_value;
        }
    }

    pub fn heal_to_full(&mut self) {
        self.value = self.max_value;
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0
    }

    pub fn new(value: u32, max_value: u32) -> Self {
        Self { value, max_value }
    }

    pub fn get_val(&self) -> u32 {
        self.value
    }
}
