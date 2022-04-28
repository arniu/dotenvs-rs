mod constant;
mod expand;
mod parse;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::error::*;

pub struct Dotenv<R = File> {
    context: HashMap<String, Option<String>>,
    lines: Quoted<BufReader<R>>,
}

impl<R: Read> Dotenv<R> {
    pub(crate) fn new(reader: R) -> Dotenv<R> {
        Dotenv {
            context: HashMap::new(),
            lines: Quoted {
                buf: BufReader::new(reader),
            },
        }
    }

    /// Loads all variables into the environment.
    pub fn load(self) -> Result<()> {
        for pair in self {
            let (key, value) = pair?;
            if env::var(&key).is_err() {
                env::set_var(&key, value);
            }
        }

        Ok(())
    }
}

impl<R: Read> Iterator for Dotenv<R> {
    type Item = Result<(String, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = match self.lines.next() {
                Some(Ok(line)) => line,
                Some(Err(err)) => return Some(Err(err)),
                None => return None,
            };

            match parse::parse_line(&line, &mut self.context) {
                Ok(Some(result)) => return Some(Ok(result)),
                Ok(None) => {}
                Err(err) => return Some(Err(err)),
            }
        }
    }
}

struct Quoted<B> {
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
                constant::BACKSLASH => QuoteState::Escape,
                constant::DOUBLE_QUOTE => QuoteState::WeakOpen,
                constant::SINGLE_QUOTE => QuoteState::StrongOpen,
                _ => QuoteState::Close,
            },
            QuoteState::WeakOpen => match c {
                constant::BACKSLASH => QuoteState::WeakOpenEscape,
                constant::DOUBLE_QUOTE => QuoteState::Close,
                _ => QuoteState::WeakOpen,
            },
            QuoteState::WeakOpenEscape => QuoteState::WeakOpen,
            QuoteState::StrongOpen => match c {
                constant::BACKSLASH => QuoteState::StrongOpenEscape,
                constant::SINGLE_QUOTE => QuoteState::Close,
                _ => QuoteState::StrongOpen,
            },
            QuoteState::StrongOpenEscape => QuoteState::StrongOpen,
        })
    }
}

impl<B: BufRead> Iterator for Quoted<B> {
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
