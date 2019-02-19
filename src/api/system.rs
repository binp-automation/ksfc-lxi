use std::io;
use std::ops::{Deref, DerefMut};

use crate::parse::{ParseError};
use crate::api::{Handle as BaseHandle};
use crate::error::{Error};
use crate::constants::*;


pub struct Handle<'a> {
    base: &'a mut BaseHandle<'a>,
}

impl<'a> Handle<'a> {
    pub fn new(base: &'a mut BaseHandle<'a>) -> Self {
        Self { base }
    }
}

impl<'a> Deref for Handle<'a> {
    type Target = BaseHandle<'a>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> DerefMut for Handle<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<'a> Handle<'a> {
    pub fn error(&mut self) -> io::Result<Option<Error>> {
        self.dev.send(b"SYST:ERR?")
        .and_then(|()| self.dev.receive())
        .and_then(|buf| {
            buf.splitn(2, |c| *c == b',').next().ok_or(ParseError::EndOfString)
            .and_then(|part| {
                parse_bytes!(part, i32).map(|v| v.0).map_err(|e| e.into())
            }).map_err(|e| e.into())
        })
        .and_then(|code| {
            if code == NO_ERROR_CODE {
                Ok(None)
            } else {
                match Error::new(code) {
                    Some(e) => Ok(Some(e)),
                    None => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Unknown error code: '{}'", code),
                    )),
                }
            }
        })
    }
}

