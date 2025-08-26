use std::path::Path;

/// Configuration.
pub struct Config<'a> {
    /// Path to java, usually just "java".
    pub java: &'a Path,
    /// Path to the KickAssembler.
    #[allow(clippy::doc_markdown)]
    pub kick_assembler: &'a Path,
    /// Path to the assembly source directory.
    pub source_directory: &'a Path,
    /// Path to the output directory.
    pub output_directory: &'a Path,
}
