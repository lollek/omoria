###############################################################################
# Makefile for omoria
#
# A hopefuly helpful list of things that probably need to be done:
#
# You should fill in the correct paths and stuff in configure.h
# Put a copy of monsters.dat into the DATA_FILE_PATH
# Then pick a uid/gid for the game
# make omoria
# run omoria, and it should create a bunch of files and quit
# make privs
#
###############################################################################
CC =		gcc

CFLAGS =	-Wall -Wextra -pedantic -Werror=implicit-function-declaration -std=gnu89 -g3 -DDO_DEBUG=1
LDFLAGS =	-lncurses -ltermcap -lm -lgdbm -Wl,--gc-sections -lpthread -ldl

#
# the owner and group for the game and data files
#
#OWNER =	games
OWNER =	iix
#GROUP =	games
GROUP =	iix

#
# privs needed on each file depends on if it is for read or write
#
READFILES =	data/hours.dat data/moria.dat data/monsters.dat data/moria_gcustom.mst
WRITEFILES =	data/death.log data/moriamas.dat data/moriatop.dat data/moriatrd.dat
DATAFILES =	$(READFILES) $(WRITEFILES)

all:	omoria ctags

CFILES = $(wildcard src/*.c)
HFILES = $(wildcard src/*.h)
OBJFILES = $(addsuffix .o, $(basename $(CFILES)))

.c.o:
	$(CC) $(CFLAGS) -c -o $*.o $*.c

omoria: $(OBJFILES)
	cargo build
	$(CC) $(LDFLAGS) $(OBJFILES) -o $@ target/debug/libomoria.a
.PHONY: omoria

run: omoria ctags
	>debug2.out
	RUST_BACKTRACE=1 ./omoria

privs ::
	chown $(OWNER):$(GROUP) omoria $(DATAFILES)
	chmod 2711        omoria
	chmod 640         $(READFILES)
	chmod 660         $(WRITEFILES)
	chmod 755         data/mhelp.pl

nodata ::
	$(RM) data/hours.dat data/moria.dat data/death.log data/moriamas.dat data/moriatop.dat data/moriatrd.dat data/moria_gcustom.mst data/TRADE.DUMP

clean ::
	$(RM) $(OBJFILES) core omoria
.PHONY: clean

ctags:
	@ctags -R . --exclude .git
.PHONY: ctags

format:
	@clang-format -i $(CFILES) $(HFILES)
.PHONY: format

spotless : nodata clean
.PHONY: spotless
