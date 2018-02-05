MIDI-to-Keypress
================

Takes MIDI events and turns them into keypresses.  Mostly designed for "Perform" in FFXIV.

See [Releases](https://github.com/xobs/midi-to-keypress/releases) for a list of releases.

Installing
----------

You only need to download the [latest release](https://github.com/xobs/midi-to-keypress/releases/latest).  Download it, unzip it, and run it from cmd.exe.

Building
--------

This program requires Rust.  Download it from [rustup.rs](https://rustup.rs).

To and run, go into this directory and type:

````
cargo run
````

Usage
-----

To list available devices, run "miditran --list".  To specify a device to use as an input, run "miditran --device [device-name]".

Currently, there is no external configuration.  The program will search for a device named MIDI\_DEV\_NAME, and will monitor key events from that device.

For channel 0 (i.e. the main keys), it will translate keys 40-61 into the following keyboard piano:

````
  2 3   5 6 7
 Q W E R T Y U I
````

For keys one octave below C-4, it will additionally press the Ctrl key.  For keys one octave above C-4, it will instead press the Shift key.

For channel 9 (i.e. the drum pads above), pressing pads 1-4 will press Esc, followed by Ctrl+Alt+Shift+{Z, X, C, or V}.  This can be used to switch instruments.