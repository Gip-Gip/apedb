
use crate::apetypes::*;
use simple_error::bail;
use std::error::Error;


pub struct Field
{
    id: String,
    value: Type,
}

impl Field
{
    pub fn new(id: String, value: Type) -> Field
    {
        return Field
        {
            id: id,
            value: value,
        };
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>>
    {
        let id_data = self.id.as_bytes();
        let id_length:u8 = id_data.len().try_into().expect("ID length check fail!");

        let value_data:Vec<u8> = match &self.value
        {
            Type::S(string) =>
            {
                string.to_bytes()
            }
            _=>
            {
                bail!("Unsupported type passed to Field::to_bytes!");
            }
        };

        let value_type: u8 = match &self.value
        {
            Type::S(string) =>
            {
                b'S'
            }
            _=>
            {
                panic!("Previous type check failed in Field::to_bytes, you shouldn't see this!");
            }
        };

        let value_length:u64 = value_data.len().try_into().expect("Size of field to big for u64!");

        let mut data = Vec::<u8>::new();

        data.push(value_type);
        data.push(id_length);

        data.extend_from_slice(&id_data);
        data.extend_from_slice(&value_length.to_be_bytes());
        data.extend_from_slice(&value_data);

        return Ok(data);
    }
}
