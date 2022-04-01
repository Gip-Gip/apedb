// dbstruct.rs - Contains everything related to database structures
//


use crate::dbio::dbfield::*;
use crate::apetypes::*;



// Structs!
//



// dbio::dbstruct::Requirement - A single requirement in a database structure
//
pub struct Requirement
{
    pub field_id: String, // The ID of the required field
    pub field_type: Type, // The type of the required field
}

impl Requirement
{
    // dbio::dbstruct::Requirement::new - Simple requirement constructor
    //
    // ARGUMENTS:
    //  field_id: &str - A string containing the ID of the required field
    //  field_type: Type - The type of the required field
    pub fn new(field_id: &str, field_type: Type) -> Requirement
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
            std::mem::discriminant(&self.field_type) == std::mem::discriminant(&field.value);
    }
}



// dbio::dbstruct::Structure - A database structure
//
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