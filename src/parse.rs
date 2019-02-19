use std::io;
use std::fmt;
use std::error::{Error};


macro_rules! parse_bytes {
    ( $s:expr, $( $x:ty ),*) => {{
        let parse_fn = |buf: &[u8]| -> Result<( $( $x, )* ), ParseError> {
            let s = String::from_utf8_lossy(buf);
            let mut si = s.split(',');
            let mut i = 0;
            let res = ( $( {
                    i += 1;
                    let ss = si.next().ok_or(ParseError::EndOfString)?;
                    let arg = ss.parse::<$x>().map_err(|_| ParseError::Arg(i - 1))?;
                    arg
            }, )* );
            if si.next().is_some() {
                return Err(ParseError::TooFewArgs);
            }
            Ok(res)
        };
        parse_fn($s)
    }};
}

#[derive(Debug)]
pub enum ParseError {
    EndOfString,
    TooFewArgs,
    Arg(usize),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

impl Into<io::Error> for ParseError {
    fn into(self) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ints() {
        assert_eq!(parse_bytes!(&b"-1,+2,+3"[..], i32, i32, i32).unwrap(), (-1, 2, 3))
    }

    #[test]
    fn parse_float() {
        assert_eq!(parse_bytes!(&b"+9.91000000000000E+37"[..], f64).unwrap(), (9.91e37,))
    }
}
