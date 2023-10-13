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

### Options

> **NOTE**: None of the options are implemented yet.

There are two ways to set options: config file and command line arguments.
The CLI arguments override the config file.

**Options:**

- logging: Set logging level, written to stdout
  - off: No logging
  - error: Only errors (default)
  - warn: Errors and warnings
  - info: Errors, warnings, and info
  - debug: Errors, warnings, info, and debug
  - trace: All logging
- log-file: What file to writte logs to
- show-fps: Start the game with the FPS counter showing
- visual-debug: Show visual debug information, takes a list of options
  - none: No visual debug information (default)
  - colliders: Show colliders
  - normals: Show normals
  - velocity: Show velocity
  - impulses: Show impulses
  - forces: Show forces
  - camera: Show camera
  - all: Show all visual debug information
- scene
  - game: The actual game (default)
  - colliders: Every asset and their colliders
  - enemy-turret: player turret

#### Config file (TODO)

Options can be set in the `config.toml` file. The file is created on first run.
Otherwise the file is located in the same directory as the executable.
You can override the config file location with the `--config` command
line argument. Options

#### Command line arguments (TODO)

##### General

- **--help**: Prints help information
- **--version**: Prints version information
- **--logging**: Set logging level, written to stdout, takes one of the following
  - values: off, error, warn, info, debug, trace
- **--log-file**: What file to writte logs to
- **--show-fps**: Start the game with the FPS counter showing
- **--visual-debug**: Show visual debug information, takes a list of options
  - values: colliders, normals, velocity, impulses, forces, camera, all

##### Commands

###### scene \[scene\]

Special debug sceens for faster iteration speed.

- **colliders**: Every asset and their colliders
  - Move around with WASD
  - Zoom in and out with Q and E or the mouse wheel
- **enemy-turret**: player turret
  - Move around with player character

##

```bash
cargo run -- --help
```

## TODO

### Current

Current objective is to get a gameplay loop working.

- [ ] Make an infinite world
  - [ ] Sphere world?
  - [ ] Torus world?
  - [ ] Infinite plane world?
- [ ] Add enemy
- [ ] Spawn enemies in waves
- [ ] Add player health
- [ ] Add player death
- [ ] Add Score
- Add UI
  - [ ] Add Replay button
  - [ ] Add Game Over screen
  - [ ] Add Main Menu
  - [ ] Add Pause Menu
  - [ ] Add highscore list
- Add HUD
  - Add score
  - Add health

### Backlog

- Make character controller more realistic by adding thrusters
  - Rotational thrusters
  - Linear thrusters
  - Thrusters have max impulse they can generate
- Add HUD
  - Add boost fuel
  - Add shield
- Add visual player damage level.
- Keep track of player score
- Add 1 enemy type
- Add Health to player
- Spawn enemies in waves
- Add Background
- Add Boost
  - Boost fuel pickups
  - Boost fuel bar
  - Boost juice (sound, animation, etc)
- Add Shield
  - Shield pickups
  - Shield levels (three hits)
  - Shield juice (sound, animation, etc)
- Add sound effects
- Add music
- Add particle effects
- Add player death
- Add player respawn
- Add weapon pickups
- Add mine powerup
- Add examples for different developing scenarios
- Add visual debugging for camera
