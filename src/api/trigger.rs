use std::time::Duration;

use crate::{
    KsFc,
    types::{TriggerSource},
    format::{parse, into_text, dur_as_secs, secs_as_dur},
};


impl KsFc {
    /// `TRIGger:COUNt <count>`
    pub fn trigger_count_set(&mut self, count: usize) -> crate::Result<()> {
        self.send(format!("TRIG:COUN {}", count).as_bytes())
        .and_then(|()| self.system_error())
        .and_then(|e| match e {
            Some(e) => Err(e.into()),
            None => Ok(()),
        })
    }
    /// `TRIGger:COUNt?`
    pub fn trigger_count_get(&mut self) -> crate::Result<usize> {
        self.send(b"TRIG:COUN?")
        .and_then(|()| self.receive())
        .and_then(|data| into_text(data))
        .and_then(|text| parse::<usize>(&text).map_err(|e| e.into()))
    }

    /// `TRIGger:DELay <time>`
    pub fn trigger_delay_set(&mut self, time: Duration) -> crate::Result<()> {
        self.send(format!("TRIG:DEL {}", dur_as_secs(time)).as_bytes())
        .and_then(|()| self.system_error())
        .and_then(|e| match e {
            Some(e) => Err(e.into()),
            None => Ok(()),
        })
    }
    /// `TRIGger:DELay?`
    pub fn trigger_delay_get(&mut self) -> crate::Result<Duration> {
        self.send(b"TRIG:DEL?")
        .and_then(|()| self.receive())
        .and_then(|data| into_text(data))
        .and_then(|text| {
            parse::<f64>(&text)
            .map(|s| secs_as_dur(s).unwrap())
            .map_err(|e| e.into())
        })
    }

    /// `TRIGger:SOURse <source>`
    pub fn trigger_source_set(&mut self, source: TriggerSource) -> crate::Result<()> {
        let text = format!("TRIG:SOUR {}", match source {
            TriggerSource::Immediate => "IMM",
            TriggerSource::External => "EXT",
            TriggerSource::Bus => "BUS",
        });
        self.send(text.as_bytes())
        .and_then(|()| self.system_error())
        .and_then(|e| match e {
            Some(e) => Err(e.into()),
            None => Ok(()),
        })
    }
}

