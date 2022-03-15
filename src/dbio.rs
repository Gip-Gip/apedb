// dbio.rs - contains functions and structs neccessery for database file io

use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::path::Path;
use std::io::SeekFrom;

use crate::crc24;

// Macros!
//

// chunk_number2offset - convert a chunk number to a file offset...
//
// ARGUMENTS:
//  $number - the chunk number to be converted
macro_rules! chunk_number2offset
{
    ($number:expr)=>
    {
        ($number * CHUNK_SIZE as u64)
    }
}

// chunk_offset2number - convert a file offset to a chunk number...
//
// ARGUMENTS:
//  $number - the chunk number to be converted
macro_rules! chunk_offset2number
{
    ($offset:expr)=>
    {
        ($offset / CHUNK_SIZE as u64)
    }
}

// Constants!
//

pub const CHUNK_SIZE: usize = 256; // Size of one chunk in bytes
pub const CHUNK_HEADSZ: usize = 1; // Size of the header in bytes
pub const CHUNK_CRCSZ: usize = 3; // Size of the ApeDB CRC24 in bytes
pub const CHUNK_DATASZ: usize = CHUNK_SIZE - (CHUNK_HEADSZ + CHUNK_CRCSZ); // Room left for data!

pub mod chunk_flags
{
    pub const UNDER_CONSTRUCTION:u8 = 0b10000000; // If under construction, bit 7 is set
    pub const CONTINUED:u8 = 0b01000000; // If the chunk's data is continued elsewhere
}

pub mod chunk_variants
{
    pub const FREE: u8 = 0x0; // Free chunk open to be used for anything
    pub const DBHEAD: u8 = 0x1; // Database header chunk
}


// Structs!
//

// Database struct, used for working with databases
//
pub struct Database
{
    file: File, // The database file
    file_name: String, // The full file path as a string
    chunk_count: u64,
}

