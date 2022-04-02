


use std::error::Error;
use simple_error::*;
use crate::dbio::dbstruct::Structure;
use crate::dbio::dbstruct::Requirement;
use crate::dbio::dbfield::Field;
use crate::dbio::dbuuid::UuidV4;
use crate::apetypes::S;
use crate::apetypes::Type;

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
                return Some(field);
            }
        }

        return None;
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
}