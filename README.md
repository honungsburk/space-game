# Space Game

Physics based space game. In development.

## Getting Started

### Prerequisites

Install [rustup](https://rustup.rs/) and use it to install the latest stable version of Rust and cargo.

### Installing

```bash
cargo install
```

### Running

```bash
cargo run
```

### Settings (Not implemented yet)

Settings can be set in the `settings.toml` file. The file is created on first run.
Otherwise the file is located in the same directory as the executable.

```toml
[window]
width = 1280
height = 720
fullscreen = false
vsync = true

[accessibility]
# TODO: Add accessibility options

[keybindings.game]
shoot = ["space"]
# TODO: Add keybindings

[keybindings.development]
restart = ["r"]
# TODO: Add keybindings

```

### Options (Not implemented yet)

There are two ways to set options: config file and command line arguments.
The CLI arguments override the config file.

**Options:**

- loglevel: Set logging level, written to stdout
  - off: No logging
  - error: Only errors (default)
  - warn: Errors and warnings
  - info: Errors, warnings, and info
  - debug: Errors, warnings, info, and debug
  - trace: All logging
- log-file: What file to writte logs to
- visual-debug: Show visual debug information, takes a list of options
  - none: No visual debug information (default)
  - colliders: Show colliders
  - normals: Show normals
  - velocity: Show velocity
  - impulses: Show impulses
  - forces: Show forces
  - camera: Show camera
  - fps: Show FPS
  - all: Show all visual debug information
- scene
  - game: The actual game (default)
  - assets: Every asset and their colliders
  - enemy-turret: play against a turret

#### Config file (TODO)

Options can be set in the `config.toml` file. The file is created on first run.
Otherwise the file is located in the same directory as the executable.
You can override the config file location with the `--config` command
line argument. Options

#### Command line arguments (TODO)

##### General

- **--help**: Prints help information
- **--version**: Prints version information
- **--log-level**: Set logging level, written to stdout, takes one of the following
  - values: off, error, warn, info, debug, trace
- **--log-file**: What file to writte logs to
- **--visual-debug**: Show visual debug information, takes a list of options
  - values: colliders, normals, velocity, impulses, forces, fps, camera, all
- **--config**: Path to config file
- **--settings**: Path to settings file

##### Commands

###### scene \[scene\]

Special debug sceens for faster iteration speed.

- **assets**: Every asset and their colliders
  - Move around with WASD
  - Zoom in and out with Q and E or the mouse wheel
- **enemy-turret**: player turret
  - Move around with player character

##

```bash
cargo run -- --help
```

### Resources

- [ThisFromThat](https://accessible.games/accessible-player-experiences/access-patterns/distinguish-this-from-that/)
- [Accessable Game Design](https://www.youtube.com/watch?v=4NGe4dzlukc)
- [Game Feel](https://www.youtube.com/watch?v=AJdEqssNZ-U)
