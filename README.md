# MainUI Game Picker

Randomly picks a ROM and prints it, can be passed to a launch script.

Uses the system's `config.json`:
* `extlist`: used for building regex for gathering rom lists
* `rompath`: does what it says on the tin, if it's not in the config we use `/mnt/SDCARD/Roms/<system>`

**Not tested on MinUI (unlikely to work in its current state)**

To build for both armv7 (Cortex-A7) and arm64 (Cortex-A53):
```
$ just release
```

Uses [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) to make cross-compilation stupid easy
