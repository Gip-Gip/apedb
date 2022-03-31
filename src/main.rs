mod dbio;
mod apetypes;

//use crate::apetypes::*;
//use crate::dbio::dbfield::*;
use crate::dbio::dbchunk::*;

// Test function, not made to be pretty...
fn main()
{
    let mut chunky_db = match ChunkyFile::create("test.apedb")
    {
        Ok(chunky) =>
        {
            chunky
        }
        Err(e) =>
        {
            panic!("{}", e);
        }
    };

    let db_chunk = DbHeadChunk::new("Ape Database!", "root");

    chunky_db.add_chunk(ChunkTypes::DbHead(db_chunk)).expect("Oopseies!");
}
