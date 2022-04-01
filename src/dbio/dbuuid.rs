// dbuuid.rs - contains functions for the creation and caching of UUIDs


use uuid::Uuid; // Use the uuid library



// Structs!
//



// dbuuid::UuidV4 - apedb uuid v4
//
pub struct UuidV4
{
    uuid: Uuid,
}

impl UuidV4
{
    // dbuuid::UuidV4::new - create a new UUID
    //
    pub fn new() -> UuidV4
    {
        return UuidV4
        {
            uuid: Uuid::new_v4()
        };
    }

    // dbuuid::UuidV4::to_bytes - convert a UUID to bytes
    //
    pub fn to_bytes(&self) -> Vec<u8>
    {
        return self.uuid.as_bytes().to_vec();
    }
}

// dbuuid::UuidV4Cache - cache of UUIDs
//
pub struct UuidV4Cache
{
    pub cache: Vec<UuidV4>,
}

impl UuidV4Cache
{
    // dbuuid::UuidV4Cache::new - create a new cache
    //
    // ARGUMENTS:
    //  size: usize - The number of uuids the cache can hold
    pub fn new(size: usize) -> UuidV4Cache
    {
        let mut cache = Vec::<UuidV4>::with_capacity(size);

        // Fil the cache with uuids
        for _ in 0 .. cache.capacity()
        {
            cache.push(UuidV4::new());
        }

        return UuidV4Cache
        {
            cache: cache,
        };
    }

    // dbuuid::UuidV4Cache::get - get a UUID from the cache
    //
    pub fn get(&mut self) -> UuidV4
    {
        match self.cache.pop()
        {
            Some(uuid) =>
            {
                return uuid;
            }
            None =>
            {
                return UuidV4::new();
            }
        }
    }

    // dbuuid::UuidV4Cache::is_empty - check if the cache is empty
    //
    pub fn is_empty(&self) -> bool
    {
        return self.cache.is_empty();
    }

    // dbuuid::UuidV4Cache::refill - refill the cache
    //
    pub fn refill(&mut self)
    {
        for _ in self.cache.len() .. self.cache.capacity()
        {
            self.cache.push(UuidV4::new());
        }
    }
}

// Tests!
//
#[cfg(test)]
mod tests
{
    use super::*;

    // dbio::dbuuid::test_uuid_v4 - test the uuid v4 generation
    //
    #[test]
    fn test_uuid_v4()
    {
        let uuid = UuidV4::new();

        assert_eq!(uuid.to_bytes().len(), 16); // Check the length of the uuid
        assert_ne!(uuid.to_bytes(), UuidV4::new().to_bytes()); // Check that the uuid is unique
    }

    // dbio::dbuuid::test_uuid_v4_cache_new - test the uuid v4 cache creation
    //
    #[test]
    fn test_uuid_v4_cache_new()
    {
        let cache = UuidV4Cache::new(10);

        assert_eq!(cache.cache.len(), 10); // Check the length of the cache
        assert_eq!(cache.cache.capacity(), 10); // Check the capacity of the cache
    }

    // dbio::dbuuid::test_uuid_v4_cache_get - test the uuid v4 cache get
    //
    #[test]
    fn test_uuid_v4_cache_get()
    {
        let mut cache = UuidV4Cache::new(10);

        let uuid = cache.get();

        assert_eq!(cache.cache.len(), 9); // Check the length of the cache

        assert_eq!(uuid.to_bytes().len(), 16); // Check the length of the uuid
        assert_ne!(uuid.to_bytes(), UuidV4::new().to_bytes()); // Check that the uuid is unique
    }

    // dbio::dbuuid::test_uuid_v4_cache_drain - make sure the cache functions propertly when empty
    //
    #[test]
    fn test_uuid_v4_cache_drain()
    {
        let mut cache = UuidV4Cache::new(10);

        cache.cache.drain(..);

        assert_eq!(cache.cache.len(), 0); // Check the length of the cache

        assert_ne!(cache.get().to_bytes(), cache.get().to_bytes()); // Check that the cache doesn't spit out the same uuid if drained
    }

    // dbio::dbuuid::test_uuid_v4_cache_refill - test the uuid v4 cache refill
    //
    #[test]
    fn test_uuid_v4_cache_refill()
    {
        let mut cache = UuidV4Cache::new(10);

        cache.cache.drain(..);

        cache.refill();

        assert_eq!(cache.cache.len(), 10); // Check the length of the cache
    }
}