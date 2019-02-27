use std::io;
use std::time::{Duration};
use std::thread::{sleep};

use ksfc_lxi::{Fc, types::{EventReg}, error::{Error}};


static VAL: f64 = 7e3;
static REPS: f64 = 1e-2;
static MEAS_TIME: Duration = Duration::from_secs(1);


fn assert_val(val: Result<f64, Error>) {
    let v = match val {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let eps = REPS*VAL.abs();
    if (v - VAL).abs() > eps {
        panic!("Invalid value: {} != {}", v, VAL);
    }
}


macro_rules! test_all {
    ( $fc_new:expr, [ $( $func:ident ),* ] ) => {{
        $( {
            match std::panic::catch_unwind(|| {
                let mut fc = $fc_new;
                fc.api().rst().unwrap();
                $func(&mut fc).unwrap();
            }) {
                Ok(()) => println!("[ ok ] {}", stringify!($func)),
                Err(e) => {
                    println!("[fail] {}", stringify!($func));
                    std::panic::resume_unwind(e);
                },
            }
        } )*
    }};
}

fn main() {
    test_all!(
        Fc::new(&"10.0.0.9", None, Duration::from_secs(10)).unwrap(),
        [
            test_abort,
            test_cls,
            //test_cal,
            test_ese,
            test_autoscale,
            test_init,
            test_fetch,
            test_read,
            test_r
        ]
    );
}


fn test_abort(fc: &mut Fc) -> io::Result<()> {
    fc.api().abort()
}

fn test_cls(fc: &mut Fc) -> io::Result<()> {
    fc.api().cls()
}

#[allow(dead_code)]
fn test_cal(fc: &mut Fc) -> io::Result<()> {
    assert!(fc.api().cal()?);
    Ok(())
}

fn test_ese(fc: &mut Fc) -> io::Result<()> {
    let mut api = fc.api();
    let mut ese = api.ese();
    ese.set(EventReg::all())?;
    assert_eq!(ese.get()?, EventReg::all());
    Ok(())
}

fn test_autoscale(fc: &mut Fc) -> io::Result<()> {
    fc.api().autoscale()
}

fn test_init(fc: &mut Fc) -> io::Result<()> {
    fc.api().initiate()?.unwrap();
    assert_eq!(fc.api().initiate()?, Err(Error::new(-213).unwrap()));
    sleep(MEAS_TIME);
    Ok(())
}

fn test_fetch(fc: &mut Fc) -> io::Result<()> {
    assert_eq!(fc.api().initiate().unwrap(), Ok(()));
    assert_val(fc.api().fetch().unwrap());
    Ok(())
}

fn test_read(fc: &mut Fc) -> io::Result<()> {
    assert_val(fc.api().read()?);
    Ok(())
}


fn test_r(fc: &mut Fc) -> io::Result<()> {
    fc.api().initiate()?.unwrap();
    sleep(Duration::from_secs(1));
    println!("R: {:?}", String::from_utf8_lossy(fc.api().r(None).unwrap().as_ref()));
    Ok(())
}
