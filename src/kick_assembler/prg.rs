use std::path::Path;
use std::{fs, io};

/// A program.
pub struct Prg {
    /// Address.
    pub address: u16,
    /// Contents.
    pub contents: Vec<u8>,
}

pub(crate) fn read_prg<P: AsRef<Path>>(path: P) -> Result<Prg, io::Error> {
    let path = path.as_ref();
    let mut contents = fs::read(path)?;

    let byte0 = contents.remove(0);
    let byte1 = contents.remove(0);

    Ok(Prg {
        address: u16::from_le_bytes([byte0, byte1]),
        contents,
    })
}
