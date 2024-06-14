use std::thread::sleep;

use bevy::{
    app::Plugin,
    log,
    tasks::{block_on, AsyncComputeTaskPool, Task},
};
use sqlx::{pool::PoolConnection, Sqlite, SqlitePool};

use self::weapons::Weapons;

pub mod weapons;

// TODO: Try to define a macro for loading resources
// with the `tasks.push(...)`
// macro_rules! register_task {
//     ($f:expr,$t:expr,$p:expr,$c: expr) => {
//             $t.push($p.spawn($f(
//                 connection_pool.acquire().await.unwrap(),
//                 thread_pool,
//             )));
//     };
// }

/// Trait that defines creating a Task<()> for loading data
pub trait LithiumData {
    fn load(conn: PoolConnection<Sqlite>, thread_pool: &AsyncComputeTaskPool) -> Task<()>;
}

pub struct LithiumDataPlugin;

impl Plugin for LithiumDataPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(bevy::app::Startup, load_data_startup);
        log::debug!("   - loaded the data plugin");
    }
}

fn load_data_startup() {
    let thread_pool = AsyncComputeTaskPool::get();
    let mut tasks: Vec<Task<()>> = vec![];

    // First we create the conn pool itself
    let connection_pool: SqlitePool = block_on(async {
        let connection_pool = SqlitePool::connect("sqlite://lithium.db");
        return connection_pool
            .await
            .expect("Failed connecting to database!");
    });

    // Needs to be done in async context
    // So we block on the whole future
    // Should still spawn the tasks to run async
    block_on(async {
        // Loading weapons
        tasks.push(Weapons::load(
            connection_pool.acquire().await.unwrap(),
            thread_pool,
        ));

        // Wait for all the spawned tasks to finish
        while tasks.iter().find(|t| !t.is_finished()).is_some() {
            sleep(std::time::Duration::from_millis(100));
        }
    });

    // NOTE: logic to hide future loading screen here!
    // TODO: Create global boolean resource `is_loaded` and set here

    log::debug!("Data Plugin - Finished loading resources at startup!");
}
