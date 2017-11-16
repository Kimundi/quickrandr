extern crate xrandr_profile;

const EXAMPLE1: &str = include_str!("xrandr_output_1.txt");
const EXAMPLE2: &str = include_str!("xrandr_output_2.txt");
const EXAMPLE3: &str = include_str!("xrandr_output_3.txt");

#[test]
fn test() {
    let x = xrandr_profile::parse_xrandr(EXAMPLE1);

    println!("Example setup:");
    for o in x.connected_outputs {
        println!("Display: {}, EDID: {}", o.name, o.edid_raw);
        println!("    Geometry: {:?}", o.geometry);
    }
}

#[test]
fn test2() {
    let x = xrandr_profile::parse_xrandr(EXAMPLE2);

    println!("Example setup:");
    for o in x.connected_outputs {
        println!("Display: {}, EDID: {}", o.name, o.edid_raw);
        println!("    Geometry: {:?}", o.geometry);
    }
}

#[test]
fn test3() {
    let x = xrandr_profile::parse_xrandr(EXAMPLE3);

    println!("Example setup:");
    for o in x.connected_outputs {
        println!("Display: {}, EDID: {}", o.name, o.edid_raw);
        println!("    Geometry: {:?}", o.geometry);
    }
}

#[macro_use]
extern crate json;

#[test]
fn json() {
    let j = array![
        object!{
            "displays" => object![
                "LVDS1" => "00ffffffffffff0030e435030000000000150103801f1178ead1059558578f2920505400000001010101010101010101010101010101121b5668500012302020350036ae10000019000000000000000000000000000000000000000000fe004c4720446973706c61790a2020000000fe004c503134305748322d544c463100d5"
            ]
        },
        object!{
            "displays" => object![
                "LVDS1" => "00ffffffffffff0030e435030000000000150103801f1178ead1059558578f2920505400000001010101010101010101010101010101121b5668500012302020350036ae10000019000000000000000000000000000000000000000000fe004c4720446973706c61790a2020000000fe004c503134305748322d544c463100d5",
                "HDMI2" => "00ffffffffffff00410c68086a5b000022120103802f1d78eeb505a5564a9a25125054bfef80b3008180818f9500950fa940010101017c2e90a0601a1e4030203600da281100001a000000ff0046453330383334323334303238000000fc005068696c697073203232305357000000fd00384c1e5311000a202020202020006a"
            ]
        },
        object!{
            "displays" => object![
                "LVDS1" => "00ffffffffffff0030e435030000000000150103801f1178ead1059558578f2920505400000001010101010101010101010101010101121b5668500012302020350036ae10000019000000000000000000000000000000000000000000fe004c4720446973706c61790a2020000000fe004c503134305748322d544c463100d5",
                "DP2" => "00ffffffffffff0010acb9a053413732191b0104a53420783a0495a9554d9d26105054a54b00714f8180a940d1c0d100010101010101283c80a070b023403020360006442100001e000000ff00374d54303137364c323741530a000000fc0044454c4c2055323431350a2020000000fd00313d1e5311000a2020202020200145",
                "DP3" => "00ffffffffffff0010acb9a053383732191b0104a53420783a0495a9554d9d26105054a54b00714f8180a940d1c0d100010101010101283c80a070b023403020360006442100001e000000ff00374d54303137364c323738530a000000fc0044454c4c2055323431350a2020000000fd00313d1e5311000a2020202020200157"
            ]
        }
    ];

    xrandr_profile::parse_json(j).unwrap();
}
