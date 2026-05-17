use crate::parse::*;
use std::collections::HashMap;

/// Parsed dotenv content
pub struct Dotenv {
    buf: String,
}

fn normalize_input(buf: String) -> String {
    // Strip UTF-8 BOM (\u{FEFF}) if present — the parser does not expect it
    let buf = if let Some(stripped) = buf.strip_prefix('\u{FEFF}') {
        stripped.to_string()
    } else {
        buf
    };

    // Normalise line endings per spec: \r\n → \n, then standalone \r → \n
    buf.replace("\r\n", "\n").replace('\r', "\n")
}

impl Dotenv {
    pub(crate) fn new(buf: String) -> Self {
        Self {
            buf: normalize_input(buf),
        }
    }

    /// Return an iterator over the dotenv entries.
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(&self.buf)
    }

    /// Load the dotenv into the current process's environment variables.
    ///
    /// Existing variables are **not** overwritten.
    pub fn load(self) {
        self.set_vars(false)
    }

    /// Load the dotenv into the current process's environment variables,
    /// **overwriting** any existing values.
    pub fn load_override(self) {
        self.set_vars(true)
    }

    fn set_vars(self, override_env: bool) {
        for (key, value) in self.iter() {
            if override_env || std::env::var(key).is_err() {
                std::env::set_var(key, value);
            }
        }
    }
}

/// An iterator over the entries in a dotenv file.
///
/// Yields `(&str, String)` pairs. Variable substitutions are resolved
/// lazily against both the process environment and previously-yielded
/// entries within the same file.
pub struct Iter<'a> {
    resolved: HashMap<&'a str, String>,
    input: &'a str,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            resolved: HashMap::new(),
            input,
        }
    }

    /// Resolve a variable name to a non-empty value.
    ///
    /// Checks the process environment first, then previously-resolved
    /// entries from this file.
    fn resolve_var(&self, name: &'a str) -> Option<String> {
        std::env::var(name)
            .ok()
            .or_else(|| self.resolved.get(name).cloned())
            .filter(|it| !it.is_empty())
    }

    fn resolve(&self, value: Value<'a>) -> String {
        match value {
            Value::Lit(text) => text.to_string(),
            Value::Sub(name, fallback) => self
                .resolve_var(name)
                .or_else(|| fallback.map(|it| self.resolve(*it)))
                .unwrap_or_default(),
            Value::List(list) => list.into_iter().map(|it| self.resolve(it)).collect(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, String);

    fn next(&mut self) -> Option<Self::Item> {
        while !self.input.is_empty() {
            match parse(&mut self.input) {
                Ok(Some((key, value))) => {
                    let resolved = self.resolve(value);
                    self.resolved.insert(key, resolved.clone());
                    return Some((key, resolved));
                }
                Ok(None) => {
                    // comment or blank line, continue
                }
                Err(_) => {
                    // parse error
                    break;
                }
            }
        }

        None
    }
}