impl<'a> Database
{
    // dbio::Database::create - create and return a database with an empty file
    //
    // ARGUMENTS:
    //  name: &'a str - name of the database file to create
    //
    // RESULTS:
    //  Ok - return Database
    //  Err - return std::io::Error
    pub fn create(name: &'a str) -> Result<Database, Error>
    {
        let path = Path::new(name.clone()); // Convert the name to a path we can work with

        if path.exists() // If the file exists...
        {
            // Tell the user we refuse to overwrite the database and return!
            return Err(Error::new(ErrorKind::Other, "File Already Exists!"));
        }

        // Otherwise, create the file and make it read-write enabled!
        // Also abort the function and return the error if anything goes wrong...
        let file = File::options().read(true).write(true).create(true).open(&path)?;


        return Ok // If everything is ok...
        (
            // Return a database struct with...
            //
            // - self.file set to the previously specified file
            // - self.file_name set to the full path to the file
            // - self.chunk_count set to 0 since there are no chunks in a new file
            Database
            {
                file: file,
                file_name: path.display().to_string(),
                chunk_count: 0,
            }
        );
    }

    // dbio::Database::open - open, initialize and return a database from an existing file
    //
    // ARGUMENTS:
    //  name: &'a str - name of the database file to open
    //
    // RESULTS:
    //  Ok - return Database
    //  Err - return std::io::Error
    pub fn open(name: &'a str) -> Result<Database, Error>
    {
        let path = Path::new(name.clone()); // Convert the name to a path we can work with...

        // Open the file and make it read-write enabled!
        // Also abort the function if the file doesn't exist or if any other error occurs...
        let mut file = File::options().read(true).write(true).create(false).open(&path)?;

        // Next get the amount of chunks in a database file...
        //
        // This is fairly simple, since the smallest unit a database should be operated on, at
        // least when it comes to file io, is a chunk. Meaning, that if the file size of the
        // database is not equal to CHUNK_SIZE * chunk_count, there is without a doubt a case of
        // corruption...
        //
        // First, get the length of the file in bytes. For now we'll have to do it the hacky way of
        // getting the return value of a seek to the end of the file...
        // Currently the function that does this a not hacky way is only availible in the nightly
        // build -- when it is added to the official lanuage please replace this hack.
        // Also if either of these fail abort the function
        let file_length = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start(0))?; // Seek back to the start

        // If the file size of the database is not perfectly divisible by CHUNK_SIZE...
        if file_length % CHUNK_SIZE as u64 != 0 // Btw we need to cast the CHUNK_SIZE to a u64
        {
            // There is most certainly a sign of corruption! Abort the function and notify the user
            return Err
            (
                Error::new(ErrorKind::Other, "Database an unusual size, corruption detected!")
            )
        }

        let chunk_count = chunk_offset2number!(file_length); // Convert the file size to a count

        return Ok // If everything is ok...
        (
            // Return a database struct with...
            //
            // - self.file set to the previously specified file
            // - self.file_name set to the full path to the file
            // - self.chunk_count set to the previously specified chunk count...
            Database
            {
                file: file,
                file_name: path.display().to_string(),
                chunk_count: chunk_count,
            }
        );
    }

    // dbio::Database::add_chunk - add raw chunk to database file, should probably be made private
    //
    // ARGUMENTS:
    //  &mut self - The database we are appending to
    //  chunk: &Chunk - The chunk to write
    //
    // RESULTS:
    //  Ok - return () aka nothing
    //  Err - return std::io::Error
    pub fn add_chunk(&mut self, chunk: &Chunk) -> Result<(), Error>
    {
        let mut data = Vec::<u8>::with_capacity(CHUNK_SIZE); // Create chunk-sized data buffer

        // If some erronous code made a data entry greater than the maximum size for a data entry..
        if chunk.data.len() > CHUNK_DATASZ
        {
            return Err // Hopefully the end user never sees this error...
            (
                Error::new(ErrorKind::Other, "Chunks size over max. You shouldn't be seeing this!")
            );
        }

        // Pad the rest of the unused data space with zeros, to keep the chunk size == CHUNK_SIZE
        let padding = CHUNK_DATASZ - chunk.data.len();

        // Flags and types are laid out to where if the two numbers are added together the
        // flags are the upper 4 bits and the type is the lower 4 bits of the header.
        // If I could I'd set chunk.flags and chunk.type to a hypothetical u4...
        let header: u8 = chunk.flags | chunk.variant;
        data.push(header);

        // Write all of the data and it's padding to the buffer aswell...
        data.extend_from_slice(&chunk.data);
        data.extend_from_slice(&vec![0;padding]);

        // Compute the CRC from the buffer so far
        // (keep in mind the buffer is only CHUNK_HEADSZ + CHUNK_DATASZ long...)
        let crc = crc24::compute(&data);

        // Write the CRC to the file(making sure to convert the CRC to big endian)
        data.extend_from_slice(&crc24::to_be_bytes(crc));

        // Seek to the end of the file so we can append the data
        // Also if either of these two function calls fail abort the function and return the error
        self.file.seek(SeekFrom::End(0))?;

        // Write/append the data!
        self.file.write_all(&data)?;

        // Since nothing bad return Ok!
        return Ok(());
    }

    // dbio::Database::verify_chunk - verify a chunk given the chunk number
    //
    // ARGUMENTS:
    //  &mut self - the database to retrieve the chunk from
    //  number: u64 - the chunk number
    //
    // RESULTS:
    //  Ok - return bool
    //  Err - return std::io::Error
    pub fn verify_chunk(&mut self, number: u64) -> Result<bool, Error>
    {
        // Create data buffer(will always be CHUNK_SIZE)
        let mut data:[u8;CHUNK_SIZE] = [0; CHUNK_SIZE];

        // Seek to the chunk with the chunk number...
        self.file.seek(SeekFrom::Start(chunk_number2offset!(number)))?;

        // Read data of size CHUNK_SIZE to the data buffer
        // If for some reason there's not enough data to fill the buffer, that means the chunk must
        // be corrupt!
        // Also in general file errors are handled aswell...
        self.file.read_exact(&mut data)?;

        // An interesting function of the CRC is that if you calculate the crc of any amount of
        // data and then append the crc to the end off said data, a CRC calculation of this
        // CRC-appended data will always be zero, so long as there is no corruption
        //
        // Return true if the crc is equal to zero, false if not
        return Ok(crc24::compute(&data) == 0);
    }
}



// Chunk struct, used for working with primitive chunks. Should probably be private...
//
pub struct Chunk<'a>
{
    pub flags: u8, // Chunk flags
    pub variant: u8, // Chunk variant
    pub data: &'a [u8], // Data for the rest of the chunk
    pub number: u64, // Chunk number
}
