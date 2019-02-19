pub mod error;

#[macro_use]
mod parse;
mod constants;
pub mod traits;
pub mod types;
pub mod api;


#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

use std::io;
use std::ops::{Deref, DerefMut};
use std::time::{Duration};

use lxi::LxiDevice;


pub struct Fc {
    device: LxiDevice,
}

impl Fc {
    pub fn new(host: &str, port: Option<u16>) -> io::Result<Self> {
        let mut device = LxiDevice::new((
            String::from(host),
            port.unwrap_or(5025),
        ), Some(Duration::from_secs(10)));
        device.connect()?;
        Ok(Fc { device })
    }

    pub fn api(&mut self) -> api::Handle {
        api::Handle::new(self)
    }
}

impl Deref for Fc {
    type Target = LxiDevice;
    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl DerefMut for Fc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.device
    }
}
