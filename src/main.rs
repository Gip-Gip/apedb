mod crc24;
mod uuid;
mod dbio;

use std::time::SystemTime;

fn main()
{
    let hello = "Hello, World!";
    println!("{}", hello);

    let mut crc = crc24::compute(&hello.as_bytes());
    println!("CRC: {:x}", crc);

    let hello = &hello.to_uppercase();

    println!("{}", hello);

    crc = crc24::compute(&hello.as_bytes());
    println!("CRC: {:x}", crc);

// Quick UUID benchmark, not meant to be permanant so it's ugly
    let its = 1000;

// Iterate just randomly generating uuids
    let mut cnt = 0;
    let timer = SystemTime::now();

    while cnt < its
    {
        uuid::generate();
        cnt += 1;
    }

    let time = timer.elapsed();

    match time
    {
        Ok(elapsed) =>
        {
            println!("UUID Iterations: {}\nUUID Time: {}", cnt, elapsed.as_nanos());
        }
        Err(e) =>
        {
            println!("Error: {:?}", e);
        }
    }

// Generate a UUID cache...
    let timer = SystemTime::now();

    let mut cache = uuid::generate_cache(its);

    let time = timer.elapsed();

    match time
    {
        Ok(elapsed) =>
        {
            println!("UUID Init Time: {}", elapsed.as_nanos());
        }
        Err(e) =>
        {
            println!("Error: {:?}", e);
        }
    }

    let mut cnt = 0;
    let timer = SystemTime::now();

    while cnt < its / 2
    {
        uuid::get(&mut cache);
        cnt += 1;
    }

    let time = timer.elapsed();

    uuid::fill(&mut cache);

    match time
    {
        Ok(elapsed) =>
        {
            println!("UUID Iterations: {}\nUUID Time: {}\nUUID: {:x}", cnt, elapsed.as_nanos(), uuid::get(&mut cache));
        }
        Err(e) =>
        {
            println!("Error: {:?}", e);
        }
    }

// Create database file

    match dbio::Database::open(&"test.apedb")
    {
        Ok(mut db) =>
        {
            let chunk = dbio::Chunk
            {
                flags: dbio::chunk_flags::UNDER_CONSTRUCTION,
                variant: dbio::chunk_variants::DBHEAD,
                data: &[1, 2, 3, 4, 5],
                number: 0,
            };

            db.add_chunk(&chunk).unwrap();

            match db.verify_chunk(0)
            {
                Ok(good) =>
                {
                    println!("{}", good);
                }

                Err(e) =>
                {
                    println!("Error: {:?}", e)
                }
            };
        }
        Err(e) =>
        {
            println!("Error: {:?}", e)
        }
    };
}
