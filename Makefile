CC =		gcc

CFLAGS =	-Wall -Wextra -Werror -pedantic -Werror=implicit-function-declaration -std=gnu99 -g3 -DDO_DEBUG=1
LDFLAGS =	-lncurses -ltinfo -ltermcap -lm -Wl,--gc-sections -lpthread -ldl

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
	$(CC) $(OBJFILES) target/debug/libomoria.a $(LDFLAGS) -o $@
.PHONY: omoria

run: omoria ctags
	>debug2.out
	RUST_BACKTRACE=1 ./omoria

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
