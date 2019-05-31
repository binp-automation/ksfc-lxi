use crate::{KsFc};
use crate::format::{into_text, ParseError};
use crate::{KsDevErr};
use crate::constants::*;


impl KsFc {
    /// `SYSTem:ERRor?`
    pub fn system_error(&mut self) -> crate::Result<Option<KsDevErr>> {
        self.send(b"SYST:ERR?")
        .and_then(|()| self.receive())
        .and_then(|data| into_text(data))
        .and_then(|text| {
            text.splitn(2, ',').next().ok_or(ParseError::EndOfString)
            .and_then(|part| {
                parse!(part, i32).map(|v| v.0).map_err(|e| e.into())
            }).map_err(|e| e.into())
        })
        .and_then(|code| {
            if code == NO_ERROR_CODE {
                Ok(None)
            } else {
                match KsDevErr::new(code) {
                    Some(e) => Ok(Some(e)),
                    None => Err(format!("Unknown error code: '{}'", code).into()),
                }
            }
        })
    }
}

