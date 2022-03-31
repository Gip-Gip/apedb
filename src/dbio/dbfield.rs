// dbfield.rs - Contains everything related to fields



use crate::apetypes::*;
use std::error::Error;



// Structs!
//



// dbio::dbfield::Field - A data structure used to assign IDs to values.
//
pub struct Field
{
    id: String, // The ID of the field
    value: Type, // The value of the field
}

impl Field
{
    // dbio::dbfield::Field::new - Simple field constructor
    //
    // ARGUMENTS:
    //  id: &str - A string containing the ID to assign to the field
    //  value: Type - an enum containing the Type and value to be assigned
    pub fn new(id: &str, value: Type) -> Field
    {
        return Field
        {
            id: id.to_string(),
            value: value,
        };
    }

    // dbio::dbfield::Field::to_bytes - Converts a field to bytes.
    //
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>>
    {
        let id_data = self.id.as_bytes();
        let id_length:u8 = id_data.len().try_into().expect("ID length check fail!"); // The maximum length of an ID is 255 bytes, this should have been checked before...

        let value_data:Vec<u8> = match &self.value
        {
            Type::S(string) =>
            {
                string.to_bytes()
            }
            Type::I(integer) =>
            {
                integer.to_bytes()
            }
            Type::B(_) =>
            {
                Vec::<u8>::new() // Boolean's values are stored in their type, there is no value to store
            }
        };

        let value_type: u8 = match &self.value
        {
            Type::S(_) =>
            {
                b'S'
            }
            Type::I(_) =>
            {
                b'I'
            }
            Type::B(boolean) =>
            {
                // If the boolean is true, the type is an uppercase 'B', otherwise the type is a lowercase 'b'
                if boolean.is_true()
                {
                    b'B'
                }
                else
                {
                    b'b'
                }
            }
        };

        let mut data = Vec::<u8>::new();

        // Add, in order...
        data.push(value_type); // The type of the value(a single case-sensitive ascii letter)
        data.push(id_length); // The length of the ID string
        data.extend_from_slice(&id_data); // The ID string

        match &self.value
        {
            // If the type is an I or an S, the length of the value only needs to be one byte long...
            Type::S(_) | Type::I(_) =>
            {
                let value_length:u8 = value_data.len().try_into().expect("Size of field too big for u8");
                data.push(value_length); // Store the length...
            }

            // If the type is a B, there is no value to store, so the length is zero!
            Type::B(_) =>
            {
                // Nothing to do here
            }
        }

        // If there is a value, add it to the buffer...
        if value_data.len() > 0
        {
            data.extend_from_slice(&value_data);
        }

        return Ok(data);
    }
}
