# Emulating a Vixen CPU

The Vixen source code contains a barebones emulator engine to run a reference implementation of the architecture on your favorite platform. This is the recommended way of experimenting with the architecture as of now.

If you are interested in getting a physical Vixen system-on-a-chip made, feel free to reach out to us!

## Installing the emulator

First, make sure you have git and Rust installed. Then, first clone this repository:
```
git clone https://github.com/floofi-org/vixen.git
cd vixen
```

Once that's done, compile the emulator with release optimizations:
```
cargo build --release
```

And run the emulator:
```
./target/release/vixen-emulator
```

(or `cargo run -p vixen-emulator`)

This will use the file `rom.bin` as a system ROM and boot from it.