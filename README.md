# Imoria
A fork of Imoria with some fixes and changes, making it more fun to play for me.
This project has also involved into a way for me to try out the Rust language,
so I am in the process of porting it from C to Rust. So basically this project
is a port of a port (Pascal -> C -> Rust). I wouldn't recommend anyone else
compiling this project and running themselves, as there are probably a lot more
bugs you would like.

## Requirements
* Modern GCC (Game won't run with other compilers)
* Rustc (v1.61)
* Curses
* Termcap
* Pthreads

## Getting started
If you have installed the requirements above, you should only be required to
`make run` to compile and start the project.

# Original Imoria README
(There may be some outdated information here, because of changes I have
introduced in this repo)

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

