# MainUI Game Picker

Randomly picks a ROM and prints it, can be passed to a launch script.

Uses the system's `config.json`:
* `extlist`: used for building regex for gathering rom lists
* `rompath`: does what it says on the tin, if it's not in the config we use `/mnt/SDCARD/Roms/<system>`

**Not tested on MinUI (unlikely to work in its current state)**

## Building

To build for both armv7 (Cortex-A7) and arm64 (Cortex-A53):
```
$ just release
```

Uses [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) to make cross-compilation stupid easy

## Usage

Use as part of an app in MainUI

```
# get a random game from any system and print it
$ ./mainui-game-picker
/mnt/SDCARD/Roms/FC/Super Mario Bros. (World).zip

# get a random game from SFC and print it
$ ./mainui-game-picker SFC
/mnt/SDCARD/Roms/SFC/Breath of Fire II (USA).zip

# get a random game from any system and launch it using the randomly-picked system's launch script
# must be used as part of an app or X-menu script due to using execv instead of writing to /tmp/cmd_to_run.sh
$ ./mainui-game-picker GBC --launch
/mnt/SDCARD/Roms/MD/Sonic the Hedgehog (Japan, Europe).zip
...

# get a random game from GBC and launch it using the GBC system's launch script
# must be used as part of an app or X-menu script due to using execv instead of writing to /tmp/cmd_to_run.sh
$ ./mainui-game-picker GBC --launch
/mnt/SDCARD/Roms/SFC/Pokemon - Crystal Version (USA).zip
...
```


