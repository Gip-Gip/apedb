// apetypes.rs - Types used in the database



use std::error::Error;



// Enums!
//



// apetypes::Type - Enum for the different types of data that can be stored in the database.
//
#[derive(Debug)]
pub enum Type
{
    I(I), // Integer
    S(S), // String
    B(B), // Boolean
}


// Implement equality function for Type
impl PartialEq for Type
{
    fn eq(&self, other: &Self) -> bool
    {
        return match (self, other)
        {
            (Type::I(i1), Type::I(i2)) => i1 == i2,
            (Type::S(s1), Type::S(s2)) => s1 == s2,
            (Type::B(b1), Type::B(b2)) => b1 == b2,
            _ => false,
        };
    }
}

// Structs!
//



// apetypes::S - Database string type
//
#[derive(Debug)]
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

impl PartialEq for S
{
    fn eq(&self, other: &Self) -> bool
    {
        return self.string == other.string;
    }
}

// apetypes::I - Database integer type
//
#[derive(Debug)]
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
}

impl PartialEq for I
{
    fn eq(&self, other: &Self) -> bool
    {
        return self.most_significant == other.most_significant;
    }
}

// apetypes::B - Database boolean type
//
#[derive(Debug)]
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

impl PartialEq for B
{
    fn eq(&self, other: &Self) -> bool
    {
        return self.boolean == other.boolean;
    }
}