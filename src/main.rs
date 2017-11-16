extern crate xrandr_profile;

fn main() {
    let x = xrandr_profile::query_xrandr().unwrap();
    let x = xrandr_profile::parse_xrandr(&x);

    println!("Current setup:");
    for o in x.connected_outputs {
        println!("Display: {}, EDID: {}", o.name, o.edid_raw);
        println!("    Geometry: {:?}", o.geometry);
    }
}
