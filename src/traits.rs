use std::io;
use std::ops::{DerefMut};
use std::time::{Duration};

use lxi::LxiDevice;


pub trait Handle {
    type Device: DerefMut<Target=LxiDevice>;

    fn device(&self) -> &Self::Device;
    fn device_mut(&mut self) -> &mut Self::Device;

    fn send(&mut self, data: &[u8]) -> io::Result<()>;
    fn receive(&mut self) -> io::Result<Vec<u8>>;
    fn send_timeout(&mut self, data: &[u8], to: Option<Duration>) -> io::Result<()>;
    fn receive_timeout(&mut self, to: Option<Duration>) -> io::Result<Vec<u8>>;

    fn with_timeout(self, to: Option<Duration>) -> Self;
    fn set_timeout(&mut self, to: Option<Duration>);
    fn reset_timeout(&mut self);
    fn timeout(&self) -> Option<Option<Duration>>;
}
