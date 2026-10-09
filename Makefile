CC =		gcc

CFLAGS =	-Wall -Wextra -Wno-format -Wno-incompatible-pointer-types -Werror=implicit-function-declaration -std=gnu99 -g3 -DDO_DEBUG=1 -MMD -MP
ifeq ($(shell uname -s),Linux)
LDFLAGS =	-lncursesw -lm -lpthread -ldl
else
LDFLAGS =	-lncurses -ltermcap -lm -lpthread -ldl
endif

READFILES =	data/hours.dat data/monsters.dat data/moria_gcustom.mst
WRITEFILES = data/death.log data/moriamas.dat data/moriatop.dat data/moriatrd.dat
DATAFILES =	$(READFILES) $(WRITEFILES)

RSFILES = $(shell find src/ -type f -name '*.rs')
CFILES = $(shell find src/ -type f -name '*.c')
HFILES = $(shell find src/ -type f -name '*.h')
OBJFILES = $(addsuffix .o, $(basename $(CFILES)))
DEPFILES = $(OBJFILES:.o=.d)

.DEFAULT_GOAL := all
-include $(DEPFILES)

.PHONY: all
all:	omoria

.c.o:
	$(CC) $(CFLAGS) -c -o $*.o $*.c

omoria: $(OBJFILES) $(RSFILES) Cargo.toml Cargo.lock
	cargo build
	$(CC) $(OBJFILES) target/debug/libomoria.a $(LDFLAGS) -o $@

.PHONY: run
run: omoria
	>debug_rust.out
	RUST_BACKTRACE=1 ./omoria

.PHONY: pre-commit
pre-commit:
	rustfmt $$(git diff --cached --name-only | grep ".*\.rs$$")
	#clang-format -i $$(git diff --cached --name-only | grep ".*\.[ch]$$")
	cargo test

.PHONY: test
test:
	cargo test

.PHONY: test-movement
test-movement: $(filter-out src/main.o,$(OBJFILES))
	cargo build
	$(CC) $(CFLAGS) tests/movement_ffi.c $(filter-out src/main.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/movement-ffi-test
	./target/debug/movement-ffi-test

.PHONY: test-messages
test-messages: $(filter-out src/main.o src/io.o,$(OBJFILES))
	cargo build
	$(CC) $(CFLAGS) -Dinkey=headless_inkey -Dput_buffer=headless_put_buffer -DErase_Line=headless_erase_line -c src/io.c -o target/debug/message-io.o
	$(CC) $(CFLAGS) -include tests/support/terminal_redirects.h -c src/screen.c -o target/debug/headless-screen.o
	$(CC) $(CFLAGS) -include tests/support/terminal_redirects.h -c src/dungeon/light.c -o target/debug/headless-light.o
	$(CC) $(CFLAGS) tests/message_ffi.c tests/support/terminal.c tests/support/terminal_test.c target/debug/message-io.o target/debug/headless-screen.o target/debug/headless-light.o $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/message-ffi-test
	./target/debug/message-ffi-test

.PHONY: test-headless
test-headless: $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/main_loop/main_loop.o src/main_loop/command.o,$(OBJFILES))
	cargo build
	$(CC) $(CFLAGS) -Dinkey=headless_inkey -Dput_buffer=headless_put_buffer -DErase_Line=headless_erase_line -c src/io.c -o target/debug/headless-io.o
	$(CC) $(CFLAGS) -include tests/support/terminal_redirects.h -c src/screen.c -o target/debug/headless-screen.o
	$(CC) $(CFLAGS) -include tests/support/terminal_redirects.h -c src/dungeon/light.c -o target/debug/headless-light.o
	$(CC) $(CFLAGS) -include tests/support/terminal_redirects.h -c src/main_loop/main_loop.c -o target/debug/headless-loop.o
	$(CC) $(CFLAGS) -include tests/support/terminal_redirects.h -c src/main_loop/command.c -o target/debug/headless-command.o
	$(CC) $(CFLAGS) tests/headless_turn.c tests/support/terminal.c target/debug/headless-io.o target/debug/headless-screen.o target/debug/headless-light.o target/debug/headless-loop.o target/debug/headless-command.o $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/main_loop/main_loop.o src/main_loop/command.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/headless-turn-test
	./target/debug/headless-turn-test

.PHONY: test-save
test-save: $(filter-out src/main.o,$(OBJFILES))
	cargo build --features save-test-support
	$(CC) $(CFLAGS) tests/save_apply_ffi.c $(filter-out src/main.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/save-apply-ffi-test
	./target/debug/save-apply-ffi-test

.PHONY: check
check:
	cargo fmt --check
	cargo clippy --all-targets
	cargo test
	$(MAKE) omoria
	$(MAKE) test-movement
	$(MAKE) test-messages
	$(MAKE) test-save
	$(MAKE) test-headless

.PHONY: debug
debug: omoria
	>debug_rust.out
	RUST_BACKTRACE=1 rust-gdb ./omoria

.PHONY: nodata
nodata ::
	$(RM) data/hours.dat data/death.log data/moriamas.dat data/moriatop.dat data/moriatrd.dat data/moria_gcustom.mst data/TRADE.DUMP

.PHONY: clean
clean ::
	$(RM) $(OBJFILES) $(DEPFILES) core omoria

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
