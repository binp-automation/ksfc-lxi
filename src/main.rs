use ksfc_lxi::{Fc, types::{EventReg}};


fn main() {
    let mut fc = Fc::new(&"10.0.0.9", None).unwrap();

    let mut api = fc.api();
    api.abort().unwrap();
    api.cls().unwrap();

    println!("{:?}", api.cal().unwrap());

    let mut ese = api.ese();
    ese.set(EventReg::all()).unwrap();
    println!("{:?}", ese.get().unwrap());

    api.autoscale().unwrap();

    api.initiate().unwrap();
    println!("{:?}", api.fetch().unwrap());
}
