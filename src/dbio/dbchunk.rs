// dbchunk.rs - Code for working with database files and thier chunks



use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::io::SeekFrom;
use simple_error::*;
use crate::dbio::dbfield::*;
use crate::dbio::dbcrc24::*;
use crate::dbio::dbstruct::*;
use crate::dbio::dblist::*;
use crate::apetypes::*;
use apebdlm::*;


// Constants!
//



const CHUNKSZ: usize = 256; // Total size of a chunk
const CHUNKCRCSZ: usize = 3; // Size of the chunk CRC
const CHUNKFIELDALLOC: usize = 256; // The default amount of memory to allocate with working with individual fields

const CHUNK_ENTRY_CONT_HEADSZ: usize = 9; // 1 u8 + 1 u64 = 9 bytes
const CHUNK_ENTRY_CONT_DATASZ: usize = CHUNKSZ - (CHUNK_ENTRY_CONT_HEADSZ + CHUNKCRCSZ);

const CHUNK_ENTRY_STUB_HEADSZ: usize = 2; // 2 u8s = 2 bytes
const CHUNK_ENTRY_STUB_DATASZ: usize = CHUNKSZ - (CHUNK_ENTRY_STUB_HEADSZ + CHUNKCRCSZ);

const DB_DEFAULT_UUID_CACHE_SIZE: i64 = 1024; // Default size of the UUID cache, 1024*16 bytes
const DB_DEFAULT_UNIX_PERMISSIONS: i64 = 0o777; // Default unix octal permissions

// dbchunks::CHUNK_TYPE - Chunk type constants
pub mod CHUNK_TYPE
{
    pub const DBHEAD: u8 = 0x01; // DB header
    pub const ENTRY: u8 = 0x02; // Entry
}

pub mod CHUNK_FLAG
{
    pub const UNDER_CONSTRUCTION: u8 = 0b1000000;
    pub const CONTINUED: u8 = 0b01000000;
}

// Enums!
//


// 
pub enum ChunkTypes
{
    DbHead(DbHeadChunk),
    Entry(EntryChunk)
}

// dbchunk::ChunkyFile - Struct for interfacing with chunky files
//
#[derive(Debug)]
pub struct ChunkyFile
{
    pub file: File, // The file
    pub size: usize, // The size of the file
}

impl ChunkyFile
{
    // dbchunk::ChunkyFile::create() - Create a new chunky file, throw an error if it already exists
    //
    // ARGUMENTS:
    //  file_name: other: &[T]
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

    // dbchunk::ChunkyFile::add_chunk() - Add a chunk to the file, and depending on the type of chunk return the insertion points of the chunk fields
    //
    // ARGUMENTS:
    //  chunk: ChunkTypes - The chunk to add wrapped in a ChunkTypes enum
    pub fn add_chunk(&mut self, chunk: &ChunkTypes) -> Result<Option<Vec<u64>>, Box<dyn Error>>
    {
        return Ok(None); // To be removed...   
    }

    pub fn add_entry_chunk(&mut self, chunk: EntryChunk) -> Result<Vec<u64>, Box<dyn Error>>
    {
        let mut i = 0;
        let g = chunk.fields.len(); // Allocate enough memory for the insertion points equal to the number of fields +1 for the insertion point of the chunk itself
        let mut insertion_points = Vec::<u64>::with_capacity(g);

        // Get the total data of all of the fields + the total length
        let mut fields_total_data = Vec::<Vec<u8>>::with_capacity(g);
        let mut fields_total_length: usize = 0;
        for field in chunk.fields
        {
            let field_data = field.to_bytes()?;
            fields_total_length += field_data.len();
            fields_total_data.push(field_data);
        }

        let mut carry_data = Vec::<u8>::new();
        let mut written_len: usize = 0;

        while (i < g) || (carry_data.len() > 0)
        {
            // There are two types of entry chunks...
            if (fields_total_length - written_len) > CHUNK_ENTRY_STUB_DATASZ // continued entry chunks...
            {
                let mut file_position = self.file.seek(SeekFrom::End(0))?;

                let header = CHUNK_FLAG::CONTINUED | CHUNK_TYPE::ENTRY;
                let next_chunk: u64 = file_position + (CHUNKSZ as u64);
                let mut data = Vec::<u8>::with_capacity(CHUNK_ENTRY_CONT_DATASZ);

                if carry_data.len() > 0
                {
                    data.extend_from_slice(&carry_data);
                    carry_data.clear();
                }
                
                file_position += (CHUNK_ENTRY_CONT_HEADSZ as u64) + (data.len() as u64);
                while (i < g) && (data.len() < CHUNK_ENTRY_CONT_DATASZ)
                {
                    insertion_points.push(file_position);
                    data.extend_from_slice(&fields_total_data[i]);
                    file_position += fields_total_data[i].len() as u64;
                    i += 1;
                }

                carry_data = data.split_off(CHUNK_ENTRY_CONT_DATASZ);
                written_len += data.len();

                // Layout of the continued entry chunk!
                //
                let mut chunk_data = binary_data!
                (
                    byte!(header), // Chunk header
                    u64_be!(next_chunk), // Next chunk position in file
                    bytes_from_vec!(data) // Chunk field data
                    // CRC to be appended...
                );

                let crc = ApeCrc24::new(&chunk_data);

                chunk_data.extend_from_slice(&crc.to_be_bytes());

                self.file.write_all(&chunk_data)?;
            }
            else // and stub entry chunks
            {
                let mut file_position = self.file.seek(SeekFrom::End(0))?; // Seek to the end of the file and get the position

                let header = CHUNK_TYPE::ENTRY; // Set the header to represent a bare entry chunk
                let mut data = Vec::<u8>::with_capacity(CHUNKSZ);
                
                if carry_data.len() > 0
                {
                    data.extend_from_slice(&carry_data);
                    carry_data.clear();
                }

                file_position += CHUNK_ENTRY_STUB_HEADSZ as u64;
                while i < g
                {
                    insertion_points.push(file_position);
                    data.extend_from_slice(&fields_total_data[i]);
                    file_position += fields_total_data[i].len() as u64;
                    i += 1;
                }

                let data_length: u8 = data.len().try_into().expect("Stub chunk data length over 255! You shouldn't see this!");
                let padding = vec![0; CHUNK_ENTRY_STUB_DATASZ - (data_length as usize)];

                // Layout of the stub entry chunk!
                //
                let mut chunk_data = binary_data!
                (
                    byte!(header), // Header
                    byte!(data_length), // length of the following data...
                    bytes_from_vec!(data), // Padding...
                    bytes_from_vec!(padding) // Data...
                    // CRC to be appended later...
                );

                let crc = ApeCrc24::new(&chunk_data);

                chunk_data.extend_from_slice(&crc.to_be_bytes());

                self.file.write_all(&chunk_data)?;
            }

            
        }

        return Ok(insertion_points);
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



pub struct EntryChunk
{
    //pub chunk_numbers: Vec<u64>,
    pub fields: Vec<Field>,
}

impl EntryChunk
{
    pub fn new(entry: Entry) -> Self
    {
        return Self
        {
            //chunk_numbers: Vec::<u64>::new(),
            fields: entry.fields,
        };
    }
}