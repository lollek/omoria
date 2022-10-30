CC =		gcc

CFLAGS =	-Wall -Wextra -Werror -pedantic -Wno-stringop-overflow -Wno-format-truncation -Werror=implicit-function-declaration -std=gnu99 -g3 -DDO_DEBUG=1
LDFLAGS =	-lncurses -ltinfo -ltermcap -lm -Wl,--gc-sections -lpthread -ldl

READFILES =	data/hours.dat data/monsters.dat data/moria_gcustom.mst
WRITEFILES =	data/death.log data/moriamas.dat data/moriatop.dat data/moriatrd.dat
DATAFILES =	$(READFILES) $(WRITEFILES)

RSFILES = $(shell find src/ -type f -name '*.rs')
CFILES = $(shell find src/ -type f -name '*.c')
HFILES = $(shell find src/ -type f -name '*.h')
OBJFILES = $(shell find src/ -type f -name '*.o')

.PHONY: all
all:	omoria

.c.o:
	$(CC) $(CFLAGS) -c -o $*.o $*.c

omoria: $(OBJFILES) $(RSFILES)
	cargo build
	$(CC) $(OBJFILES) target/debug/libomoria.a $(LDFLAGS) -o $@

.PHONY: run
run: omoria
	>debug_rust.out
	RUST_BACKTRACE=1 ./omoria

.PHONY: test
test: 
	cargo test

.PHONY: debug
debug: omoria
	>debug_rust.out
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
	@rustfmt $(RSFILES)
	#@clang-format -i $(CFILES) $(HFILES)

.PHONY: spotless
spotless : nodata clean
