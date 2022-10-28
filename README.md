## Instructions

To run Quinti-Maze on a Mac, Windows or (hopefully) Linux computer:

1. [Install Rust](https://www.rust-lang.org/tools/install)
1. [Install the SDL2 libraries](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries)
1. cd to `quinti-maze-2022/sim`
1. `cargo run --release`

## History
In 1982 I wrote a
[program that was published in Byte Magazine](https://archive.org/details/byte-magazine-1982-09-rescan/page/n25/mode/2up).
To celebrate the forty years that have past since, I decided
to rewrite it today, in embedded Rust.

I entered this program in the Byte Magazine games contest at the urging of my mother. I wasn't optimistic that it would win, but it squeaked in. The prize was $65 a page for the article, including the listing. This was the first time, but not the last, where I regretted not putting in more comments. That works out to almost a thousand 2022 dollars.

Even more lucrative was the callout on the first page where I offer to put the program on disk for $5. I must have gotten dozens of those.

The Applesoft BASIC source for the original program can be seen
[here](https://gist.github.com/rtsuk/929585ba97c2a7270affd4120935edce). If you have trouble understanding it, don't feel
bad, I can't easily understand it either.
I was able to get it to work in an [Apple II emulator](https://storage.googleapis.com/tsuk-large-media/maze.mov).

When I wrote this, I had no notion how to generate a maze, so I did the best I could; I randomly added a lot
of doors and hoped there would be a path out.

## Plans

### Hardware

Here's the target hardware for the 2022 version, all stuff I had laying around from my escape room prop hobby.

#### [Adafruit Feather M4](https://www.adafruit.com/product/3857)

The feather is 120 times faster than the Apple II by raw clock speed, not to mention being a 32 bit processor instead of the 8 bit 6502. The feather's 160k of RAM is not too far off. The 512kb of flash storage is more than three Apple II floppy disks.

#### [TFT FeatherWing - 2.4" 320x240 Touchscreen For All Feathers](https://www.adafruit.com/product/3315)

The Apple II HiRes graphics mode had a resolution of 280 by 192 pixels, so this screen is in the right ballpark.

### Code

For the 2022 version I plan to implement an actual maze generation algorithm.
Luckily, I'm not the [first](https://crates.io/crates/knossos) to
[want](https://crates.io/crates/maze_generator) such
a [thing](https://crates.io/crates/irrgarten) so
I have code to reference.

For getting pixels onto the screen I can use the excellent [embedded-graphics](https://github.com/embedded-graphics) crates.
Someone has already written a driver for the screen, so I'm just on the hook for the touchscreen driver. The embedded graphics
folks also provide a desktop simulator, which will speed up development a bit.
