extern crate quickrandr;

const EXAMPLE1: &str = include_str!("xrandr_output_1.txt");
const EXAMPLE2: &str = include_str!("xrandr_output_2.txt");
const EXAMPLE3: &str = include_str!("xrandr_output_3.txt");

#[test]
fn test() {
    let _ = quickrandr::parse_xrandr(EXAMPLE1);
}

#[test]
fn test2() {
    let _ = quickrandr::parse_xrandr(EXAMPLE2);
}

#[test]
fn test3() {
    let _ = quickrandr::parse_xrandr(EXAMPLE3);
}

#[test]
fn json() {
    let j = r#"{
        "autoprofiles": [
            {
                "HDMI2": {
                    "edid": "00ffffffffffff00410c68086a5b000022120103802f1d78eeb505a5564a9a25125054bfef80b3008180818f9500950fa940010101017c2e90a0601a1e4030203600da281100001a000000ff0046453330383334323334303238000000fc005068696c697073203232305357000000fd00384c1e5311000a202020202020006a",
                    "geometry": {
                        "width": 1680,
                        "height": 1050,
                        "x_offset": 1366,
                        "y_offset": 0,
                        "orientation": "Normal",
                        "is_primary": false
                    }
                },
                "LVDS1": {
                    "edid": "00ffffffffffff0030e435030000000000150103801f1178ead1059558578f2920505400000001010101010101010101010101010101121b5668500012302020350036ae10000019000000000000000000000000000000000000000000fe004c4720446973706c61790a2020000000fe004c503134305748322d544c463100d5",
                    "geometry": {
                        "width": 1366,
                        "height": 768,
                        "x_offset": 0,
                        "y_offset": 0,
                        "orientation": "Normal",
                        "is_primary": true
                    }
                }
            },
            {
                "LVDS1": {
                    "edid": "00ffffffffffff0030e435030000000000150103801f1178ead1059558578f2920505400000001010101010101010101010101010101121b5668500012302020350036ae10000019000000000000000000000000000000000000000000fe004c4720446973706c61790a2020000000fe004c503134305748322d544c463100d5",
                    "geometry": {
                        "width": 1366,
                        "height": 768,
                        "x_offset": 0,
                        "y_offset": 0,
                        "orientation": "Normal",
                        "is_primary": true
                    }
                }
            },
            {
                "DP2": {
                    "edid": "00ffffffffffff0010acb9a053413732191b0104a53420783a0495a9554d9d26105054a54b00714f8180a940d1c0d100010101010101283c80a070b023403020360006442100001e000000ff00374d54303137364c323741530a000000fc0044454c4c2055323431350a2020000000fd00313d1e5311000a2020202020200145",
                    "geometry": {
                        "width": 1920,
                        "height": 1200,
                        "x_offset": 1200,
                        "y_offset": 352,
                        "orientation": "Normal",
                        "is_primary": true
                    }
                },
                "LVDS1": {
                    "edid": "00ffffffffffff0030e435030000000000150103801f1178ead1059558578f2920505400000001010101010101010101010101010101121b5668500012302020350036ae10000019000000000000000000000000000000000000000000fe004c4720446973706c61790a2020000000fe004c503134305748322d544c463100d5",
                    "geometry": null
                },
                "DP3": {
                    "edid": "00ffffffffffff0010acb9a053383732191b0104a53420783a0495a9554d9d26105054a54b00714f8180a940d1c0d100010101010101283c80a070b023403020360006442100001e000000ff00374d54303137364c323738530a000000fc0044454c4c2055323431350a2020000000fd00313d1e5311000a2020202020200157",
                    "geometry": {
                        "width": 1200,
                        "height": 1920,
                        "x_offset": 0,
                        "y_offset": 0,
                        "orientation": "Left",
                        "is_primary": false
                    }
                }
            },
            {
                "LVDS1": {
                    "edid": "00ffffffffffff0030e435030000000000150103801f1178ead1059558578f2920505400000001010101010101010101010101010101121b5668500012302020350036ae10000019000000000000000000000000000000000000000000fe004c4720446973706c61790a2020000000fe004c503134305748322d544c463100d5",
                    "geometry": {
                        "width": 1366,
                        "height": 768,
                        "x_offset": 0,
                        "y_offset": 0,
                        "orientation": "Normal",
                        "is_primary": true
                    }
                },
                "HDMI1": {
                    "edid": "00ffffffffffff001ab3e807a89301002715010380331d782afe25a85337ae24115054a54b00a940b300950081008180010101010101023a801871382d40582c9600fd1e11000018000000fd00314c0f5211000a202020202020000000fc00503233542d36204950530a2020000000ff00595633513130333333360a2020006f",
                    "geometry": {
                        "width": 1920,
                        "height": 1080,
                        "x_offset": 1366,
                        "y_offset": 0,
                        "orientation": "Normal",
                        "is_primary": false
                    }
                }
            }
        ],
        "profiles": {
            "default": {
                "outputs": {
                    "LVDS1": "--auto --primary"
                },
                "other_outputs": "--auto --right-of LVDS1"
            },
            "only-lvds": {
                "outputs": {
                    "LVDS1": "--auto --primary"
                },
                "other_outputs": "--off"
            }
        }
    }"#;

    quickrandr::parse_json(j).unwrap();

}
