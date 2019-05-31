use std::fmt;
use std::error::{Error};
use std::str::FromStr;
use std::time::Duration;

use ks_lxi::{KsData};


// KsData conversion yielding crate error

#[derive(Debug)]
pub enum DTError {
    NotText,
    NotBin,
}

pub fn into_text(data: KsData) -> crate::Result<String> {
    data.into_text().ok_or(DTError::NotText.into())
}

pub fn into_bin(data: KsData) -> crate::Result<Vec<u8>> {
    data.into_bin().ok_or(DTError::NotBin.into())
}

// conversions

pub fn secs_as_dur(sec: f64) -> Option<Duration> {
    let i = sec.floor();
    if i < 0.0 {
        return None;
    }
    let f = sec.fract();
    Some(
        Duration::from_secs(i as u64) +
        Duration::from_nanos((1e9*f) as u64)
    )
}

pub fn dur_as_secs(dur: Duration) -> f64 {
    (dur.as_secs() as f64) + 1e-9*(dur.subsec_nanos() as f64)
}


// parsing from string

#[derive(Debug)]
pub enum ParseError {
    Parse(Box<dyn Error + Send + Sync>),
    EndOfString,
    TooFewArgs,
    Arg(usize, Box<dyn Error + Send + Sync>),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Parse(e) => write!(f, "ParseError::Parse({})", e),
            ParseError::Arg(n, e) => write!(f, "ParseError::Arg({}, {})", n, e),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Error for ParseError {}

pub fn parse<T: FromStr>(text: &str) -> Result<T, ParseError>
where T::Err: 'static + Error + Send + Sync {
    text.parse::<T>().map_err(|e| ParseError::Parse(e.into()))
}

#[macro_use]
macro_rules! parse {
    ( $s:expr, $( $x:ty ),*) => {{
        let parse_fn = |text: &str| -> Result<( $( $x, )* ), crate::format::ParseError> {
            let mut si = text.split(',');
            let mut i = 0;
            let res = ( $( {
                    i += 1;
                    let ss = si.next().ok_or(crate::format::ParseError::EndOfString)?;
                    let arg = ss.parse::<$x>().map_err(|e| {
                        crate::format::ParseError::Arg(i - 1, e.into())
                    })?;
                    arg
            }, )* );
            if si.next().is_some() {
                return Err(crate::format::ParseError::TooFewArgs);
            }
            Ok(res)
        };
        parse_fn($s)
    }};
}


#[cfg(test)]
mod tests {
    #[test]
    fn parse_ints() {
        assert_eq!(parse!(&"-1,+2,+3"[..], i32, i32, i32).unwrap(), (-1, 2, 3))
    }

    #[test]
    fn parse_float() {
        assert_eq!(parse!(&"+9.91000000000000E+37"[..], f64).unwrap(), (9.91e37,))
    }
}
