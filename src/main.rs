mod dbio;
mod apetypes;

use crate::apetypes::*;
use crate::dbio::dbfield::*;
use crate::dbio::dbchunk::*;

use std::time::SystemTime;

fn main()
{
    let name = S::new("Ape Database!".to_string());
    let name_field = Field::new("name".to_string(), Type::S(name));

    let version = S::new("0.1.0".to_string());
    let version_field = Field::new("version".to_string(), Type::S(version));

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

    let mut db_chunk = DbHeadChunk::new();

    db_chunk.add_field(name_field);
    db_chunk.add_field(version_field);

    chunky_db.add_chunk(ChunkTypes::DbHead(db_chunk));
}
