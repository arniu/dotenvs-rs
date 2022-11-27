use crate::parse::*;
use std::collections::HashMap;

/// Dotenv content
pub struct Dotenv {
    buf: String,
}

impl Dotenv {
    pub(crate) fn new(buf: String) -> Self {
        Self { buf }
    }

    /// Return an iterator over the dotenv.
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(&self.buf)
    }

    /// Load the dotenv into the current process's environment variables
    ///
    /// **NOTE**: The existing variables will be ignored.
    pub fn load(self) {
        self.set_vars(false)
    }

    /// Load the dotenv into the current process's environment variables
    ///
    /// **NOTE**: This will override the existing variables.
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

/// Dotenv iterator
pub struct Iter<'a> {
    resolved: HashMap<&'a str, String>,
    input: &'a str,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            resolved: HashMap::new(),
            input: strip_bom(input),
        }
    }

    /// resolve **NON-EMPTY** variable
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
        while let Ok((rest, maybe)) = parse(self.input) {
            self.input = rest;

            if let Some((key, value)) = maybe {
                let resolved = self.resolve(value);
                self.resolved.insert(key, resolved.clone());
                return Some((key, resolved));
            }

            if rest.is_empty() {
                break;
            }
        }

        None
    }
}

fn strip_bom(input: &str) -> &str {
    // https://www.unicode.org/faq/utf_bom.html
    input.strip_prefix('\u{FEFF}').unwrap_or(input)
}
