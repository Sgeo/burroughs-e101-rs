use crate::ints::Word12;


pub trait IO {
    type Error: std::error::Error;

    fn keyboard(&mut self) -> Result<Word12, Self::Error>;
    fn print(&mut self, output: Word12) -> Result<(), Self::Error>;
}