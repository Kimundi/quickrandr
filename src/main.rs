extern crate xrandr_profile;

fn main() {
    let x = xrandr_profile::query_xrandr().unwrap();
    let x = xrandr_profile::parse_xrandr(&x);

    println!("Current setup:");
    for (name, o) in x {
        println!("Display: {}, EDID: {}", name, o.edid);
        println!("    Geometry: {:?}", o.geometry);
    }
}
