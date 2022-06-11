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

pub const FIELDHEADSZ: usize = 18; // 1 header byte + pointer to left child + pointer to right child + field type

// Enums!
//

#[derive(Debug, Clone, PartialEq)]
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
    pub avl_balace: i8,
    pub left_child: u64,
    pub right_child: u64,
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
            avl_balace: 0,
            left_child: 0,
            right_child: 0,
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
                    byte_signed!(self.avl_balace), // Header byte, used for binary tree metadata
                    u64_be!(self.left_child), // Pointer to left child
                    u64_be!(self.right_child), // Pointer to right child
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
                    byte_signed!(self.avl_balace), // Header byte, used for binary tree metadata
                    u64_be!(self.left_child), // Pointer to left child
                    u64_be!(self.right_child), // Pointer to right child
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
        // Check to make sure the length of the data isn't too short(to be implemented)

        // Use an iterator through the data to keep track of where we are...
        let mut i: usize = 0;
        // Get all the fixed header data...

        let avl_balance: i8 = data[i] as i8; // Get the avl balance...
        i += 1;

        let left_child: u64 = u64::from_be_bytes(data[i..i+8].try_into().expect("Slice of incorrect size when reading the left child of an entry, you shouldn't see this!")); // Get the left child pointer...
        i += 8;

        let right_child: u64 = u64::from_be_bytes(data[i..i+8].try_into().expect("Slice of incorrect size when reading the right child of an entry, you shouldn't see this!")); // Get the right child pointer...
        i += 8;

        let value_type_byte: u8 = data[i]; // Get the value type...
        i += 1;

        // Get the ID...
        let id_length: u8 = data[i]; // Get the length of the ID...
        // Check to see if the ID length doesn't make sense (to be implemented)...
        i += 1;

        let id_data: Vec<u8> = data[i..i+id_length as usize].to_vec(); // Get the ID data...
        i += id_length as usize;

        // Get the value data...
        // Note that the way we extrapolate the data depends on the value type...
        let value = match value_type_byte
        {
            b'S' =>
            {
                let value_length: u8 = data[i]; // Get the length of the value...
                i += 1;
                let value_data: Vec<u8> = data[i..i+value_length as usize].to_vec(); // Get the value data...

                Type::S(Some(S::from_bytes(&value_data)?)) // Set value to a string...
            }

            b'I' =>
            {
                let value_length: u8 = data[i]; // Get the length of the value...
                i += 1;
                let value_data: Vec<u8> = data[i..i+8].to_vec(); // Get the value data, should work with different sized integers(to be implemented)...

                Type::I(Some(I::from_bytes(&value_data)?)) // Set value to an integer...
            }

            b'B' =>
            {
                Type::B(Some(B::new(true))) // Set value to a boolean...
            }

            b'b' =>
            {
                Type::B(Some(B::new(false))) // Set value to a boolean...
            }

            _ =>
            {
                bail!("Invalid value type byte!");
            }
        };

        return Ok
        (
            Field
            {
                avl_balace: avl_balance,
                left_child: left_child,
                right_child: right_child,
                id: String::from_utf8(id_data.to_vec())?,
                value: value,
            }
        )
    }

    pub fn cmp(&self, field_b: &Field) -> Result<FieldCmp, Box<dyn Error>>
    {
        if(self.id < field_b.id)
        {
            return Ok(FieldCmp::LessThan);
        }
        if(self.id > field_b.id)
        {
            return Ok(FieldCmp::GreaterThan);
        }

        if(self.value < field_b.value)
        {
            return Ok(FieldCmp::LessThan);
        }
        if(self.value > field_b.value)
        {
            return Ok(FieldCmp::GreaterThan);
        }

        return Ok(FieldCmp::Equal);
    }

    pub fn cmp_in_file(file: &mut File, field_point_a: u64, field_point_b: u64) -> Result<FieldCmp, Box<dyn Error>>
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
    use std::fs::OpenOptions;
use std::fs::remove_file;
use std::io::Write;
    use std::mem::drop;
    use super::*;

    // Test data for all string field tests. Consists of an id equal to "Hello" and a value equal to "World"
    // The total size should be equal to 13 bytes.
    // The first byte should be a 'S' to denote the type, the second byte should be the length of the string "Hello"(5). After those two bytes, the string "Hello" should be stored.
    // The byte following the id string should be the length of the string "World"(5). After that, the string "World" should be stored.
    const test_string_data:[u8; 13] = [b'S', 5, b'H', b'e', b'l', b'l', b'o', 5, b'W', b'o', b'r', b'l', b'd'];
    const test_string_data_invalid:[u8; 12] = [b'S', 5, b'H', b'e', b'l', b'l', b'o', 5, b'W', b'o', b'r', b'l'];

    const test_filename: &str = "test.foobar";

    #[test]
    fn test_in_file_cmp_equal()
    {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(test_filename)
            .unwrap();

        let field_a = Field::new("Hello", Type::S(Some(S::new("World"))));
        let field_b = field_a.clone();

        let insertion_point_a = file.seek(std::io::SeekFrom::End(0)).unwrap();
        file.write(&field_a.to_bytes().unwrap()).unwrap();
        
        let insertion_point_b = file.seek(std::io::SeekFrom::End(0)).unwrap();
        file.write(&field_b.to_bytes().unwrap()).unwrap();

        let cmp = Field::cmp_in_file(&mut file, insertion_point_a, insertion_point_b).unwrap();

        assert_eq!(cmp, FieldCmp::Equal);

        drop(file);

        remove_file(test_filename).unwrap();
    }
}