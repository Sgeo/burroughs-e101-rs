use crate::ints::Word12;


pub trait IO {
    type Error;

    fn read(&mut self) -> Result<Word12, Self::Error>;
    fn print(&mut self, output: Word12) -> Result<(), Self::Error>;
}