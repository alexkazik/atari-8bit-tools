use crate::xex::Segment;
use anyhow::{Context, anyhow};
use std::fs::File;
use std::io;
use std::path::Path;

/// A xex writer.
pub struct XexFile<T> {
    file: T,
}

impl XexFile<File> {
    /// Create a new xex file.
    ///
    /// This function will create a file if it does not exist, and will truncate it if it does.
    ///
    /// For detailed information see [`File::create`](File::create).
    ///
    /// # Errors
    ///
    /// Create/write errors.
    pub fn create<P: AsRef<Path>>(path: P) -> anyhow::Result<XexFile<File>> {
        let path = path.as_ref();
        let file = File::create(path)
            .with_context(|| format!("Failed to create file {}", path.display()))?;
        let mut file = XexFile { file };
        file.write_header()?;
        Ok(file)
    }
}

impl XexFile<Vec<u8>> {
    /// Create a xex file in ram.
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> XexFile<Vec<u8>> {
        let mut file = XexFile { file: Vec::new() };
        #[allow(clippy::missing_panics_doc)]
        file.write_header().unwrap();
        file
    }
}

impl<W: io::Write> XexFile<W> {
    /// Create a xex file in a stream.
    ///
    /// # Errors
    ///
    /// Write errors.
    pub fn stream(file: &mut W) -> anyhow::Result<XexFile<&mut W>> {
        let mut file = XexFile { file };
        file.write_header()?;
        Ok(file)
    }

    /// Create a xex file in a owned stream.
    ///
    /// # Errors
    ///
    /// Write errors.
    pub fn owned_stream(file: W) -> anyhow::Result<XexFile<W>> {
        let mut file = XexFile { file };
        file.write_header()?;
        Ok(file)
    }

    fn write_header(&mut self) -> anyhow::Result<()> {
        self.file.write_all(&[0xff, 0xff])?;
        Ok(())
    }

    /// Write a segment.
    ///
    /// # Errors
    ///
    /// Write errors.
    pub fn write_segment<S: Segment>(&mut self, segment: &S) -> anyhow::Result<()> {
        let address = segment.address();
        let contents = segment.contents();

        let end = contents
            .len()
            .checked_sub(1)
            .and_then(|x| x.checked_add(address.into()))
            .and_then(|x| u16::try_from(x).ok())
            .ok_or(anyhow!("the segment is either empty or exceeds 0xffff"))?;

        self.file.write_all(&address.to_le_bytes())?;
        self.file.write_all(&end.to_le_bytes())?;
        self.file.write_all(contents)?;

        Ok(())
    }

    /// Call a init function.
    ///
    /// # Errors
    ///
    /// Write errors.
    pub fn init(&mut self, addr: u16) -> anyhow::Result<()> {
        self.write_segment(&(0x02e2, &addr.to_le_bytes()))
    }

    /// Call the run function.
    ///
    /// This returns the underlying file type, most useful to
    /// acquire the `Vec<u8>` from [`Self::new`].
    ///
    /// # Errors
    ///
    /// Write/flush errors.
    pub fn run(mut self, addr: u16) -> anyhow::Result<W> {
        self.write_segment(&(0x02e0, &addr.to_le_bytes()))?;

        self.file.flush()?;

        Ok(self.file)
    }
}
