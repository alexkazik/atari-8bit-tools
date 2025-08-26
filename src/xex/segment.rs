use crate::kick_assembler::Output;
use crate::kick_assembler::prg::Prg;

/// A segment of data to be added to the xex file.
pub trait Segment {
    /// Get the address of this segment.
    fn address(&self) -> u16;

    /// Get the contents of this segment.
    fn contents(&self) -> &[u8];
}

impl Segment for Prg {
    #[inline]
    fn address(&self) -> u16 {
        self.address
    }

    #[inline]
    fn contents(&self) -> &[u8] {
        &self.contents
    }
}

impl Segment for (u16, &[u8]) {
    #[inline]
    fn address(&self) -> u16 {
        self.0
    }

    #[inline]
    fn contents(&self) -> &[u8] {
        self.1
    }
}

impl<const N: usize> Segment for (u16, &[u8; N]) {
    #[inline]
    fn address(&self) -> u16 {
        self.0
    }

    #[inline]
    fn contents(&self) -> &[u8] {
        self.1.as_ref()
    }
}

impl Segment for Output {
    #[inline]
    fn address(&self) -> u16 {
        // forward to Prg
        self.prg.address()
    }

    #[inline]
    fn contents(&self) -> &[u8] {
        // forward to Prg
        self.prg.contents()
    }
}
