mod esi;
mod evetrade;
mod urls;

use evetrade::Evetrade;

fn main() {
    println!("initializing logger...");
    let mut et = Evetrade::new();
    et.init();
}
