use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::io::BufReader;

use crate::constant;
use crate::error::*;
use crate::parse;

pub struct Iter<R> {
    lines: QuotedLines<BufReader<R>>,
    substitution_data: HashMap<String, Option<String>>,
}

impl<R: Read> Iter<R> {
    pub(crate) fn new(reader: R) -> Iter<R> {
        Iter {
            lines: QuotedLines {
                buf: BufReader::new(reader),
            },
            substitution_data: HashMap::new(),
        }
    }

    /// Loads all variables found in the `reader` into the environment.
    pub fn load(self) -> Result<()> {
        for item in self {
            let (key, value) = item?;
            if env::var(&key).is_err() {
                env::set_var(&key, value);
            }
        }

        Ok(())
    }
}

struct QuotedLines<B> {
    buf: B,
}

#[derive(Clone, Copy)]
enum QuoteState {
    Escape,
    StrongOpen,
    StrongOpenEscape,
    WeakOpen,
    WeakOpenEscape,
    Close,
}

impl QuoteState {
    fn next_state(self, input: &str) -> Self {
        input.chars().fold(self, |curr, c| match curr {
            QuoteState::Escape => QuoteState::Close,
            QuoteState::Close => match c {
                constant::SLASH => QuoteState::Escape,
                constant::DOUBLE_QUOTE => QuoteState::WeakOpen,
                constant::SINGLE_QUOTE => QuoteState::StrongOpen,
                _ => QuoteState::Close,
            },
            QuoteState::WeakOpen => match c {
                constant::SLASH => QuoteState::WeakOpenEscape,
                constant::DOUBLE_QUOTE => QuoteState::Close,
                _ => QuoteState::WeakOpen,
            },
            QuoteState::WeakOpenEscape => QuoteState::WeakOpen,
            QuoteState::StrongOpen => match c {
                constant::SLASH => QuoteState::StrongOpenEscape,
                constant::SINGLE_QUOTE => QuoteState::Close,
                _ => QuoteState::StrongOpen,
            },
            QuoteState::StrongOpenEscape => QuoteState::StrongOpen,
        })
    }
}

impl<B: BufRead> Iterator for QuotedLines<B> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Result<String>> {
        let mut buf = String::new();
        let mut cur_state = QuoteState::Close;
        let mut buf_pos;

        loop {
            buf_pos = buf.len();
            match self.buf.read_line(&mut buf) {
                Ok(0) => match cur_state {
                    QuoteState::Close => return None,
                    _ => {
                        let len = buf.len();
                        return Some(Err(Error::LineParse(buf, len)));
                    }
                },
                Ok(_) => {
                    cur_state = cur_state.next_state(&buf[buf_pos..]);
                    if let QuoteState::Close = cur_state {
                        if buf.ends_with('\n') {
                            buf.pop();
                            if buf.ends_with('\r') {
                                buf.pop();
                            }
                        }
                        return Some(Ok(buf));
                    }
                }
                Err(e) => return Some(Err(Error::Io(e))),
            }
        }
    }
}

impl<R: Read> Iterator for Iter<R> {
    type Item = Result<(String, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = match self.lines.next() {
                Some(Ok(line)) => line,
                Some(Err(err)) => return Some(Err(err)),
                None => return None,
            };

            match parse::parse_line(&line, &mut self.substitution_data) {
                Ok(Some(result)) => return Some(Ok(result)),
                Ok(None) => {}
                Err(err) => return Some(Err(err)),
            }
        }
    }
}
