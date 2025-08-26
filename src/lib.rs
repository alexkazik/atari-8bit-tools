//! Atari-8bit tool framework
//!
//! See <https://github.com/alexkazik/atari-8bit-soft> for examples.

/// Tools for assembler file.
pub mod assembler;

/// Include files.
pub mod bundeled_files;

/// Cli framework.
#[cfg(feature = "cli")]
pub mod cli;

/// Tools for KickAssembler.
#[allow(clippy::doc_markdown)]
pub mod kick_assembler;

/// Tools for xex-files.
pub mod xex;
