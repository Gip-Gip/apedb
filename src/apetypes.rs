
pub enum Type
{
    S(S),
}

pub struct S
{
    string: String,
}

impl S
{
    pub fn new(string: String) -> S
    {
        return S
        {
            string: string,
        };
    }

    pub fn to_bytes(&self) -> Vec<u8>
    {
        return self.string.as_bytes().to_vec();
    }
}
