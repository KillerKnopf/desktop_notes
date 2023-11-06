# desktop_notes

Very simple sticky notes app made in rust using eframe.
The settings button isn't working yet. It has no click event handler associated.
Notes won't be saved yet.
Notes won't be loaded from file yet.

## ToDo

- [ ] Settings
  - What settings should I include?
  - Where do the settings get stored?
- [ ] Saving and loading of notes
  - Where should I save to ?

## How to use

1. Install rust on your system.
2. Clone this repository.
3. In a terminal run
```
cargo run --release
```
or
```
cargo build --release
```
4. Cargo run will launch the app automatically. With cargo rebuild you have to start the app from /target/release/desktop_notes.exe

If you leave out --release, then on windows a console will be launched where you may get spammed with logging messages about the windows position.