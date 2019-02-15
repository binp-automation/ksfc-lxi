pub mod error;

#[macro_use]
mod parse;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

use std::io;
use std::ops::{Deref, DerefMut};
use std::time::{Duration};

use lxi::LxiDevice;

use crate::parse::{ParseError};


bitflags! {
    pub struct EventReg: u8 {
        const OpComplete = 0b00000001;
        const QueryErr   = 0b00000100;
        const DevSpecErr = 0b00001000;
        const ExecErr    = 0b00010000;
        const CmdErr     = 0b00100000;
        const PowerOn    = 0b10000000;
    }
}

pub struct KsFc {
    device: LxiDevice,
}

impl KsFc {
    pub fn new(host: &str, port: Option<u16>) -> io::Result<Self> {
        let mut device = LxiDevice::new((
            String::from(host),
            port.unwrap_or(5025),
        ), Some(Duration::from_millis(2000)));
        device.connect()?;
        Ok(KsFc { device })
    }

    pub fn api(&mut self) -> ApiCalls {
        ApiCalls { orig: self }
    }
}

impl Deref for KsFc {
    type Target = LxiDevice;
    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl DerefMut for KsFc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.device
    }
}

pub struct ApiCalls<'a> {
    orig: &'a mut KsFc,
}

impl<'a> ApiCalls<'a> {
    pub fn abort(&mut self) -> io::Result<()> {
        self.orig.send(b"ABOR")
    }

    pub fn autoscale(&mut self) -> io::Result<()> {
        self.orig.send(b"AUT")
    }

    pub fn fetch(&mut self) -> io::Result<Vec<u8>> {
        self.orig.send(b"FETC?").and_then(|()| self.orig.receive())
    }

    pub fn cal(&mut self) -> io::Result<bool> {
        self.orig.send(b"*CAL?").and_then(|()| {
            self.orig.receive_timeout(Some(Duration::from_secs(20)))
        }).and_then(|buf| {
            parse_bytes!(buf, (i32)).map(|t| t.0 == 0)
            .map_err(|e| e.into())
        })
    }

    pub fn cls(&mut self) -> io::Result<()> {
        self.orig.send(b"*CLS")
    }

    pub fn ese(&mut self) -> EseCalls {
        EseCalls { orig: self.orig }
    }
}

pub struct EseCalls<'a> {
    orig: &'a mut KsFc,
}

impl<'a> EseCalls<'a> {
    pub fn get(&mut self) -> io::Result<EventReg> {
        self.orig.send(b"*ESE?")
        .and_then(|()| self.orig.receive())
        .and_then(|buf| parse_bytes!(buf, (u8)).map_err(|e| e.into()))
        .map(|b| EventReg::from_bits_truncate(b.0))
    }

    pub fn set(&mut self, ereg: EventReg) -> io::Result<()> {
        self.orig.send(format!("*ESE {}", ereg.bits).as_bytes())
    }
}

pub struct SystemCalls<'a> {
    orig: &'a mut KsFc,
}

impl<'a> SystemCalls<'a> {
    pub fn error(&mut self) -> io::Result<EventReg> {
        self.orig.send(b"*ESE?")
        .and_then(|()| self.orig.receive())
        .and_then(|buf| parse_bytes!(buf, (u8)).map_err(|e| e.into()))
        .map(|b| EventReg::from_bits_truncate(b.0))
    }
}

