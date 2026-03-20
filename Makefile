CC =		gcc

CFLAGS =	-Wall -Wextra -Wno-format -Wno-incompatible-pointer-types -Werror=implicit-function-declaration -std=gnu99 -g3 -DDO_DEBUG=1
LDFLAGS =	-lncurses -ltermcap -lm -lpthread -ldl

READFILES =	data/hours.dat data/monsters.dat data/moria_gcustom.mst
WRITEFILES = data/death.log data/moriamas.dat data/moriatop.dat data/moriatrd.dat
DATAFILES =	$(READFILES) $(WRITEFILES)

RSFILES = $(shell find src/ -type f -name '*.rs')
CFILES = $(shell find src/ -type f -name '*.c')
HFILES = $(shell find src/ -type f -name '*.h')
OBJFILES = $(addsuffix .o, $(basename $(CFILES)))

.PHONY: all
all:	omoria

.c.o:
	$(CC) $(CFLAGS) -c -o $*.o $*.c

.PHONY: clean
clean ::
	$(RM) $(OBJFILES) core omoria

.PHONY: ctags
ctags:
	@ctags -R . --exclude .git
	@rusty-tags vi

.PHONY: debug
debug: omoria
	>debug_rust.out
	RUST_BACKTRACE=1 rust-gdb ./omoria

.PHONY: format
format:
	@rustfmt $(RSFILES)
	#@clang-format -i $(CFILES) $(HFILES)

.PHONY: format-changed
format-changed:
	@rustfmt $$(git diff --cached --name-only | grep ".*\.rs$$")

.PHONY: nodata
nodata ::
	$(RM) data/hours.dat data/death.log data/moriamas.dat data/moriatop.dat data/moriatrd.dat data/moria_gcustom.mst data/TRADE.DUMP

omoria: $(OBJFILES) $(RSFILES)
	cargo build
	$(CC) $(OBJFILES) target/debug/libomoria.a $(LDFLAGS) -o $@

.PHONY: pre-commit
pre-commit:
	@rustfmt --check $$(git diff --cached --name-status | awk '!/^D/{print $$2}' | grep ".*\.rs$$")
	@#clang-format -i $$(git diff --cached --name-only | grep ".*\.[ch]$$")
	@cargo test --quiet

.PHONY: prepare
prepare:
	./scripts/install-git-hooks.sh

.PHONY: run
run: omoria
	>debug_rust.out
	RUST_BACKTRACE=1 ./omoria

.PHONY: test
test:
	cargo test