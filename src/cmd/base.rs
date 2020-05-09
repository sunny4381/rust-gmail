use crate::error::Error;

pub trait Cmd {
    fn run(&self) -> Result<(), Error>;
}
