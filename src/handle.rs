use std::io;
use std::ops::{DerefMut};
use std::time::{Duration};

use lxi::{LxiDevice, LxiData};


pub trait Handle: Sized {
    type Device: DerefMut<Target=LxiDevice>;

    fn device(&self) -> &Self::Device;
    fn device_mut(&mut self) -> &mut Self::Device;

    fn params(&self) -> &HandleParams;
    fn params_mut(&mut self) -> &mut HandleParams;

    fn send(&mut self, data: &[u8]) -> io::Result<()> {
        match self.params().to {
            Some(to) => self.device_mut().send_timeout(data, Some(to)),
            None => self.device_mut().send(data),
        }
    }
    fn receive(&mut self) -> io::Result<LxiData> {
        match self.params().to {
            Some(to) => self.device_mut().receive_timeout(Some(to)),
            None => self.device_mut().receive(),
        }
    }
    fn send_timeout(&mut self, data: &[u8], to: Duration) -> io::Result<()> {
        self.device_mut().send_timeout(data, Some(to))
    }
    fn receive_timeout(&mut self, to: Duration) -> io::Result<LxiData> {
        self.device_mut().receive_timeout(Some(to))
    }

    fn with_timeout(mut self, to: Duration) -> Self {
        self.set_timeout(to);
        self
    }
    fn set_timeout(&mut self, to: Duration) {
        self.params_mut().to = Some(to);
    }
    fn reset_timeout(&mut self) {
        self.params_mut().to = None;
    }
    fn timeout(&self) -> Duration {
        match self.params().to {
            Some(to) => to,
            None => self.device().timeout().unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct HandleParams {
    to: Option<Duration>,
}

impl HandleParams {
    pub fn new() -> Self {
        HandleParams { to: None }
    }
}
