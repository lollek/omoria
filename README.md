# Omoria
A fork of Imoria with some fixes and changes, making it more fun to play for me.
This project has also involved into a way for me to try out the Rust language,
so I am in the process of porting it from C to Rust. So basically this project
is a port of a port (Pascal -> C -> Rust). I wouldn't recommend anyone else
compiling this project and running themselves, as there are probably a lot more
bugs you would like.

## Requirements
Build on macOS or Linux with:

* Rust and Cargo, including the rustfmt and Clippy components.
* Make and GCC or a compatible C compiler supporting GNU C99. The Makefile
	defaults to `gcc`; select another compiler with `make CC=your-compiler`.
* ncurses development headers and libraries, plus termcap-compatible linkage.
	The build also links the platform math, pthread, and dynamic-loader libraries.

On macOS, install the Xcode Command Line Tools for the C toolchain and Make;
install ncurses development files if they are not available to the compiler.
On Debian/Ubuntu Linux, install `build-essential` and `libncurses-dev`.
Install Rust with rustup and add the checks with
`rustup component add rustfmt clippy`. Non-system ncurses installations may
require include/library search paths; termcap library availability varies by
platform.

## Getting started
Run these commands from the repository root:

```sh
make       # Build the C objects, Rust static library, and linked omoria executable
make run   # Build if needed, then start the game in an interactive terminal
make check # Check formatting, Clippy, Rust tests, and the C/Rust build
```

The `make check` contract is `cargo fmt --check`, `cargo clippy --all-targets`,
`cargo test`, a full C/Rust build and link, `make test-movement`, and
`make test-messages`. The C boundary tests run without a terminal; message tests
use controlled input/drawing doubles. This verifies tested logic and that the
game links; it does **not** prove gameplay or run headless turn tests.
The gate passes locally on macOS. Existing Clippy warnings remain non-blocking,
and six legacy denied lints have scoped allowances to preserve existing behavior.
CI runs the same gate on macOS and Linux; runner results still need confirmation.

## Documentation

* [Source structure](docs/STRUCTURE.md)
* [C-to-Rust migration plan](docs/c-to-rust-migration-plan.md)
* [Testing roadmap and verification levels](docs/testing-roadmap.md)
* [Combat proposal (not implemented)](docs/proposals/combat-system-specification.md)
* [Historical item guide](docs/legacy/item_guide.txt)

# Original Imoria README (Historical)
(There may be some outdated information here, because of changes I have
introduced in this repo. This section is preserved for historical reference;
use the build instructions above for the current project.)

I finally wrote a README file.

## Making imoria:

Edit configure.h to set the paths you want the data files to go and
were the help file (mhelp.pl) will be.

Put a copy of monsters.dat into the DATA_FILE_PATH, and move mhelp.pl
to the propper place as well.

In the Makefile edit the owner and group to use for the game and data files.
run `make imoria`

the first time you run imoria it should make a bunch of data files and
then quit. Since the Makefile does not know where the data files are at this may
not work correctly for you (I do intend to make this better).


## Common problems:

Several people are having a problem running the game where it quits
right after starting, without any error messages.  Two problems have
been found that cause this.  The first is not having copied monsters.dat
to the data files directory that is set in configure.h.  The second
is if ncurses does not like the default terminal type.

