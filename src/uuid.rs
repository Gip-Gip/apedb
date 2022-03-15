// uuid.rs - contains functions for the creation and caching of UUIDs
use uuid::Uuid; // Use the uuid library

pub type UuidV4 = u128; // UUIDs are just u128s
pub type UuidV4Cache = Vec<UuidV4>; // UUID Caches are just vectors

// uuid::generate - generate a UUID and return it as a u128
pub fn generate() -> UuidV4
{
    return Uuid::new_v4().as_u128(); // Generate UUID and return it as a UuidV4
}

// uuid::generate_cache - generate a vector of uuids, meant to be used as a uuid cache
//
// ARGUMENTS:
//  size: usize
pub fn generate_cache(size: usize) -> UuidV4Cache
{
    let mut cache = Vec::<UuidV4>::with_capacity(size);

    fill(&mut cache);

    return cache;
}

// uuid::get - get a uuid from a cache if there are any left, and if not generate a uuid
//
// ARGUMENTS:
//  cache: &mut UuidV4Cache - the cache to pull from
pub fn get(cache: &mut UuidV4Cache) -> UuidV4
{
    match cache.pop()
    {
        Some(uuid_v4) =>
        {
            return uuid_v4;
        }
        None =>
        {
            return generate();
        }
    }
}

// uuid::fill - fill a uuid cache with uuids
//
// ARGUMENTS:
//  cache: &mut UuidV4Cache - the cache to fill
pub fn fill(cache: &mut UuidV4Cache)
{
    for _i in cache.len()..cache.capacity()
    {
        cache.push(generate());
    }
}
