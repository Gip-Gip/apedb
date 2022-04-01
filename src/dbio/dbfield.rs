// dbfield.rs - Contains everything related to fields



use crate::apetypes::*;
use std::error::Error;
use simple_error::*;

// Structs!
//



// dbio::dbfield::Field - A data structure used to assign IDs to values.
//
#[derive(Debug)]
pub struct Field
{
    pub id: String, // The ID of the field
    pub value: Type, // The value of the field
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

    // dbio::dbfield::Field::from_bytes - Converts bytes to a field.
    //
    // ARGUMENTS:
    //  data: &[u8] - A slice of bytes to be converted to a field
    fn from_bytes(data: &[u8]) -> Result<Field, Box<dyn Error>>
    {
        if data.len() < 3
        {
            return Err(Box::new(SimpleError::new("Field data is too short!")));
        }

        let mut index:usize = 0;

        // Get the value type
        let value_type:u8 = data[index];
        index += 1;

        // Get the ID length
        let id_length:u8 = data[index];
        if id_length as usize + index + 1 > data.len()
        {
            return Err(Box::new(SimpleError::new("Field data is too short!")));
        }
        index += 1;

        // Get the ID data
        let id_data = &data[index..(index+id_length as usize)];
        index += id_length as usize;

        let id = String::from_utf8(id_data.to_vec())?;

        // Get the value length
        let value_length:u8 = data[index];
        if value_length as usize + index + 1 > data.len()
        {
            return Err(Box::new(SimpleError::new("Field data is too short!")));
        }
        index += 1;

        // Get the value data
        let value_data = &data[index..(index+value_length as usize)];

        let value = match value_type
        {
            b'S' =>
            {
                Type::S(S::from_bytes(value_data)?)
            }
            _ =>
            {
                return Err(Box::new(SimpleError::new("Invalid value type!")));
            }
        };

        return Ok(Field::new(&id, value));
    }
}

// Implement equality function for Field
impl PartialEq for Field
{
    fn eq(&self, other: &Field) -> bool
    {
        return self.id == other.id && self.value == other.value;
    }
}

// Tests!
//

#[cfg(test)]
mod test
{
    use super::*;

    // Test data for all string field tests. Consists of an id equal to "Hello" and a value equal to "World"
    // The total size should be equal to 13 bytes.
    // The first byte should be a 'S' to denote the type, the second byte should be the length of the string "Hello"(5). After those two bytes, the string "Hello" should be stored.
    // The byte following the id string should be the length of the string "World"(5). After that, the string "World" should be stored.
    const test_string_data:[u8; 13] = [b'S', 5, b'H', b'e', b'l', b'l', b'o', 5, b'W', b'o', b'r', b'l', b'd'];
    const test_string_data_invalid:[u8; 12] = [b'S', 5, b'H', b'e', b'l', b'l', b'o', 5, b'W', b'o', b'r', b'l'];

    // dbio::dbfield::test::test_field_to_bytes_string() - Tests the to_bytes function for a string field.
    //
    #[test]
    fn test_field_to_bytes_string()
    {
        let id = "Hello";
        let value = "World";
        let field = Field::new(id, Type::S(S::new(value)));

        assert_eq!(field.to_bytes().unwrap(), test_string_data);
    }

    // dbio::dbfield::test::test_field_from_bytes_string() - Tests the from_bytes function for a string field.
    //
    #[test]
    fn test_field_from_bytes_string()
    {
        let id = "Hello";
        let value = "World";
        let field = Field::new(id, Type::S(S::new(value)));

        assert_eq!(Field::from_bytes(&test_string_data).unwrap(), field);
    }

    // dbio::dbfield::test::test_field_to_bytes_string_invalid() - Tests the to_bytes function for a string field.
    //
    #[test]
    fn test_field_from_bytes_string_invalid()
    {
        match Field::from_bytes(&test_string_data_invalid)
        {
            Err(error) =>
            {
                assert_eq!(error.to_string(), "Field data is too short!");
            }
            Ok(_) =>
            {
                panic!("Error check failed!");
            }
        }
    }

    // dbio::dbfield::test::test_field_from_bytes_string_null() - Tests to see if the from bytes function handles empty byte vectors properly
    //
    #[test]
    fn test_field_from_bytes_string_null()
    {
        match Field::from_bytes(&[])
        {
            Err(error) =>
            {
                assert_eq!(error.to_string(), "Field data is too short!");
            }
            Ok(_) =>
            {
                panic!("Error check failed!");
            }
        }
    }

    // dbio::dbfield::test::test_field_to_from_bytes() - Tests passing the to_bytes function to the from_bytes function
    //
    #[test]
    fn test_field_to_from_bytes()
    {
        let id = "Hello";
        let value = "World";
        let field = Field::new(id, Type::S(S::new(value)));

        assert_eq!(Field::from_bytes(&field.to_bytes().unwrap()).unwrap(), field);
    }
}