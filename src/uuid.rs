// uuid.rs - contains functions for the creation and caching of UUIDs
use uuid::Uuid;

pub type UuidV4 = u128; // UUIDs are just u128s

// uuid::generate - generate a UUID and return it as a u128
pub fn generate() -> UuidV4
{
    return Uuid::new_v4().as_u128();
}
