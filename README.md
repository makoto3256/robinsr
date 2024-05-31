# Supported Version: 2.2.51, 2.2.53, 2.2.54
Run the game by clicking run.bat file.

Tool website: [https://freesr-tools.pages.dev](https://freesr-tools.pages.dev)

Start battle by entering any calyx in the map, DON'T ATTACK THE ENEMIES, IT WON'T WORK (maybe)

Some scenes might not loaded properly. If you stuck at loading screen, remove `persistent` file.

# RobinSR
Original: 

[https://git.xeondev.com/reversedrooms/RobinSR](https://git.xeondev.com/reversedrooms/RobinSR)

[https://git.xeondev.com/reversedrooms/JadeSR](https://git.xeondev.com/reversedrooms/JadeSR)

A Server emulator for turn-based anime game.
![screenshot](https://raw.githubusercontent.com/amizing25/robinsr/main/screenshot.png)

## Installation

#### Requirements

- [Rust](https://www.rust-lang.org/tools/install)

**NOTE**: Nightly Rust is required to build the project. To install it, first install
Rust itself, then run the following command:

```sh
rustup toolchain install nightly
rustup default nightly
```

#### Building

```sh
git clone https://github.com/amizing25/robinsr.git
cd robinsr
cargo install --path gameserver
cargo install --path sdkserver
```

## Usage

To begin using the server, you need to run both the SDK server and the game server. Just run the `run.bat` file.