use anyhow::bail;
use regex_lite::Regex;
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};

/// A u16, when printed shown as hex
#[derive(Copy, Clone)]
struct U16Hex(pub u16);

impl Debug for U16Hex {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:04x}", self.0)
    }
}

/// Symbols of a compiled program
pub struct Symbols {
    /// The map of a symbol tree level.
    pub symbols: BTreeMap<String, (u16, Option<Symbols>)>,
}

impl Debug for Symbols {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Symbols");
        let mut symbols = self.symbols.iter().collect::<Vec<_>>();
        symbols.sort_by(|a, b| a.1.0.cmp(&b.1.0));
        for (k, v) in symbols {
            match v {
                (addr, None) => d.field(k, &U16Hex(*addr)),
                (addr, Some(sub)) => d.field(k, &(U16Hex(*addr), sub)),
            };
        }
        d.finish()
    }
}

impl Symbols {
    /// Get the address of a symbol.
    ///
    /// A dot `.` can be used to access symbols in lower levels.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<u16> {
        let mut current = (0u16, Some(self));
        for segment in name.split('.') {
            let (_, Some(symbols)) = current else {
                return None;
            };
            if let Some((a, n)) = symbols.symbols.get(segment) {
                current = (*a, n.as_ref());
            } else {
                return None;
            }
        }
        Some(current.0)
    }

    pub(crate) fn parse(input: &str) -> anyhow::Result<Symbols> {
        let mut lines = input.lines();
        let line_match = Regex::new("^\\.label ([0-9a-zA-Z_]+)=\\$([0-9a-fA-F]{1,4}) *([{]?)$")
            .expect("Regex parse failed");
        let mut line_number = 1;
        Self::scan(&line_match, &mut line_number, &mut lines, true)
    }

    fn scan<'a, I: Iterator<Item = &'a str>>(
        line_match: &Regex,
        line_number: &mut usize,
        lines: &mut I,
        top: bool,
    ) -> anyhow::Result<Symbols> {
        let mut symbols = Symbols {
            symbols: BTreeMap::new(),
        };

        while let Some(line) = lines.next() {
            *line_number += 1;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line == "}" {
                if top {
                    bail!("unexpected closing brace on line {line_number}");
                }
                return Ok(symbols);
            } else if let Some(captures) = line_match.captures(line) {
                let subtree = if &captures[3] == "{" {
                    Some(Self::scan(line_match, line_number, lines, false)?)
                } else {
                    None
                };
                symbols.symbols.insert(
                    captures[1].to_string(),
                    (u16::from_str_radix(&captures[2], 16).unwrap(), subtree),
                );
            } else {
                bail!("can't parse symbols on line {line_number}");
            }
        }

        if !top {
            bail!("missing closing brace at end of file");
        }

        Ok(symbols)
    }
}
