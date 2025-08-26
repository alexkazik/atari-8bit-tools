use crate::kick_assembler::{Prg, Symbols};

/// Result of running the KickAssembler.
pub struct Output {
    /// The compiled program.
    pub prg: Prg,
    /// The symbols.
    pub symbols: Symbols,
}
