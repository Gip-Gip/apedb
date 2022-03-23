
use std::fs::File;
use std::error::Error;
use std::io::{Error as IoError, ErrorKind};
use std::io::prelude::*;
use std::path::Path;
use std::io::SeekFrom;
use simple_error::*;

use crate::dbio::dbfield::*;
use crate::dbio::dbcrc24::*;

const CHUNKSZ: usize = 256;
const CHUNKHEADSZ: usize = 1;
const CHUNKCRCSZ: usize = 3;
const CHUNKDATASZ: usize = 256 - CHUNKHEADSZ - CHUNKCRCSZ;

pub mod ChunkFlags
{
    pub const CONTINUED: u8 = 0b10000000;
}

pub mod ChunkType
{
    pub const DBHEAD: u8 = 0x01;
}

pub enum ChunkTypes
{
    DbHead(DbHeadChunk)
}

pub struct ChunkyFile
{
    file: File,
    size: usize,
}

impl ChunkyFile
{
    pub fn create(file_name: &str) -> Result<ChunkyFile, Box<dyn Error>>
    {
        let path = Path::new(file_name);

        if path.exists() // If the file exists...
        {
            // Tell the user we refuse to overwrite the database and return!
            bail!("File Already Exists!");
        }

        let file = File::options().read(true).write(true).create(true).open(&path)?;

        return Ok
        (
            ChunkyFile
            {
                file: file,
                size: 0,
            }
        );
    }

    pub fn add_chunk(&mut self, chunk: ChunkTypes) -> Result<(), Box<dyn Error>>
    {
        let mut data = Vec::<u8>::with_capacity(CHUNKDATASZ);

        let chunk_type: u8 = match &chunk
        {
            ChunkTypes::DbHead(dbchunk) =>
            {
                for field in &dbchunk.fields
                {
                    data.extend_from_slice(&field.to_bytes()?);
                }

                ChunkType::DBHEAD
            }
        };

        // Seek to the end of the file so we can append the data
        // Also if either of these two function calls fail abort the function and return the error
        self.file.seek(SeekFrom::End(0))?;

        while !data.is_empty()
        {
            if data.len() <= CHUNKDATASZ
            {
                let head = chunk_type;
                let padding = CHUNKDATASZ - data.len();
                let mut chunk_data = Vec::<u8>::with_capacity(CHUNKSZ);

                chunk_data.push(head);
                chunk_data.extend_from_slice(&data);
                chunk_data.extend_from_slice(&vec![0;padding]);

                let crc24 = ApeCrc24::new(&chunk_data);

                chunk_data.extend_from_slice(&crc24.to_be_bytes());

                // Write/append the data!
                self.file.write_all(&chunk_data)?;
                self.size += CHUNKSZ;

                data.truncate(0);
            }
            else
            {
                panic!("Chunk too big!");
            }
        }

        return Ok(());
    }
}

pub struct DbHeadChunk
{
    pub chunkNumbers: Vec<u64>,
    pub fields: Vec<Field>,
}

impl DbHeadChunk
{
    pub fn new(name: String, owner: String) -> DbHeadChunk
    {
        let mut dbfields = Vec::<Field>::new();

        dbfields.push(Field::new("name".to_string(), Type::S(S::new(name))); // Name field, database name
        dbfields.push(Field::new("ver".to_string(), Type::I(I::new(0)))) // Version field, database file version
        dbfields.push(Field::new("uuid_cache_size".to_string(), Type::I(I::new(DB_DEFAULT_UUID_CACHE_SIZE)))); // Uuid cache size field, start out with the default size for a uuid cache
        dbfields.push(Field::new("perm".to_string(), Type::I(I::new(DB_DEFAULT_UNIX_PERMISSIONS))));
        dbfields.push(Field::new("owner".to_string(), Type::S(S::new(owner))));

        return DbHeadChunk
        {
            chunkNumbers: Vec::<u64>::new(),
            fields: dbfields,
        };
    }
}
