use crate::{KsFc};
use crate::format::{parse, into_bin};


// Basic commands
impl KsFc {
    /// `ABORt`
    pub fn abort(&mut self) -> crate::Result<()> {
        self.send(b"ABOR")
    }

    /// `AUToscale`
    pub fn autoscale(&mut self) -> crate::Result<()> {
        self.send(b"AUT")
    }

    /// `FETCh?`
    pub fn fetch(&mut self) -> crate::Result<f64> {
        self.send(b"FETC?").and_then(|()| self.receive_value())
    }

    /// `INITiate`
    pub fn initiate(&mut self) -> crate::Result<()> {
        self.send(b"INIT")
        .and_then(|()| self.system_error())
        .and_then(|e| match e {
            Some(e) => Err(e.into()),
            None => Ok(()),
        })
    }

    /// `READ?`
    pub fn read(&mut self) -> crate::Result<f64> {
        self.send(b"READ?").and_then(|()| self.receive_value())
    }

    /// `R? <N>`
    pub fn r(&mut self, max_count: Option<usize>) -> crate::Result<Vec<f64>> {
        match max_count {
            Some(n) => self.send(format!("R? {}", n).as_bytes()),
            None => self.send(b"R?"),
        }
        .and_then(|()| self.receive())
        .and_then(|data| into_bin(data))
        .and_then(|buf| String::from_utf8(buf).map_err(|_| "bad utf8 sequence".into()))
        .and_then(|text| -> crate::Result<Vec<f64>> {
            text.split(',')
            .map(|s| parse::<f64>(s).map_err(|e| e.into()) )
            .collect()
        })
    }
}
