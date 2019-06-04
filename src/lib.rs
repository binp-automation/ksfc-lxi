pub mod error;
pub mod deverr;

#[macro_use]
mod format;
mod constants;
pub mod types;
pub mod api;

pub use error::{Error, Result};
pub use deverr::{KsDevErr};


#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

use std::time::{Duration};

use ks_lxi::{KsDevice, KsData};


pub struct KsFc {
    lxi: KsDevice,
}

impl KsFc {
    pub fn new(host: &str, port: Option<u16>, timeout: Duration) -> (Self, crate::Result<()>) {
        let mut lxi = KsDevice::new((
            String::from(host),
            port.unwrap_or(5025),
        ), Some(timeout));
        let r = lxi.connect().map_err(|e| e.into());
        (Self { lxi }, r)
    }

    pub fn reconnect(&mut self) -> crate::Result<()> {
        self.lxi.reconnect().map_err(|e| e.into())
    } 

    fn send(&mut self, data: &[u8]) -> crate::Result<()> {
        self.lxi.send(data).map_err(|e| e.into())
    }
    fn receive(&mut self) -> crate::Result<KsData> {
        self.lxi.receive().map_err(|e| e.into())
    }
    #[allow(dead_code)]
    fn send_timeout(&mut self, data: &[u8], to: Duration) -> crate::Result<()> {
        self.lxi.send_timeout(data, Some(to)).map_err(|e| e.into())
    }
    fn receive_timeout(&mut self, to: Duration) -> crate::Result<KsData> {
        self.lxi.receive_timeout(Some(to)).map_err(|e| e.into())
    }
}
