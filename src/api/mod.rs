pub mod system;


use std::io;

use lxi::{LxiData};

use crate::parse::{ParseError};
use crate::handle::{Handle as HandleTrait, HandleParams};
use crate::types::*;
use crate::constants::*;
use crate::{Fc};
use crate::error::{Error};


pub struct Handle<'a> {
    dev: &'a mut Fc,
    par: HandleParams,
}

impl<'a> Handle<'a> {
    pub fn new(dev: &'a mut Fc) -> Self {
        Handle { dev, par: HandleParams::new() }
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

    fn params(&self) -> &HandleParams {
        &self.par
    }
    fn params_mut(&mut self) -> &mut HandleParams {
        &mut self.par
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

    fn read_value(&mut self) -> io::Result<Result<f64, Error>> {
        self.receive()
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
            parse_types!(&buf, f64).map(|v| v.0).map_err(|e| e.into())
            .map(|v| {
                if v >= ERROR_VALUE {
                    None
                } else {
                    Some(v)
                }
            })
        })
        .and_then(|v| {
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

    pub fn fetch(&mut self) -> io::Result<Result<f64, Error>> {
        self.send(b"FETC?").and_then(|()| self.read_value())
    }

    pub fn initiate(&mut self) -> io::Result<Result<(), Error>> {
        self.send(b"INIT")
        .and_then(|()| self.system().error())
        .map(|e| match e {
            Some(e) => Err(e),
            None => Ok(()),
        })
    }

    pub fn read(&mut self) -> io::Result<Result<f64, Error>> {
        self.send(b"READ?").and_then(|()| self.read_value())
    }

    pub fn r(&mut self, max_count: Option<usize>) -> io::Result<Vec<u8>> {
        match max_count {
            Some(n) => self.send(format!("R? {}", n).as_bytes()),
            None => self.send(b"R?"),
        }
        .and_then(|()| self.receive())
        .and_then(|data| {
            match data {
                LxiData::Text(_) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("The response is in text format"),
                )),
                LxiData::Bin(buf) => Ok(buf),
            }
        })
    }

    // IEEE-488 Common commands

    pub fn cal(&mut self) -> io::Result<bool> {
        self.send(b"*CAL?")
        .and_then(|()| {
            if self.timeout() >= CAL_TIMEOUT {
                self.receive()
            } else {
                self.receive_timeout(CAL_TIMEOUT)
            }
        })
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
            parse_types!(&buf, i32).map(|t| t.0 == 0)
            .map_err(|e| e.into())
        })
    }

    pub fn cls(&mut self) -> io::Result<()> {
        self.send(b"*CLS")
    }

    pub fn ese<'b>(&'b mut self) -> EseHandle<'a, 'b> where 'a: 'b {
        EseHandle::new(self, self.par.clone())
    }

    pub fn rst(&mut self) -> io::Result<()> {
        self.send(b"*RST")
    }

    // Subsystems

    pub fn system<'b>(&'b mut self) -> system::Handle<'a, 'b> where 'a: 'b {
        system::Handle::new(self, self.par.clone())
    }
}

pub struct EseHandle<'a, 'b> where 'a: 'b {
    base: &'b mut Handle<'a>,
    par: HandleParams,
}

impl<'b, 'a: 'b> EseHandle<'a, 'b> {
    pub fn new(base: &'b mut Handle<'a>, par: HandleParams) -> Self {
        Self { base, par }
    }
}

impl<'b, 'a: 'b> HandleTrait for EseHandle<'a, 'b> {
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


impl<'b, 'a: 'b> EseHandle<'a, 'b> {
    pub fn get(&mut self) -> io::Result<EventReg> {
        self.send(b"*ESE?")
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
        .and_then(|buf| parse_types!(&buf, u8).map_err(|e| e.into()))
        .map(|b| EventReg::from_bits_truncate(b.0))
    }

    pub fn set(&mut self, ereg: EventReg) -> io::Result<()> {
        self.send(format!("*ESE {}", ereg.bits()).as_bytes())
    }
}
