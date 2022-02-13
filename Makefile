CC =		gcc

CFLAGS =	-Wall -Wextra -Werror -pedantic -Werror=implicit-function-declaration -std=gnu99 -g3 -DDO_DEBUG=1
LDFLAGS =	-lncurses -ltinfo -ltermcap -lm -Wl,--gc-sections -lpthread -ldl

READFILES =	data/hours.dat data/monsters.dat data/moria_gcustom.mst
WRITEFILES =	data/death.log data/moriamas.dat data/moriatop.dat data/moriatrd.dat
DATAFILES =	$(READFILES) $(WRITEFILES)

all:	omoria ctags

RSFILES = $(wildcard src/*.rs) $(wildcard src/*/*.rs)
CFILES = $(wildcard src/*.c) $(wildcard src/*/*.c)
HFILES = $(wildcard src/*.h) $(wildcard src/*/*.h)
OBJFILES = $(addsuffix .o, $(basename $(CFILES)))

.c.o:
	$(CC) $(CFLAGS) -c -o $*.o $*.c

omoria: $(OBJFILES) $(RSFILES)
	cargo build
	$(CC) $(OBJFILES) target/debug/libomoria.a $(LDFLAGS) -o $@

.PHONY: run
run: omoria ctags
	>debug2.out
	RUST_BACKTRACE=1 ./omoria

.PHONY: debug
debug: omoria ctags
	>debug2.out
	RUST_BACKTRACE=1 rust-gdb ./omoria

.PHONY: nodata
nodata ::
	$(RM) data/hours.dat data/death.log data/moriamas.dat data/moriatop.dat data/moriatrd.dat data/moria_gcustom.mst data/TRADE.DUMP

.PHONY: clean
clean ::
	$(RM) $(OBJFILES) core omoria

.PHONY: ctags
ctags:
	@ctags -R . --exclude .git
	@rusty-tags vi

.PHONY: format
format:
	@clang-format -i $(CFILES) $(HFILES)

.PHONY: spotless
spotless : nodata clean
