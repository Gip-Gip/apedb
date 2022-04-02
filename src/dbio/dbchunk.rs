// dbchunk.rs - Code for working with database files and thier chunks



use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::io::SeekFrom;
use std::mem::*;
use simple_error::*;

use crate::dbio::dbfield::*;
use crate::dbio::dbcrc24::*;
use crate::dbio::dbstruct::*;
use crate::apetypes::*;



// Constants!
//



const CHUNKSZ: usize = 256; // Total size of a chunk
const CHUNKHEADSZ: usize = 1; // Size of the chunk header
const CHUNKCRCSZ: usize = 3; // Size of the chunk CRC
const CHUNKDATASZ: usize = 256 - CHUNKHEADSZ - CHUNKCRCSZ; // Size of the chunk data

const DB_DEFAULT_UUID_CACHE_SIZE: i64 = 1024; // Default size of the UUID cache, 1024*16 bytes
const DB_DEFAULT_UNIX_PERMISSIONS: i64 = 0o777; // Default unix octal permissions

// dbchunks::chunk_type - Chunk type constants
pub mod chunk_type
{
    pub const DBHEAD: u8 = 0x01; // DB header
}



// Enums!
//


// 
pub enum ChunkTypes
{
    DbHead(DbHeadChunk)
}

// dbchunk::ChunkyFile - Struct for interfacing with chunky files
//
pub struct ChunkyFile
{
    file: File, // The file
    size: usize, // The size of the file
}

impl ChunkyFile
{
    // dbchunk::ChunkyFile::create() - Create a new chunky file, throw an error if it already exists
    //
    // ARGUMENTS:
    //  file_name: &str - The name of the file to create
    pub fn create(file_name: &str) -> Result<ChunkyFile, Box<dyn Error>>
    {
        let path = Path::new(file_name);

        if path.exists() // If the file exists...
        {
            // Tell the user we refuse to overwrite the database and return!
            bail!("File Already Exists!");
        }

        // Open a file with reading and writing enabled, also create it since it shouldn't exist
        let file = File::options().read(true).write(true).create(true).open(&path)?;

        return Ok
        (
            ChunkyFile
            {
                file: file,
                size: 0, // Set the size to zero since we haven't written anything yet
            }
        );
    }

    // dbchunk::ChunkyFile::add_chunk() - Add a chunk to the file
    //
    // ARGUMENTS:
    //  chunk: ChunkTypes - The chunk to add wrapped in a ChunkTypes enum
    pub fn add_chunk(&mut self, chunk: ChunkTypes) -> Result<(), Box<dyn Error>>
    {
        let mut data = Vec::<u8>::with_capacity(CHUNKDATASZ); // Initialize the data buffer vector to chunksize, this size should not be exceeded...

        // Get the chunk type from the enum and fill the data buffer with the chunk data
        let chunk_type: u8 = match &chunk
        {
            ChunkTypes::DbHead(dbchunk) =>
            {
                for field in &dbchunk.fields
                {
                    data.extend_from_slice(&field.to_bytes()?);
                }

                chunk_type::DBHEAD
            }
        };

        // Seek to the end of the file so we can append the data
        // Also if either of these two function calls fail abort the function and return the error
        self.file.seek(SeekFrom::End(0))?;

        // Write all the data to the file, to multiple chunks if necessary
        while !data.is_empty()
        {
            // If the (remaining) data can fit into one chunk...
            if data.len() <= CHUNKDATASZ
            {
                let head = chunk_type;
                let padding = CHUNKDATASZ - data.len(); // Pad the unused space with zeros
                let mut chunk_data = Vec::<u8>::with_capacity(CHUNKSZ); // Initialize the chunk buffer

                // Add all the data to the chunk buffer
                chunk_data.push(head);
                chunk_data.extend_from_slice(&data);
                chunk_data.extend_from_slice(&vec![0;padding]);

                // Calculate the CRC and add it to the chunk buffer
                let crc24 = ApeCrc24::new(&chunk_data);
                chunk_data.extend_from_slice(&crc24.to_be_bytes());

                // Write/append the data!
                self.file.write_all(&chunk_data)?;
                self.size += CHUNKSZ;

                // Set the data size to zero to end the loop
                data.truncate(0);
            }
            else
            {
                // To implement later...
                panic!("Chunk too big!");
            }
        }

        return Ok(());
    }
}

// dbchunk::DbHeadChunk - Struct for creating and modifying the DB header chunk
pub struct DbHeadChunk
{
    //pub chunk_numbers: Vec<u64>,
    pub fields: Vec<Field>,
}

impl DbHeadChunk
{
    // dbchunk::DbHeadChunk::new() - Create a new DB header chunk
    //
    // ARGUMENTS:
    //  name: &str - The name of the database
    //  owner: &str - The owner of the database
    pub fn new(name: &str, owner: &str) -> DbHeadChunk
    {
        let mut dbfields = Vec::<Field>::new();

        let mut requirements = Vec::<Requirement>::new();

        //requirements.push(Requirement::new("name", std::mem::discriminant(&Type::S(S::new("")))));

        let dbstructure = Structure::new("db", requirements);

        dbfields.push(Field::new("name", Type::S(Some(S::new(name))))); // Name field, database name
        dbfields.push(Field::new("ver", Type::I(Some(I::new(0))))); // Version field, database file version
        dbfields.push(Field::new("uuid_cache_size", Type::I(Some(I::new(DB_DEFAULT_UUID_CACHE_SIZE))))); // Uuid cache size field
        dbfields.push(Field::new("perm", Type::I(Some(I::new(DB_DEFAULT_UNIX_PERMISSIONS))))); // Unix permissions field
        dbfields.push(Field::new("owner", Type::S(Some(S::new(owner))))); // Owner field
        dbfields.push(Field::new("sane", Type::B(Some(B::new(true))))); // Sane field
        dbfields.push(Field::new("insane", Type::B(Some(B::new(false))))); // Insane field

        println!("{}", dbstructure.meets(&dbfields));

        return DbHeadChunk
        {
            //chunk_numbers: Vec::<u64>::new(),
            fields: dbfields,
        };
    }
}
