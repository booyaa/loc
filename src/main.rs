
extern crate wifilocation;
use std::env;


fn main() {
    // TODO: add use a proper args parsing package
    if env::args().count() > 1 {
        println!("loc 0.1.0 - Displays your latitude and longtitude");
        return;
    }

    use wifilocation;
    let loc = wifilocation::get_location(wifilocation::get_towers());
    let message = match loc {
        Ok(x) => format!("{} {}", x.location.lat, x.location.lng),
        _ => format!("Could not find location!"),
    };

    println!("{}", message);
}
