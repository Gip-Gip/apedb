// dbfield.rs - Contains everything related to fields



use std::io::Read;
use std::io::Seek;
use std::fs::File;
use crate::apetypes::*;
use std::error::Error;
use simple_error::*;
use apebdlm::*;

// Constants!
//

pub const FIELDHEADSZ: usize = 17; // 1 header byte + pointer to left child + pointer to right child

// Enums!
//

pub enum FieldCmp
{
    Equal,
    GreaterThan,
    LessThan
}

// Structs!
//



// dbio::dbfield::Field - A data structure used to assign IDs to values.
//
#[derive(Debug, Clone, PartialEq)]
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
                string.as_ref().unwrap().to_bytes()
            }
            Type::I(integer) =>
            {
                integer.as_ref().unwrap().to_bytes()
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
                if boolean.as_ref().unwrap().is_true()
                {
                    b'B'
                }
                else
                {
                    b'b'
                }
            }
        };

        let data = match &self.value
        {
            Type::S(_) | Type::I(_) => // Types S and I have the same binary layout...
            {
                binary_data!
                (
                    byte!(0), // Header byte, used for binary tree metadata
                    u64_be!(0), // Pointer to left child
                    u64_be!(0), // Pointer to right child
                    byte!(value_type), // The type
                    byte!(id_length), // The length of the ID in bytes (max 255)
                    bytes_from_vec!(id_data), // The ID
                    byte!(value_data.len()), // The length of the value in bytes (max 255)
                    bytes_from_vec!(value_data) // The value
                )
            }
            Type::B(_) => // Booleans do not need a data field so that is omitted...
            {
                binary_data!
                (
                    byte!(0), // Header byte, used for binary tree metadata
                    u64_be!(0), // Pointer to left child
                    u64_be!(0), // Pointer to right child
                    byte!(value_type), // The type
                    byte!(id_length), // The length of the ID in bytes (max 255)
                    bytes_from_vec!(id_data) // The ID
                )
            }
        };

        return Ok(data);
    }

    // dbio::dbfield::Field::from_bytes - Converts bytes to a field.
    //
    // ARGUMENTS:
    //  data: &[u8] - A slice of bytes to be converted to a field
    pub fn from_bytes(data: &[u8]) -> Result<Field, Box<dyn Error>>
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
                Type::S(Some(S::from_bytes(value_data)?))
            }
            _ =>
            {
                return Err(Box::new(SimpleError::new("Invalid value type!")));
            }
        };

        return Ok(Field::new(&id, value));
    }

    pub fn cmp_in_file(mut file: File, field_point_a: u64, field_point_b: u64) -> Result<FieldCmp, Box<dyn Error>>
    {
        // Does not work with continued chunks, to be implemented!
        // Implement a buffer of 256 bytes in size for both the a and b fields
        let mut buffer_a: [u8;256] = [0; 256];
        let mut buffer_b: [u8;256] = [0; 256];

        // Skip past the header on the first field point since it is kinda useless for what we need to do...
        file.seek(std::io::SeekFrom::Start(field_point_a + (FIELDHEADSZ as u64)))?;
        file.read(&mut buffer_a)?; // Read the first field

        // Skip past the header on the second field point since it is kinda useless for what we need to do...
        file.seek(std::io::SeekFrom::Start(field_point_b + (FIELDHEADSZ as u64)))?;
        file.read(&mut buffer_b)?; // Read the second field

        // Initialize the iterator...
        let mut i: usize = 0;

        // Get the ID length
        let id_len_a = buffer_a[i];
        let id_len_b = buffer_b[i];

        // Compare the id lengths for a difference...

        if id_len_a > id_len_b
        {
            return Ok(FieldCmp::GreaterThan);
        }

        if id_len_a < id_len_b
        {
            return Ok(FieldCmp::LessThan);
        }

        // Increment the iterator
        i += 1;
        // Set the start point...
        let mut start_i = i;

        // Compare the id data for a difference...
        while (i - start_i) < (id_len_a as usize)
        {
            if buffer_a[i] > buffer_b[i]
            {
                return Ok(FieldCmp::GreaterThan);
            }

            if buffer_a[i] < buffer_b[i]
            {
                return Ok(FieldCmp::LessThan);
            }

            // Increment the iterator
            i += 1;
        }

        // Compare the value lengths for a difference...
        let value_len_a = buffer_a[i];
        let value_len_b = buffer_b[i];

        if value_len_a > value_len_b
        {
            return Ok(FieldCmp::GreaterThan);
        }

        if value_len_a < value_len_b
        {
            return Ok(FieldCmp::LessThan);
        }

        // Increment the iterator
        i += 1;
        // Set the start point...
        start_i = i;

        // Compare the value data for a difference...
        while (i - start_i) < (value_len_a as usize)
        {
            if buffer_a[i] > buffer_b[i]
            {
                return Ok(FieldCmp::GreaterThan);
            }

            if buffer_a[i] < buffer_b[i]
            {
                return Ok(FieldCmp::LessThan);
            }

            // Increment the iterator
            i += 1;
        }


        return Ok(FieldCmp::Equal);
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
        let field = Field::new(id, Type::S(Some(S::new(value))));

        assert_eq!(field.to_bytes().unwrap(), test_string_data);
    }

    // dbio::dbfield::test::test_field_from_bytes_string() - Tests the from_bytes function for a string field.
    //
    #[test]
    fn test_field_from_bytes_string()
    {
        let id = "Hello";
        let value = "World";
        let field = Field::new(id, Type::S(Some(S::new(value))));

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
        let field = Field::new(id, Type::S(Some(S::new(value))));

        assert_eq!(Field::from_bytes(&field.to_bytes().unwrap()).unwrap(), field);
    }
}