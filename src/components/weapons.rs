use bevy::{
    ecs::component::Component,
    math::{Vec2, Vec3},
};
use rand::prelude::*;

pub struct DamageMultiplier {
    chance: f32,
    multiplier: f32,
}

#[derive(Component)]
pub struct Weapon {
    name: String,
    /// The lower bound of the damage range
    damage_low_bound: f32,
    /// The upper bound of the damage range
    damage_up_bound: f32,
    /// Critical chance
    crit_chance: f32,
    /// Critical multiplier
    crit_multiplayer: f32,
    /// Length of the aiming triangle
    range: f32,
    /// half-width of the aiming triangle
    accuracy: f32,
    /// Rate at which the aiming triangle shrink when stationary
    shrink_rate: f32,
}

impl Weapon {
    pub fn new(
        damage_low_bound: f32,
        damage_up_bound: f32,
        crit_chance: f32,
        crit_multiplayer: f32,
        range: f32,
        accuracy: f32,
        shrink_rate: f32,
    ) -> Self {
        Self {
            damage_low_bound,
            damage_up_bound,
            crit_chance,
            crit_multiplayer,
            range,
            accuracy,
            shrink_rate,
        }
    }

    /// NOTE: Multipliers
    /// now we get just a list of multiplier's as param, because
    /// they can't be triggered internally (headshot, status effect, etc).
    /// Only one that is triggered internally is crit. It just happens randomly.
    pub fn get_damage(&self, multipliers: Vec<DamageMultiplier>) -> f32 {
        let mut rng = thread_rng();
        let mut mutliplier = 0.0;

        // Given multipliers are additively combined
        multipliers.iter().for_each(|m| {
            if rng.gen::<f32>() <= m.chance {
                mutliplier += m.multiplier;
            }
        });

        // The crit multiplier is multiplicatively combined
        if rng.gen::<f32>() <= self.crit_chance {
            mutliplier *= self.crit_multiplayer
        }
        let damage = rng.gen_range(self.damage_low_bound..self.damage_up_bound);

        damage * mutliplier
    }

    /// The first value is always the player position
    /// The rest are `range` away in the direction of the mouse (from player)
    /// with a left/right (perpendicular) offset of the weapon's `accuracy`
    pub fn get_aiming_points(&self, player_pos: Vec3, cursor_pos: Vec3) -> (Vec2, Vec2, Vec2) {
        // Currently makes sense only in 2d
        let player_pos_vec2 = Vec2::new(player_pos.x, player_pos.y);
        let cursor_pos_vec2 = Vec2::new(cursor_pos.x, cursor_pos.y);

        let dir: Vec2 = (cursor_pos_vec2 - player_pos_vec2).normalize();
        let lenght = dir * self.range;
        let cone_edge_1: Vec2 = cursor_pos_vec2 + self.accuracy * dir.perp() + lenght;
        let cone_edge_2: Vec2 = cursor_pos_vec2 + (-self.accuracy * dir.perp()) + lenght;

        (cursor_pos_vec2, cone_edge_1, cone_edge_2)
    }
}
