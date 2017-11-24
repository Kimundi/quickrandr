# quickrandr
A command line tool for saving and restoring xrandr display configurations

# Example config file:

```{.json}
{
    "autoprofiles": [
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
        },
        ...
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
}
```
