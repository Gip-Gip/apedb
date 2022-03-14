mod crc24;
mod uuid;

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

// Quick UUID benchmark, not ment to be permanant so it's ugly
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
}
