


use std::error::Error;
use simple_error::*;
use crate::dbio::dbstruct::Structure;
use crate::dbio::dbstruct::Requirement;
use crate::dbio::dbfield::Field;
use crate::dbio::dbuuid::UuidV4;
use crate::apetypes::S;
use crate::apetypes::Type;
use apebdlm::*;



#[derive(Debug, Clone, PartialEq)]
pub struct TreeField
{
    pub field: Field,
    pub parent_set: bool,
    pub left_child_set: bool,
    pub right_child_set: bool,
    pub parent_position: u64,
    pub left_child_position: u64,
    pub right_child_position: u64,
}

impl TreeField
{
    pub fn new(field: Field) -> Self
    {
        return Self
        {
            field: field,
            parent_set: false,
            left_child_set: false,
            right_child_set: false,
            parent_position: 0,
            left_child_position: 0,
            right_child_position: 0,
        };
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>>
    {
        let mut data = binary_data!
        (
            u64_be!(self.parent_position),
            u64_be!(self.left_child_position),
            u64_be!(self.right_child_position),
            bytes_from_vec!(self.field.to_bytes()?)
        );

        return Ok(data);
    }
}



#[derive(Debug, Clone, PartialEq)]
pub struct Entry
{
    pub uuid: UuidV4,
    pub fields: Vec<TreeField>,
    pub greater_than: u64,
    pub less_than: u64,
    pub parent: u64,
    pub relations_initialized: bool,
}

impl Entry
{
    pub fn new(uuid: UuidV4, fields: Vec<Field>) -> Result<Entry, Box<dyn Error>>
    {
        let mut tree_fields: Vec<TreeField> = Vec::new();

        for field in fields
        {
            tree_fields.push(TreeField::new(field));
        }

        return Ok
        (
            Entry
            {
                uuid: uuid,
                fields: tree_fields,
                greater_than: 0,
                less_than: 0,
                parent: 0,
                relations_initialized: false,
            }
        );
    }

    pub fn get_field(&self, field_id: &str) -> Option<&Field>
    {
        for field in &self.fields
        {
            if field.field.id == field_id
            {
                return Some(&field.field);
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
    fn test_tree_field_new()
    {
        let field = Field::new("test_field", Type::S(None));
        let tree_field = TreeField::new(field);

        assert_eq!(tree_field.field.id, "test_field");
        assert_eq!(tree_field.field.value, Type::S(None));
        assert_eq!(tree_field.parent_set, false);
        assert_eq!(tree_field.left_child_set, false);
        assert_eq!(tree_field.right_child_set, false);
        assert_eq!(tree_field.parent_position, 0);
        assert_eq!(tree_field.left_child_position, 0);
        assert_eq!(tree_field.right_child_position, 0);
    }

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