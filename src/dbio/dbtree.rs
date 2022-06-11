// To be implemented...

use std::io::Write;
use crate::Field;
use crate::dbio::dbfield::FieldCmp;
use std::io::Read;
use std::error::Error;
use std::io::Seek;
use std::fs::File;

pub mod LAZY_AVL_CONST
{
    pub const LAZE_MAX: u8 = 127;
    pub const BF_OFFSET: u64 = 0;
    pub const LC_OFFSET: u64 = 1;
    pub const RC_OFFSET: u64 = 9;
}

pub struct LazyAVL
{
    file: File,
    pub head: u64,
    laze: u8,
}

impl LazyAVL
{
    pub fn new(file: File, head: u64, laze: u8) -> Self
    {
        if(laze > LAZY_AVL_CONST::LAZE_MAX)
        {
            panic!("There was an attempt to create a VERY lazy avl tree, you should not see this!");
        }

        return Self
        {
            file,
            head,
            laze
        };
    }

    pub fn field_change_left_child(&mut self, field_pos: u64, new_child: u64) -> Result<(), Box<dyn Error>>
    {
        let new_child_data = new_child.to_be_bytes();
        self.file.seek(std::io::SeekFrom::Start(field_pos + LAZY_AVL_CONST::LC_OFFSET))?;
        self.file.write_all(&new_child_data)?;

        return Ok(());
    }

    pub fn field_change_right_child(&mut self, field_pos: u64, new_child: u64) -> Result<(), Box<dyn Error>>
    {
        let new_child_data = new_child.to_be_bytes();
        self.file.seek(std::io::SeekFrom::Start(field_pos + LAZY_AVL_CONST::RC_OFFSET))?;
        self.file.write_all(&new_child_data)?;

        return Ok(());
    }

    pub fn insert(&mut self, field_pos: u64) -> Result<(), Box<dyn Error>>
    {
        let mut field_data: [u8;256] = [0; 256];
        let mut node_history: Vec<u64> = Vec::new();
        self.file.seek(std::io::SeekFrom::Start(field_pos))?;
        self.file.read(&mut field_data)?;
        let field_to_insert = Field::from_bytes(&field_data)?;
        
        let mut current_node_pos = self.head;
        let mut greater_than: bool = false;

        while current_node_pos != 0
        {
            self.file.seek(std::io::SeekFrom::Start(current_node_pos))?;
            self.file.read(&mut field_data)?;

            node_history.push(current_node_pos);

            let current_node = Field::from_bytes(&field_data)?;

            current_node_pos = match current_node.cmp(&field_to_insert)?
            {
                FieldCmp::Equal | FieldCmp::LessThan =>
                {
                    greater_than = false;
                    current_node.left_child
                }

                FieldCmp::GreaterThan =>
                {
                    greater_than = true;
                    current_node.right_child
                }
            }
        }

        current_node_pos = node_history.pop().unwrap();

        if greater_than
        {
            self.field_change_right_child(current_node_pos, field_pos)?;
        }
        else
        {
            self.field_change_left_child(current_node_pos, field_pos)?;
        }
        
        return Ok(());
    }
}