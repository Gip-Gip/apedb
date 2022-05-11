mod dbio;
mod apetypes;

//use crate::apetypes::*;
//use crate::dbio::dbfield::*;
use crate::dbio::dbuuid::UuidV4;
use crate::dbio::dblist::Entry;
use crate::apetypes::*;
use crate::dbio::dbfield::Field;
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

    let mut numbers = String::new();

    let mut i = 0;

    while i < 255
    {
        numbers += &i.to_string();
        i += 1;
    }

    let entry_fields = vec![
        Field::new("id", Type::S(Some(S::new("Hello")))),
        Field::new("name", Type::S(Some(S::new("World")))),
        Field::new("numbers", Type::S(Some(S::new(&numbers))))
    ];

    let entry = Entry::new(UuidV4::new(), entry_fields).expect("Failed to create entry!");

    match chunky_db.add_entryChunk(EntryChunk::new(entry))
    {
        Ok(insertion_points) =>
        {
            println!("{:?}", insertion_points)
        }
        Err(e) =>
        {
            panic!("{}", e);
        }
    }
}
