pub mod system;


use std::io;
use std::ops::{Deref, DerefMut};
use std::time::{Duration};

use crate::parse::{ParseError};
use crate::traits::{Handle as HandleTrait};
use crate::types::*;
use crate::constants::*;
use crate::{Fc};
use crate::error::{Error};


pub struct Handle<'a> {
    dev: &'a mut Fc,
    to: Option<Option<Duration>>,
}

impl<'a> Handle<'a> {
    pub(crate) fn new(dev: &'a mut Fc) -> Self {
        Handle { dev, to: None }
    }
}

impl<'a> HandleTrait for Handle<'a> {
    type Device = Fc;

    fn device(&self) -> &Self::Device {
        self.dev
    }
    fn device_mut(&mut self) -> &mut Self::Device {
        self.dev
    }

    fn send(&mut self, data: &[u8]) -> io::Result<()> {
        match self.to {
            Some(to) => self.dev.send_timeout(data, to),
            None => self.dev.send(data),
        }
    }
    fn receive(&mut self) -> io::Result<Vec<u8>> {
        match self.to {
            Some(to) => self.dev.receive_timeout(to),
            None => self.dev.receive(),
        }
    }
    fn send_timeout(&mut self, data: &[u8], to: Option<Duration>) -> io::Result<()> {
        self.dev.send_timeout(data, to)
    }
    fn receive_timeout(&mut self, to: Option<Duration>) -> io::Result<Vec<u8>> {
        self.dev.receive_timeout(to)
    }

    fn with_timeout(mut self, to: Option<Duration>) -> Self {
        self.set_timeout(to);
        self
    }
    fn set_timeout(&mut self, to: Option<Duration>) {
        self.to = Some(to);
    }
    fn reset_timeout(&mut self) {
        self.to = None;
    }
    fn timeout(&self) -> Option<Option<Duration>> {
        self.to
    }
}

impl<'a> Handle<'a> {
    // Basic commands

    pub fn abort(&mut self) -> io::Result<()> {
        self.send(b"ABOR")
    }

    pub fn autoscale(&mut self) -> io::Result<()> {
        self.send(b"AUT")
    }

    pub fn fetch<'b: 'a>(&'b mut self) -> io::Result<Result<f64, Error>> {
        self.send(b"FETC?")
        .and_then(|()| self.receive())
        .and_then(|buf| {
            parse_bytes!(&buf, f64).map(|v| v.0).map_err(|e| e.into())
            .map(|v| {
                if v >= ERROR_VALUE {
                    None
                } else {
                    Some(v)
                }
            })
        })
        .or_else(|e: io::Error| -> io::Result<Option<f64>> {
            println!("{:?}", e);
            match e.kind() {
                io::ErrorKind::WouldBlock |
                io::ErrorKind::TimedOut => {
                    Ok(None)
                },
                _ => Err(e)
            }
        })
        .and_then(move |v| {
            match v {
                Some(v) => Ok(Ok(v)),
                None => {
                    self.system().error().and_then(|e| match e {
                        Some(e) => Ok(Err(e)),
                        None => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Request failed but there is no errors"),
                        )),
                    })
                },
            }
        })
    }

    pub fn initiate(&mut self) -> io::Result<()> {
        self.send(b"INIT")
    }

    // IEEE-488 Common commands

    pub fn cal(&mut self) -> io::Result<bool> {
        self.send(b"*CAL?").and_then(|()| {
            self.receive_timeout(Some(CAL_TIMEOUT))
        }).and_then(|buf| {
            parse_bytes!(&buf, i32).map(|t| t.0 == 0)
            .map_err(|e| e.into())
        })
    }

    pub fn cls(&mut self) -> io::Result<()> {
        self.send(b"*CLS")
    }

    pub fn ese<'b: 'a>(&'b mut self) -> EseHandle<'b> {
        EseHandle::<'b>::new(self)
    }

    // Subsystems

    pub fn system<'b: 'a>(&'b mut self) -> system::Handle<'b> {
        system::Handle::<'b>::new(self)
    }
}

pub struct EseHandle<'a> {
    base: &'a mut Handle<'a>,
}

impl<'a> EseHandle<'a> {
    pub fn new(base: &'a mut Handle<'a>) -> Self {
        Self { base }
    }
}

impl<'a> Deref for EseHandle<'a> {
    type Target = Handle<'a>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> DerefMut for EseHandle<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<'a> EseHandle<'a> {
    pub fn get(&mut self) -> io::Result<EventReg> {
        self.send(b"*ESE?")
        .and_then(|()| self.dev.receive())
        .and_then(|buf| parse_bytes!(&buf, u8).map_err(|e| e.into()))
        .map(|b| EventReg::from_bits_truncate(b.0))
    }

    pub fn set(&mut self, ereg: EventReg) -> io::Result<()> {
        self.send(format!("*ESE {}", ereg.bits()).as_bytes())
    }
}
