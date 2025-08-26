use anyhow::Context;
use regex_lite::Regex;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

/// Format assembler files.
///
/// Only files with the extension "asm" will be formatted.
///
/// # Errors
///
/// Reading/writing the files, traversing the directories.
pub fn format<O: AsRef<OsStr>>(files: &[O], recursive: bool, verbose: bool) -> anyhow::Result<()> {
    let runner = Runner {
        debug: verbose,
        multiple_whitespaces: Regex::new(r"\s+")?,
        single_whitespace: Regex::new(r"\s")?,
    };

    for file in files {
        let file = file.as_ref();
        let metadata = fs::metadata(file)
            .with_context(|| format!("Failed to read metadata for {}", file.display()))?;
        if metadata.is_file() {
            process_file(&runner, file)?;
        } else if metadata.is_dir() && recursive {
            process_dir(&runner, file)?;
        }
    }

    Ok(())
}

struct Runner {
    debug: bool,
    multiple_whitespaces: Regex,
    single_whitespace: Regex,
}

fn process_dir<P: AsRef<Path>>(runner: &Runner, path: P) -> anyhow::Result<()> {
    if runner.debug {
        println!("Processing directory {}", path.as_ref().display());
    }

    for e in fs::read_dir(path)? {
        let e = e?;
        let metadata = e.metadata()?;
        if metadata.is_dir() {
            process_dir(runner, e.path())?;
        } else if metadata.is_file() {
            process_file(runner, e.path())?;
        }
    }

    Ok(())
}

fn process_file<P: AsRef<Path>>(runner: &Runner, path: P) -> anyhow::Result<()> {
    let path = path.as_ref();

    match path.extension() {
        Some(ext) if ext == "asm" => (),
        _ => {
            if runner.debug {
                println!("Skipping {}", path.display());
            }
            return Ok(());
        }
    }

    if runner.debug {
        println!("Processing {}", path.display());
    }

    let original_contents =
        fs::read_to_string(path).with_context(|| format!("reading file {}", path.display()))?;
    let mut lines = original_contents
        .strip_prefix("\u{FEFF}")
        .unwrap_or(&original_contents)
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>();

    while let Some(line) = lines.last() {
        if line.is_empty() {
            lines.pop();
        } else {
            break;
        }
    }

    let mut content = String::with_capacity(original_contents.len() + original_contents.len() / 4);

    let mut block_comment = false;
    let mut indent: isize = 0;
    for line in lines {
        let ps = parse(line, &mut block_comment);

        let br = braces(&ps);
        if br < 0 {
            indent = (indent + br).max(0);
        }
        for _ in 0..indent {
            content.push_str("    ");
        }
        if br > 0 {
            indent += br;
        }

        let has_colon = ps.iter().any(|s| match s {
            Slice::Text(t) => t.contains(':'),
            Slice::Line | Slice::BlockOpen | Slice::BlockClose | Slice::Comment(_) => false,
        });
        if !has_colon {
            content.push_str("    ");
        }

        for s in ps {
            match s {
                Slice::Text(t) => {
                    let ts = t.split('"').collect::<Vec<_>>();
                    for (i, t) in ts.iter().enumerate() {
                        if i & 1 == 0 {
                            content.push_str(&runner.multiple_whitespaces.replace_all(t, " "));
                        } else {
                            content.push_str(t);
                        }
                        if i < ts.len() - 1 {
                            content.push('"');
                        }
                    }
                    content.push(' ');
                }
                Slice::Line => {
                    content.push_str("// ");
                }
                Slice::BlockOpen => {
                    content.push_str("/* ");
                }
                Slice::BlockClose => {
                    content.push_str("*/ ");
                }
                Slice::Comment(c) => {
                    content.push_str(&runner.single_whitespace.replace_all(c, " "));
                    content.push(' ');
                }
            }
        }
        while content.ends_with(' ') {
            content.pop();
        }
        content.push('\n');
    }

    if original_contents != content {
        if runner.debug {
            println!("Has changed {}", path.display());
        }
        fs::write(path, content).with_context(|| format!("writing file {}", path.display()))?;
    }

    Ok(())
}

fn braces(p0: &Vec<Slice>) -> isize {
    p0.iter()
        .map(|s| {
            if let Slice::Text(t) = s {
                t.bytes().fold(0_isize, |b, c| match c {
                    b'{' => b + 1,
                    b'}' => b - 1,
                    _ => b,
                })
            } else {
                0
            }
        })
        .sum()
}

#[derive(Debug)]
enum Slice<'a> {
    Text(&'a str),
    Line,
    BlockOpen,
    BlockClose,
    Comment(&'a str),
}

fn parse<'a>(mut input: &'a str, block_comment: &mut bool) -> Vec<Slice<'a>> {
    let mut result = Vec::new();

    while !input.is_empty() {
        if *block_comment {
            if let Some((a, b)) = input.split_once("*/") {
                let a = a.trim_ascii();
                if !a.is_empty() {
                    result.push(Slice::Comment(a));
                }
                result.push(Slice::BlockClose);
                input = b;
                *block_comment = false;
            } else {
                result.push(Slice::Comment(input.trim_ascii()));
                input = "";
            }
        } else {
            let c_l = input.split_once("//");
            let c_b = input.split_once("/*");
            if c_l.is_some() && (c_b.is_none() || c_l.unwrap().0.len() < c_b.unwrap().0.len()) {
                // only line comment (or it starts before the block comment)
                #[allow(clippy::unnecessary_unwrap)]
                let (a, b) = c_l.unwrap();
                let a = a.trim_ascii();
                if !a.is_empty() {
                    result.push(Slice::Text(a));
                }
                result.push(Slice::Line);
                let b = b.trim_ascii();
                if !b.is_empty() {
                    result.push(Slice::Comment(b));
                }
                input = "";
            } else if let Some((a, b)) = c_b {
                // only block comment (or it starts before the line comment)
                let a = a.trim_ascii();
                if !a.is_empty() {
                    result.push(Slice::Text(a));
                }
                result.push(Slice::BlockOpen);
                input = b;
                *block_comment = true;
            } else {
                result.push(Slice::Text(input.trim_ascii()));
                input = "";
            }
        }
    }

    result
}
