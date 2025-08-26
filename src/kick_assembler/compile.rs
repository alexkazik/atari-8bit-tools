use crate::kick_assembler::Symbols;
use crate::kick_assembler::config::Config;
use crate::kick_assembler::output::Output;
use crate::kick_assembler::prg::read_prg;
use anyhow::{Context, bail};
use regex_lite::Regex;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::{fs, io};

/// Compile with KickAssembler.
///
/// # Errors
///
/// Due to launching KickAssembler or reading the result.
#[allow(clippy::doc_markdown)]
#[allow(clippy::too_many_lines)]
pub fn compile<P: AsRef<Path>>(config: &Config, main_asm: P) -> anyhow::Result<Output> {
    let main_asm = main_asm.as_ref();
    // call the assembler
    let mut command = Command::new(config.java);
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .arg("-jar")
        .arg(config.kick_assembler)
        .arg(main_asm)
        .arg("-showmem")
        .arg("-symbolfile")
        .arg("-bytedump");
    let source_directory_components = config
        .source_directory
        .components()
        .filter(|c| c.as_os_str() != ".")
        .count();
    if source_directory_components > 0 {
        command.current_dir(config.source_directory);
    }
    if config
        .output_directory
        .components()
        .filter(|c| c.as_os_str() != ".")
        .count()
        > 0
    {
        let mut back = PathBuf::new();
        for _ in 0..source_directory_components {
            back.push("..");
        }
        command.arg("-odir").arg(back.join(config.output_directory));
    }

    let output = command.output().context("Failed to launch KickAssembler")?;

    if !output.status.success() {
        let _ = io::stdout().write_all(&output.stdout);
        bail!("KickAssembler failed with status {}", output.status);
    }

    #[allow(clippy::missing_panics_doc)]
    let match_line = Regex::new(r"^ {2}(\*?)\$([0-9a-fA-F]{4})-\$([0-9a-fA-F]{4}) (.*)$").unwrap();
    #[allow(clippy::missing_panics_doc)]
    let match_page = Regex::new(r"\(page (((\$|0x)([0-9a-fA-F]+))|(()([0-9]+)))\)$").unwrap();
    let mut last_pending = None;
    let mut free = 0;
    let mut ok = false;

    for line in output.stdout.split(|b| *b == b'\n') {
        let line = line.strip_suffix(b"\r").unwrap_or(line);

        if let Ok(line) = from_utf8(line) {
            if let Some((_, [virt, lo_hex, hi_hex, name])) =
                match_line.captures(line).map(|captures| captures.extract())
            {
                #[allow(clippy::missing_panics_doc)]
                let lo = u16::from_str_radix(lo_hex, 16).unwrap();
                #[allow(clippy::missing_panics_doc)]
                let hi = u16::from_str_radix(hi_hex, 16).unwrap();
                let virt = if virt == "*" { '*' } else { ' ' };

                if let Some((last, pending)) = last_pending.take() {
                    free += lo - last;
                    println!("  {pending}-{:04x} {:5} FREE", lo - 1, lo - last);
                }

                if name == "FREE" {
                    last_pending = Some((lo, format!("{virt}{lo_hex}")));
                } else {
                    if name == "EOF"
                        && let Some((last, _pending)) = last_pending.take()
                    {
                        free += hi.wrapping_add(1).wrapping_sub(last);
                    }
                    println!(
                        "  {virt}{}-{} {:5} {}",
                        lo_hex,
                        hi_hex,
                        hi.wrapping_add(1).wrapping_sub(lo),
                        name
                    );
                }

                if let Some((_, [_, _, is_hex, num])) =
                    match_page.captures(line).map(|captures| captures.extract())
                {
                    let page = if is_hex.is_empty() {
                        num.parse()
                    } else {
                        u16::from_str_radix(num, 16)
                    }?;
                    let page_mask = !(page - 1);
                    if page & page_mask != page {
                        bail!("Invalid page definition at {line}");
                    }
                    if (lo & page_mask) != (hi & page_mask) {
                        bail!("Invalid page usage at {line}");
                    }
                }
            } else {
                if line.starts_with("Writing Symbol") {
                    ok = true;
                }
                println!("{line}");
            }
        } else {
            let _ = io::stdout().write_all(line);
            println!();
        }
    }

    if ok {
        println!("Total Free memory {free}");

        let mut path_main_prg = config.output_directory.join(main_asm);
        let mut path_symbols = path_main_prg.clone();

        path_symbols.set_extension("sym");
        path_main_prg.set_extension("prg");

        Ok(Output {
            prg: read_prg(path_main_prg).context("Failed to read prg file")?,
            symbols: Symbols::parse(
                &fs::read_to_string(path_symbols).context("Failed to read sym file")?,
            )
            .context("Failed to parse symbols")?,
        })
    } else {
        bail!("The output couldn't be identified");
    }
}
