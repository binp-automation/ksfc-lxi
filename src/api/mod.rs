pub mod basic;
pub mod ieee488;
pub mod system;
pub mod trigger;


use crate::format::{into_text, parse};
use crate::constants::*;
use crate::{KsFc};


impl KsFc {
    fn receive_value(&mut self) -> crate::Result<f64> {
        self.receive()
        .and_then(|data| into_text(data))
        .and_then(|text| {
            parse::<f64>(&text).map_err(|e| e.into())
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
                Some(v) => Ok(v),
                None => {
                    self.system_error().and_then(|e| match e {
                        Some(e) => Err(e.into()),
                        None => Err(format!("Request failed but there is no errors").into()),
                    })
                },
            }
        })
    }
}