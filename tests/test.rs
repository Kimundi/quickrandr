extern crate xrandr_profile;

const EXAMPLE1: &str = include_str!("xrandr_output_1.txt");
const EXAMPLE2: &str = include_str!("xrandr_output_2.txt");
const EXAMPLE3: &str = include_str!("xrandr_output_3.txt");

#[test]
fn test() {
    let x = xrandr_profile::parse_xrandr(EXAMPLE1);

    println!("Example setup:");
    for (name, o) in x {
        println!("Display: {}, edid: {}", name, o.edid);
        println!("    Geometry: {:?}", o.geometry);
    }
}

#[test]
fn test2() {
    let x = xrandr_profile::parse_xrandr(EXAMPLE2);

    println!("Example setup:");
    for (name, o) in x {
        println!("Display: {}, edid: {}", name, o.edid);
        println!("    Geometry: {:?}", o.geometry);
    }
}

#[test]
fn test3() {
    let x = xrandr_profile::parse_xrandr(EXAMPLE3);

    println!("Example setup:");
    for (name, o) in x {
        println!("Display: {}, edid: {}", name, o.edid);
        println!("    Geometry: {:?}", o.geometry);
    }
}

#[test]
fn json() {
    let j = r#"{
        "configs" : [
            {
                "LVDS1" : {
                    "edid" : "00ffffffffffff0030e435030000000000150103801f1178ead1059558578f2920505400000001010101010101010101010101010101121b5668500012302020350036ae10000019000000000000000000000000000000000000000000fe004c4720446973706c61790a2020000000fe004c503134305748322d544c463100d5",
                    "geometry" : {
                        "width" : 1366,
                        "height" : 768,
                        "x_offset" : 0,
                        "y_offset" : 0,
                        "orientation" : "Normal",
                        "is_primary" : true
                    }
                }
            },
            {
                "LVDS1" : {
                    "edid" : "00ffffffffffff0030e435030000000000150103801f1178ead1059558578f2920505400000001010101010101010101010101010101121b5668500012302020350036ae10000019000000000000000000000000000000000000000000fe004c4720446973706c61790a2020000000fe004c503134305748322d544c463100d5",
                    "geometry" : {
                        "width" : 1366,
                        "height" : 768,
                        "x_offset" : 0,
                        "y_offset" : 0,
                        "orientation" : "Normal",
                        "is_primary" : true
                    }
                },
                "HDMI2" : {
                    "edid" : "00ffffffffffff00410c68086a5b000022120103802f1d78eeb505a5564a9a25125054bfef80b3008180818f9500950fa940010101017c2e90a0601a1e4030203600da281100001a000000ff0046453330383334323334303238000000fc005068696c697073203232305357000000fd00384c1e5311000a202020202020006a",
                    "geometry" : null
                }
            }
        ],
        "default" : {
            "LVDS1" : "--auto --primary",
            "VGA1" : "--auto --left-of LVDS1",
            "DP1" : "--auto --left-of LVDS1",
            "DP2" : "--auto --left-of LVDS1",
            "DP2" : "--auto --left-of LVDS1",
            "HDMI1" : "--auto --left-of LVDS1",
            "HDMI2" : "--auto --left-of LVDS1",
            "HDMI2" : "--auto --left-of LVDS1"
        }
    }"#;

    xrandr_profile::parse_json(j).unwrap();

}
