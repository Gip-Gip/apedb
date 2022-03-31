// apetypes.rs - Types used in the database



// Enums!
//



// apetypes::Type - Enum for the different types of data that can be stored in the database.
//
pub enum Type
{
    I(I), // Integer
    S(S), // String
    B(B), // Boolean
}



// Structs!
//



// apetypes::S - Database string type
//
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
}

// apetypes::I - Database integer type
//
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

// apetypes::B - Database boolean type
//
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