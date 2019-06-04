//#![allow(dead_code)]

use std::time::{Duration};
use std::thread::{sleep};

use ksfc_lxi::{
    KsFc, Error,
    types::{EventReg, ChannelNo, TriggerSource},
};


static FREQ: f64 = 7e3;
static FREPS: f64 = 1e-2;
static MEAS_TIME: Duration = Duration::from_secs(1);


fn assert_feq(val: f64, refv: f64, reps: f64) {
    let eps = reps*refv.abs();
    if (val - refv).abs() > eps {
        panic!("Float assertion error: {} != {}, relative eps: {}", val, refv, reps);
    }
}

fn assert_freq(val: f64) {
    assert_feq(val, FREQ, FREPS);
}

macro_rules! test_all {
    ( $fc_new:expr, [ $( $func:ident, )* ] ) => {{
        $( {
            println!("{} ... ", stringify!($func));
            match std::panic::catch_unwind(|| {
                let mut fc = $fc_new;
                fc.rst().unwrap();
                $func(&mut fc);
            }) {
                Ok(()) => println!("... [ ok ]"),
                Err(e) => {
                    println!("... [FAIL]");
                    std::panic::resume_unwind(e);
                },
            }
        } )*
    }};
}

fn main() {
    test_all!(
        {
            let (dev, res) = KsFc::new(
                &"10.0.0.9", None,
                Duration::from_secs(2),
            );
            res.unwrap();
            dev
        },
        [
            test_abort,
            test_cls,
            //test_cal,
            test_ese,
            test_autoscale,
            test_init,
            test_fetch,
            test_read,
            test_trig_count,
            test_trig_delay,
            test_r,
            test_conf_freq,
            test_trig_source,
        ]
    );
}


fn test_abort(fc: &mut KsFc) {
    fc.abort().unwrap();
}

fn test_cls(fc: &mut KsFc) {
    fc.cls().unwrap();
}

#[allow(dead_code)]
fn test_cal(fc: &mut KsFc) {
    assert!(fc.cal().unwrap());
}

fn test_ese(fc: &mut KsFc) {
    fc.ese_set(EventReg::all()).unwrap();
    assert_eq!(fc.ese_get().unwrap(), EventReg::all());
}

fn test_autoscale(fc: &mut KsFc) {
    fc.autoscale().unwrap();
}

fn test_init(fc: &mut KsFc) {
    fc.initiate().unwrap();
    if let Err(Error::Dev(x)) = fc.initiate() {
        assert_eq!(x.code(), -213);
    }
    sleep(MEAS_TIME);
}

fn test_fetch(fc: &mut KsFc) {
    fc.initiate().unwrap();
    assert_freq(fc.fetch().unwrap());
}

fn test_read(fc: &mut KsFc) {
    assert_freq(fc.read().unwrap());
}

fn test_trig_count(fc: &mut KsFc) {
    fc.trigger_count_set(1).unwrap();
    assert_eq!(fc.trigger_count_get().unwrap(), 1);

    fc.trigger_count_set(1000000).unwrap();
    assert_eq!(fc.trigger_count_get().unwrap(), 1000000);

    if let Err(Error::Dev(x)) = fc.trigger_count_set(0) {
        assert_eq!(x.code(), -222);
    }
    if let Err(Error::Dev(x)) = fc.trigger_count_set(1000001) {
        assert_eq!(x.code(), -222);
    }
}

fn test_trig_delay(fc: &mut KsFc) {
    fc.trigger_delay_set(Duration::from_secs(10)).unwrap();
    assert_eq!(fc.trigger_delay_get().unwrap().as_secs(), 10);

    fc.trigger_delay_set(Duration::from_micros(10)).unwrap();
    assert_eq!(fc.trigger_delay_get().unwrap().as_micros(), 10);

    if let Err(Error::Dev(x)) = fc.trigger_delay_set(Duration::from_secs(3601)) {
        assert_eq!(x.code(), -222);
    }
}

fn test_r(fc: &mut KsFc) {
    fc.trigger_count_set(4).unwrap();
    fc.initiate().unwrap();
    sleep(Duration::from_secs(1));
    let r = fc.r(None).unwrap();
    assert_eq!(r.len(), 4);
    for x in r {
        assert_freq(x);
    }
}

fn test_conf_freq(fc: &mut KsFc) {
    fc.configure_frequency(ChannelNo::Ch1).unwrap();
    fc.initiate().unwrap();
    assert_freq(fc.fetch().unwrap());

    fc.configure_frequency(ChannelNo::Ch2).unwrap();
    fc.initiate().unwrap();
    assert_freq(fc.fetch().unwrap());
}


fn test_trig_source(fc: &mut KsFc) {
    fc.trigger_source_set(TriggerSource::Immediate).unwrap();
    fc.initiate().unwrap();
    assert_freq(fc.fetch().unwrap());
}
