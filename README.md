<h2 align="center">SCATERS</h2>
<h4 align="center">A Chip 8 Emulator Written in Rust and SDL2</h4>

<p align="center">
  <img align="center" src="https://raw.githubusercontent.com/pacokwon/media/main/scaters/screenshot.png" width="650em"/>
</p>

This project is my attempt at building a Chip 8 emulator from scratch.

I have used [SDL2](https://www.libsdl.org/) and [Rust SDL2 Bindings](https://github.com/Rust-SDL2/rust-sdl2) for graphics.

## Installation
You will need `sdl2` installed on your machine. Consult the [Installation](https://wiki.libsdl.org/Installation) guide on the SDL wiki for installation details.

Next, clone the repository:
```bash
git clone https://github.com/pacokwon/scaters
```

Finally, run one of the following commands to build the emulator.
```bash
$ cargo build
$ cargo build --release   # optimized build
```

## References
* http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/Chip8.pdf
* http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.5

## LICENSE
This project is licensed under the MIT license from the very first commit.
