use crate::parse::*;
use std::collections::HashMap;

pub struct Dotenv {
    buf: String,
}

impl Dotenv {
    pub(crate) fn new(buf: String) -> Self {
        Self { buf }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(&self.buf)
    }

    pub fn load(self) {
        self.set_vars(false)
    }

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

    fn resolve_var(&self, name: &'a str) -> Option<String> {
        std::env::var(name)
            .ok()
            .or_else(|| self.resolved.get(name).cloned())
    }

    fn resolve(&self, value: Value<'a>) -> Option<String> {
        match value {
            Value::Lit(text) => Some(text.to_string()),
            Value::Var(name, default) => self
                .resolve_var(name)
                .or_else(|| default.and_then(|it| self.resolve(*it))),
            Value::List(list) => Some(list.into_iter().flat_map(|it| self.resolve(it)).collect()),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, String);

    fn next(&mut self) -> Option<Self::Item> {
        while let Ok((rest, maybe)) = parse(self.input) {
            self.input = rest; // set next input

            if let Some((key, value)) = maybe {
                if let Some(value) = self.resolve(value) {
                    self.resolved.insert(key, value.clone());
                    return Some((key, value));
                }
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
