// dbstruct.rs - Contains everything related to database structures
//



use std::mem::*;
use crate::dbio::dbfield::*;
use crate::apetypes::*;



// Structs!
//



// dbio::dbstruct::Requirement - A single requirement in a database structure
//
#[derive(Debug, Clone, PartialEq)]
pub struct Requirement
{
    pub field_id: String, // The ID of the required field
    pub field_type: Discriminant<Type>, // The type of the required field
}

impl Requirement
{
    // dbio::dbstruct::Requirement::new - Simple requirement constructor
    //
    // ARGUMENTS:
    //  field_id: &str - A string containing the ID of the required field
    //  field_type: Type - The type of the required field
    pub fn new(field_id: &str, field_type: Discriminant<Type>) -> Requirement
    {
        return Requirement
        {
            field_id: field_id.to_string(),
            field_type: field_type,
        };
    }

    // dbio::dbstruct::Requirement::meets - Checks if a field meets the requirement
    //
    // ARGUMENTS:
    //  field: &Field - The field to check
    pub fn meets(&self, field: &Field) -> bool
    {
        return self.field_id == field.id &&
            self.field_type == std::mem::discriminant(&field.value);
    }
}



// dbio::dbstruct::Structure - A database structure
//
#[derive(Debug, Clone, PartialEq)]
pub struct Structure
{
    id: String, // The ID of the structure
    requirements: Vec<Requirement>,
}

impl Structure
{
    // dbio::dbstruct::Structure::new - Simple structure constructor
    //
    // ARGUMENTS:
    //  id: &str - A string containing the ID of the structure
    //  requirements: Vec<Requirement> - A vector of requirements to be met
    pub fn new(id: &str, mut requirements: Vec<Requirement>) -> Structure
    {
        requirements.sort_by(|a, b| a.field_id.cmp(&b.field_id)); // Sort the requirements by ID so we can binary search them later

        return Structure
        {
            id: id.to_string(),
            requirements: requirements,
        };
    }

    // dbio::dbstruct::Structure::meets - Checks if a structure meets the requirements
    //
    // ARGUMENTS:
    // fields: &Vec<Field> - A vector of fields to check
    pub fn meets(&self, fields: &Vec<Field>) -> bool
    {
        for field in fields
        {
            match self.requirements.binary_search_by(|req| req.field_id.cmp(&field.id))
            {
                Ok(index) =>
                {
                    if !self.requirements[index].meets(field)
                    {
                        return false;
                    }
                }
                Err(_) =>
                {
                    return false;
                }
            }
        }

        return true;
    }
}

// Tests!
//

#[cfg(test)]
mod tests
{
    use super::*;

    // dbio::dbstruct::tests::test_requirement_new() - Tests the creation of a Requirement
    //
    #[test]
    fn test_requirement_new()
    {
        let disc = std::mem::discriminant(&Type::S(None));
        let req = Requirement::new("id", disc);
        assert_eq!(req.field_id, "id");
        assert_eq!(req.field_type, disc);
    }

    // dbio::dbstruct::tests::test_requirement_meets() - Tests the requirement meets function
    //
    #[test]
    fn test_requirement_meets()
    {
        let disc = std::mem::discriminant(&Type::S(None));
        let req = Requirement::new("id", disc);
        let field = Field::new("id", Type::S(Some(S::new("Test"))));
        assert!(req.meets(&field));
    }

    // dbio::dbstruct::tests::test_requirement_doesnt_meet_type() - Tests the requirement doesn't meet function, when the type doesn't match
    //
    #[test]
    fn test_requirement_doesnt_meet_type()
    {
        let disc = std::mem::discriminant(&Type::S(None));
        let req = Requirement::new("id", disc);
        let field = Field::new("id", Type::I(Some(I::new(10))));
        assert!(!req.meets(&field)); // Test fails because the field is not the same type
    }

    // dbio::dbstruct::tests::test_requirement_doesnt_meet_id() - Tests the requirement doesn't meet function, when the ID doesn't match
    //
    #[test]
    fn test_requirement_doesnt_meet_id()
    {
        let disc = std::mem::discriminant(&Type::S(None));
        let req = Requirement::new("id", disc);
        let field = Field::new("id2", Type::S(Some(S::new("Test"))));
        assert!(!req.meets(&field)); // Test fails because the field is not the same ID
    }

    // dbio::dbstruct::tests::test_structure_new() - Tests the creation of a Structure
    //
    #[test]
    fn test_structure_new()
    {
        let disc = std::mem::discriminant(&Type::S(None));
        let req = Requirement::new("id", disc);
        let mut requirements = Vec::new();
        requirements.push(req);
        let requirements2 = requirements.clone();
        let structure = Structure::new("id", requirements);
        assert_eq!(structure.id, "id");
        assert_eq!(structure.requirements, requirements2);
    }

    // dbio::dbstruct::tests::test_structure_meets() - Tests the structure meets function
    //
    #[test]
    fn test_structure_meets()
    {
        let disc = std::mem::discriminant(&Type::S(None));
        let req = Requirement::new("id", disc);
        let field = Field::new("id", Type::S(Some(S::new("Test"))));
        let mut requirements = Vec::new();
        requirements.push(req);
        let structure = Structure::new("id", requirements);
        let mut fields = Vec::new();
        fields.push(field);
        assert!(structure.meets(&fields));
    }

    // dbio::dbstruct::tests::test_structure_doesnt_meet_type() - Tests the structure doesn't meet function, when the type doesn't match
    //
    #[test]
    fn test_structure_doesnt_meet_type()
    {
        let disc = std::mem::discriminant(&Type::S(None));
        let req = Requirement::new("id", disc);
        let field = Field::new("id", Type::I(Some(I::new(10))));
        let mut requirements = Vec::new();
        requirements.push(req);
        let structure = Structure::new("id", requirements);
        let mut fields = Vec::new();
        fields.push(field);
        assert!(!structure.meets(&fields)); // Test fails because the field is not the same type
    }

    // dbio::dbstruct::tests::test_structure_doesnt_meet_id() - Tests the structure doesn't meet function, when the ID doesn't match
    //
    #[test]
    fn test_structure_doesnt_meet_id()
    {
        let disc = std::mem::discriminant(&Type::S(None));
        let req = Requirement::new("id", disc);
        let field = Field::new("id2", Type::S(Some(S::new("Test"))));
        let mut requirements = Vec::new();
        requirements.push(req);
        let structure = Structure::new("id", requirements);
        let mut fields = Vec::new();
        fields.push(field);
        assert!(!structure.meets(&fields)); // Test fails because the field is not the same ID
    }
}