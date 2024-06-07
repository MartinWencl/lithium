use std::error::Error;

use bevy::{
    ecs::{
        system::{CommandQueue, Resource},
        world::World,
    },
    tasks::AsyncComputeTaskPool,
};
use sqlx::SqliteConnection;

use crate::components::weapons::Weapon;

use super::LithiumData;

#[derive(Resource)]
pub struct Weapons {
    pub value: Vec<Weapon>,
}

impl LithiumData for Weapons {
    fn load(
        mut conn: sqlx::pool::PoolConnection<sqlx::Sqlite>,
        thread_pool: &AsyncComputeTaskPool,
    ) -> bevy::tasks::Task<()> {
        thread_pool.spawn(async move {
            let weapons = Weapons::load_all(&mut conn).await.unwrap();

            let mut command_queue = CommandQueue::default();

            command_queue.push(move |world: &mut World| {
                world.insert_resource(weapons);
            })
        })
    }
}

impl Weapons {
    pub async fn load_all(conn: &mut SqliteConnection) -> Result<Weapons, Box<dyn Error>> {
        let records = sqlx::query!("SELECT name, damage_low_bound, damage_up_bound, crit_chance, crit_multiplier, range, accuracy, shrink_rate  FROM Weapons").fetch_all(conn).await?;

        // Needed to convert the sqlite real (f64) to rust f32
        // bevy uses mostly f32's so it makes it more sane to convert here
        // the trim should not have any real impact
        let weapons = records
            .iter()
            .map(|r| Weapon {
                name: r.name.clone(),
                damage_low_bound: r.damage_low_bound as f32,
                damage_up_bound: r.damage_up_bound as f32,
                crit_chance: r.crit_chance as f32,
                crit_multiplier: r.crit_multiplier as f32,
                range: r.range as f32,
                accuracy: r.accuracy as f32,
                shrink_rate: r.shrink_rate as f32,
            })
            .collect();

        Ok(Self { value: weapons })
    }

    // TODO: Load from the resource, not from the db
    // Needs to know if the resource has been loaded into the world yet
    // pub async fn load_single(name: &str, conn: &mut SqliteConnection) -> Result<Weapon, Box<dyn Error>> {
    //     let weapon: Weapon = sqlx::query_as!(Weapon, "SELECT name, damage_low_bound, damage_up_bound, crit_chance, crit_multiplier, range, accuracy, shrink_rate  FROM Weapons WHERE name = ?", name).fetch_one(conn).await?;
    //     return Ok(weapon);
    // }
}
