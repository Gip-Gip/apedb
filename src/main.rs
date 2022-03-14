mod crc24;

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
}
