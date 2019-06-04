use std::time::Duration;

use crate::{
    KsFc,
    format::{parse, into_text, secs_as_dur, dur_as_secs}
};


// Configure commands
impl KsFc {
    /// `SENSe:FREQuency:GATE:TIME <time>`
    pub fn sense_frequency_gate_time_set(&mut self, time: Duration) -> crate::Result<()> {
        self.send(format!("FREQ:GATE:TIME {}", dur_as_secs(time)).as_bytes())
        .and_then(|()| self.system_error())
        .and_then(|e| match e {
            Some(e) => Err(e.into()),
            None => Ok(()),
        })
    }

    /// `SENSe:FREQuency:GATE:TIME?`
    pub fn sense_frequency_gate_time_get(&mut self) -> crate::Result<Duration> {
        self.send(b"FREQ:GATE:TIME?")
        .and_then(|()| self.receive())
        .and_then(|data| into_text(data))
        .and_then(|text| {
            parse::<f64>(&text)
            .map(|s| secs_as_dur(s).unwrap())
            .map_err(|e| e.into())
        })
    }
}
