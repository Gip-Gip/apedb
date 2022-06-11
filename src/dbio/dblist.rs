
use crate::dbio::dbtree::LazyAVL;
use std::error::Error;
use simple_error::*;
use crate::dbio::dbstruct::Structure;
use crate::dbio::dbstruct::Requirement;
use crate::dbio::dbfield::Field;
use crate::dbio::dbuuid::UuidV4;
use crate::dbio::dbchunk::*;
use crate::apetypes::S;
use crate::apetypes::Type;
use apebdlm::*;



#[derive(Debug, Clone, PartialEq)]
pub struct Entry
{
    pub uuid: UuidV4,
    pub fields: Vec<Field>,
}

impl Entry
{
    pub fn new(uuid: UuidV4, fields: Vec<Field>) -> Result<Entry, Box<dyn Error>>
    {

        return Ok
        (
            Entry
            {
                uuid: uuid,
                fields: fields,
            }
        );
    }

    pub fn get_field(&self, field_id: &str) -> Option<&Field>
    {
        for field in &self.fields
        {
            if field.id == field_id
            {
                return Some(&field);
            }
        }

        return None;
    }
}

pub struct List
{
    pub structure: Structure,
    pub tree: LazyAVL,
    pub db_file: ChunkyFile,
    pub entry_count: u64
}

impl List
{
    pub fn new(db_file: ChunkyFile, structure: Structure) -> Result<Self, Box<dyn Error>>
    {
        let tree = LazyAVL::new(db_file.file.try_clone()?, 0, 0);

        return Ok
        (
            Self
            {
                structure: structure,
                tree: tree,
                db_file: db_file,
                entry_count: 0,
            }
        );
    }

    pub fn add_entry(&mut self, entry: Entry) -> Result<(), Box<dyn Error>>
    {   
        let entry_chunk = EntryChunk::new(entry);
        let mut insertion_points = self.db_file.add_entry_chunk(entry_chunk)?;

        if insertion_points.len() == 0
        {
            bail!("Fieldless entry!");
        }

        if self.tree.head == 0
        {
            self.tree.head = insertion_points.pop().expect("Insertion point length check failed! you shouldn't see this!");
        }

        for insertion_point in insertion_points
        {
            self.tree.insert(insertion_point)?;
        }


        return Ok(());
    }
}

// Tests!
//
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_entry_new()
    {
        let uuid = UuidV4::new();
        let uuid2 = uuid.clone();
        let fields = vec![Field::new("id", Type::S(Some(S::new("Test"))))];
        let fields2 = fields.clone();
        let entry = Entry::new(uuid, fields).unwrap();
        assert_eq!(entry.uuid, uuid2);
        assert_eq!(entry.fields, fields2);
    }

    #[test]
    fn test_entry_get_field()
    {
        let uuid = UuidV4::new();
        let fields = vec![Field::new("id", Type::S(Some(S::new("Test"))))];
        let entry = Entry::new(uuid, fields).unwrap();
        assert_eq!(entry.get_field("id").unwrap().value, Type::S(Some(S::new("Test"))));
    }

    #[test]
    fn test_list_new()
    {
        let structure = Structure::new("test", vec![Requirement::new("id", std::mem::discriminant(&Type::S(None)))]);
        let db_file = ChunkyFile::create("tests/test_list_new.db").unwrap();

        let list = List::new(db_file, structure.clone()).unwrap();

        assert_eq!(list.structure, structure);
        assert_eq!(list.tree.head, 0);
        assert_eq!(list.entry_count, 0);
    }

    #[test]
    fn test_list_add_entry()
    {
        let structure = Structure::new("test", vec![Requirement::new("id", std::mem::discriminant(&Type::S(None)))]);
        let db_file = ChunkyFile::create("tests/test_list_add_entry.db").unwrap();

        let mut list = List::new(db_file, structure).unwrap();

        let uuid1 = UuidV4::new();
        let fields1 = vec![Field::new("id", Type::S(Some(S::new("Test1"))))];
        let entry1 = Entry::new(uuid1, fields1).unwrap();

        let uuid2 = UuidV4::new();
        let fields2 = vec![Field::new("id", Type::S(Some(S::new("Test2"))))];
        let entry2 = Entry::new(uuid2, fields2).unwrap();

        let uuid3 = UuidV4::new();
        let fields3 = vec![Field::new("id", Type::S(Some(S::new("Test3"))))];
        let entry3 = Entry::new(uuid3, fields3).unwrap();

        list.add_entry(entry1).unwrap();
        list.add_entry(entry2).unwrap();
        list.add_entry(entry3).unwrap();
    }
}