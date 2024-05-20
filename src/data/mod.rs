use bevy::utils::uuid;
use sqlite::ConnectionThreadSafe;

pub mod weapons;

pub trait LithiumData {
    const NAME: str;
    fn load(conn: ConnectionThreadSafe, id: uuid::Uuid) -> Self;
}
