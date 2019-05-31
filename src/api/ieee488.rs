use crate::format::{parse, into_text};
use crate::types::*;
use crate::constants::*;
use crate::{KsFc};


// IEEE-488 Common commands
impl KsFc {
    /// `*CAL?`
    pub fn cal(&mut self) -> crate::Result<bool> {
        self.send(b"*CAL?")
        .and_then(|()| self.receive_timeout(CAL_TIMEOUT))
        .and_then(|data| into_text(data))
        .and_then(|text| parse::<i32>(&text).map_err(|e| e.into()))
        .map(|t| t == 0)
    }

    /// `*CLS`
    pub fn cls(&mut self) -> crate::Result<()> {
        self.send(b"*CLS")
    }

    /// `*RST`
    pub fn rst(&mut self) -> crate::Result<()> {
        self.send(b"*RST")
    }

    /// `*IDN?`
    pub fn idn(&mut self) -> crate::Result<String> {
        self.send(b"*IDN?")
        .and_then(|_| self.receive())
        .and_then(|data| into_text(data))
    }

    /// `*ESE?`
    pub fn ese_get(&mut self) -> crate::Result<EventReg> {
        self.send(b"*ESE?")
        .and_then(|()| self.receive())
        .and_then(|data| into_text(data))
        .and_then(|buf| parse!(&buf, u8).map_err(|e| e.into()))
        .map(|b| EventReg::from_bits_truncate(b.0))
    }

    /// `*ESE <bits>`
    pub fn ese_set(&mut self, ereg: EventReg) -> crate::Result<()> {
        self.send(format!("*ESE {}", ereg.bits()).as_bytes())
    }
}