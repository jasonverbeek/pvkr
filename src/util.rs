use crate::error::Result;
use std::io::{Read, Write};

pub trait Streamable: Sized {
    fn write_to<W: Write>(&self, s: W) -> Result<()>;
    fn read_from<R: Read>(s: R) -> Result<Self>;
}

impl Streamable for u128 {
    fn write_to<W: Write>(&self, mut s: W) -> Result<()> {
        s.write_all(&self.to_be_bytes())?;
        Ok(())
    }

    fn read_from<R: Read>(mut s: R) -> Result<Self> {
        let mut buffer = [0; 16];
        s.read_exact(&mut buffer)?;
        Ok(u128::from_be_bytes(buffer))
    }
}
