use std::path::Path;
use std::{fs, io};

/// Include for several Atari 8-bit chips.
pub const ATARI_ASM: &str = include_str!("../../asm/atari.asm");

/// Copy all includes from [self] to the specified path.
///
/// Currently includes:
/// - [`ATARI_ASM`] as "atari.asm"
///
/// # Errors
///
/// Writing the files.
pub fn copy_bundled_files<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();

    fs::write(path.join("atari.asm"), ATARI_ASM.as_bytes())?;

    Ok(())
}
