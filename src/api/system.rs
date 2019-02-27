use std::io;

use lxi::{LxiData};

use crate::{Fc};
use crate::parse::{ParseError};
use crate::handle::{Handle as HandleTrait, HandleParams};
use crate::api::{Handle as BaseHandle};
use crate::error::{Error};
use crate::constants::*;


pub struct Handle<'a, 'b> where 'a: 'b {
    base: &'b mut BaseHandle<'a>,
    par: HandleParams,
}

impl<'b, 'a: 'b> Handle<'a, 'b> {
    pub fn new(base: &'b mut BaseHandle<'a>, par: HandleParams) -> Self {
        Self { base, par }
    }
}

impl<'b, 'a: 'b> HandleTrait for Handle<'a, 'b> {
    type Device = Fc;

    fn device(&self) -> &Self::Device {
        self.base.device()
    }
    fn device_mut(&mut self) -> &mut Self::Device {
        self.base.device_mut()
    }

    fn params(&self) -> &HandleParams {
        &self.par
    }
    fn params_mut(&mut self) -> &mut HandleParams {
        &mut self.par
    }
}

impl<'b, 'a: 'b> Handle<'a, 'b> {
    pub fn error(&mut self) -> io::Result<Option<Error>> {
        self.send(b"SYST:ERR?")
        .and_then(|()| self.receive())
        .and_then(|data| {
            match data {
                LxiData::Text(buf) => Ok(buf),
                LxiData::Bin(_) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("The response is in binary format"),
                )),
            }
        })
        .and_then(|buf| {
            buf.splitn(2, |c| *c == b',').next().ok_or(ParseError::EndOfString)
            .and_then(|part| {
                parse_types!(part, i32).map(|v| v.0).map_err(|e| e.into())
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

