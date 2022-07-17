## Description

This is a tiny application that allows you to set your monitors' resolution, refresh rate, scaling mode, position and rotation in the command-line.
Typically this would be done using the Nvidia control panel, but due to its lack of a cli interface, it is difficult to automate those changes.

Note that this program only works on Windows computers. 

## Installation

The 64 bit binary can be downloaded from the releases page. In order to get a 32 bit installation, the project must be built from source. This process can be simplified with cargo by running `cargo install nvcli`.

## Usage

Help text will be printed if the program is run with no arguments or by passing the `--help` option.

Each invocation of the program applies the options passed to it.
By default the settings apply to the primary monitor if there are multiple monitors connected.
In order to target a different monitor, passing the monitor's display id to the `--display` option causes the other specified options to apply to that monitor instead.

In order to obtain a monitor's display id, run `nvcli -l` to list the connected displays.
This lists each source and their targets (the vast majority of the time each source only has one target).
The display id of each target can be found in the listed information

The valid values for each setting can be found by looking in the Nvidia control panel and seeing what options are available.

Gamma settings can be set by installing GeForce Experience and using Freestyle to apply a filter.

## Examples

### Listing displays

Running `nvcli -l` will output something that looks like this:
```
Source
Primary: true                   (Whether this source is the primary monitor)
Resolution: 2560x1440           (The current resolution of this output)
Position: (0,0)                 (The current position of this monitor)
Target 1				
ID: 2147881089                  (The display id of this target, which is used to change which monitor settings are applied to)
Refresh rate: 165 Hz            (The current refresh rate)
Scaling: balanced full screen   (The current scaling mode)
Rotation: 0                     (The current display clockwise rotation in degrees)

Source
Primary: false
Resolution: 1920x1080
Position: (2560,0)
Target 1
ID: 2147881090
Refresh rate: 120 Hz
Scaling: balanced aspect ratio
Rotation: 0
```

### Stretched

If the primary monitor is originally configured to have 2560x1440 resolution, 165Hz refresh rate and balanced full screen scaling,
setting the monitor to 4:3 stretched can be done with any of the following commands:
```
nvcli -w 1920 -h 1440 -s ffs
``` 
(`bfs` also works here as there is no discernible difference between the forced and balanced scaling options).
```
nvcli -w 1920
```
(other settings remain the same if unspecified)

### Non-primary monitors

Changing the refresh rate of a non-primary monitor with display id `2147881090` to 120Hz:
```
nvcli -d 2147881090 -r 120
```

### Changing position/rotation of secondary monitor

If there are two monitors both of resolution 1920x1080, the following command would place the secondary monitor above the primary monitor:
```
nvcli -d 2147881090 -X 0 -Y=-1080
```
The following command would place the secondary monitor right beside the primary monitor on its right:
```
nvcli -d 2147881090 -X 1920 -Y 0
```
The following command would rotate the secondary monitor to vertical and place it centered above the primary monitor:
```
nvcli -d 2147881090 -R 90 -X 420 -Y=-1920
```

## Notes

Output color can be disabled by setting the environment variable `NO_COLOR=1`.