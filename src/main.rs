use ksfc_lxi::{Fc, types::{EventReg}};

fn main() {
	let mut fc = Fc::new(&"10.0.0.9", None).unwrap();

    fc.api().abort().unwrap();
    fc.api().cls().unwrap();

    //println!("{:?}", fc.api().cal().unwrap());

    fc.api().ese().set(EventReg::all()).unwrap();
    println!("{:?}", fc.api().ese().get().unwrap());

    fc.api().autoscale().unwrap();

    fc.api().initiate().unwrap();
    println!("{:?}", fc.api().fetch().unwrap());
}
