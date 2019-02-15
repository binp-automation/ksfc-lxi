use ksfc_lxi::{KsFc, EventReg};

fn main() {
	let mut fc = KsFc::new(&"10.0.0.9", None).unwrap();

    //fc.api().abort().unwrap();
    //fc.api().autoscale().unwrap();
    fc.api().cls().unwrap();
    //println!("{:?}", fc.api().cal().unwrap());
    fc.api().ese().set(EventReg::all()).unwrap();
    println!("{:?}", fc.api().ese().get().unwrap());

}
