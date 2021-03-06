// apetypes.rs - Types used in the database



use std::error::Error;



// Enums!
//



// apetypes::Type - Enum for the different types of data that can be stored in the database.
//
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Type
{
    I(Option<I>), // Integer
    S(Option<S>), // String
    B(Option<B>), // Boolean
}

// Structs!
//



// apetypes::S - Database string type
//
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct S
{
    string: String, // The string
}

impl S
{
    // apetypes::S::new() - Create a new S from a string
    //
    // ARGUMENTS:
    //  string: String - The string to create the S from
    pub fn new(string: &str) -> S
    {
        return S
        {
            string: string.to_string(),
        };
    }

    // apetypes::S::to_bytes() - Convert the S to a byte array
    //
    pub fn to_bytes(&self) -> Vec<u8>
    {
        return self.string.as_bytes().to_vec();
    }

    // apetypes::S::from_bytes() - Convert a byte array to a S
    //
    // ARGUMENTS:
    //  bytes: &[u8] - The byte array to convert
    pub fn from_bytes(bytes: &[u8]) -> Result<S, Box<dyn Error>>
    {
        return Ok
        (
            S
            {
                string: String::from_utf8(bytes.to_vec())?,
            }
        );
    }
}

// apetypes::I - Database integer type
//
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct I
{
    most_significant: i64,
    //trailing: Vec<u64>,
}

impl I
{
    // apetypes::I::new() - Create a new I type
    //
    // ARGUMENTS:
    //  integer: i64 - The integer to store
    pub fn new(integer: i64) -> I
    {
        return I
        {
            most_significant: integer,
            //trailing: Vec::<u64>::new(),
        };
    }

    // apetypes::I::to_bytes() - Convert an I to a byte array
    //
    pub fn to_bytes(&self) -> Vec<u8>
    {
        let mut data = Vec::<u8>::new();

        data.extend_from_slice(&self.most_significant.to_be_bytes());

        return data;
    }

    // apetypes::I::from_bytes() - Convert a byte array to an I
    //
    // ARGUMENTS:
    //  bytes: &[u8] - The byte array to convert
    pub fn from_bytes(bytes: &[u8]) -> Result<I, Box<dyn Error>>
    {
        return Ok
        (
            I
            {
                most_significant: i64::from_be_bytes(bytes[0..8].try_into().unwrap()),
                //trailing: Vec::<u64>::new(),
            }
        );
    }
}

// apetypes::B - Database boolean type
//
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct B
{
    boolean: bool, // The boolean value
}

impl B
{
    // apetypes::B::new(bool) - Create a new boolean
    //
    // ARGUMENTS:
    //  boolean: bool - The boolean boolean to store
    pub fn new(boolean: bool) -> B
    {
        return B
        {
            boolean: boolean,
        };
    }

    // apetypes::B::is_true() - Check if the boolean is true
    //
    pub fn is_true(&self) -> bool
    {
        return self.boolean;
    }
}