CC =		gcc

CFLAGS =	-Wall -Wextra -Wno-format -Wno-incompatible-pointer-types -Werror=implicit-function-declaration -std=gnu99 -g3 -DDO_DEBUG=1 -MMD -MP
ifeq ($(shell uname -s),Linux)
LDFLAGS =	-lncursesw -lm -lpthread -ldl
else
LDFLAGS =	-lncurses -ltermcap -lm -lpthread -ldl
endif

# Linking with -g makes clang run dsymutil on macOS, which is slow; harness links drop it.
LINKCFLAGS =	$(filter-out -g3,$(CFLAGS))

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

omoria: $(OBJFILES) $(RSFILES) build.rs Cargo.toml Cargo.lock
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

HDIR =	target/debug
HREDIRECT =	-include tests/support/terminal_redirects.h
HIODEFS =	-Dinkey=headless_inkey -Dput_buffer=headless_put_buffer -DErase_Line=headless_erase_line
HIODEFS +=	-Dput_buffer_attr=headless_put_buffer_attr
HLOOPOBJS =	$(HDIR)/headless-loop.o $(HDIR)/headless-command.o

$(HDIR):
	mkdir -p $@

$(HDIR)/headless-io.o: src/io.c $(HFILES) | $(HDIR)
	$(CC) $(CFLAGS) $(HIODEFS) -c src/io.c -o $@

$(HDIR)/headless-io-prt.o: src/io.c $(HFILES) | $(HDIR)
	$(CC) $(CFLAGS) $(HIODEFS) -Dprt=headless_prt -c src/io.c -o $@

$(HDIR)/headless-screen.o: src/screen.c tests/support/terminal_redirects.h $(HFILES) | $(HDIR)
	$(CC) $(CFLAGS) $(HREDIRECT) -c src/screen.c -o $@

$(HDIR)/headless-light.o: src/dungeon/light.c tests/support/terminal_redirects.h $(HFILES) | $(HDIR)
	$(CC) $(CFLAGS) $(HREDIRECT) -c src/dungeon/light.c -o $@

$(HDIR)/headless-player.o: src/player.c tests/support/terminal_redirects.h $(HFILES) | $(HDIR)
	$(CC) $(CFLAGS) $(HREDIRECT) -c src/player.c -o $@

$(HDIR)/headless-inven.o: src/inventory/inven.c tests/support/terminal_redirects.h $(HFILES) | $(HDIR)
	$(CC) $(CFLAGS) $(HREDIRECT) -c src/inventory/inven.c -o $@

$(HDIR)/headless-loop.o: src/main_loop/main_loop.c tests/support/terminal_redirects.h $(HFILES) | $(HDIR)
	$(CC) $(CFLAGS) $(HREDIRECT) -c src/main_loop/main_loop.c -o $@

$(HDIR)/headless-command.o: src/main_loop/command.c tests/support/terminal_redirects.h $(HFILES) | $(HDIR)
	$(CC) $(CFLAGS) $(HREDIRECT) -c src/main_loop/command.c -o $@

$(HDIR)/headless-command-turn.o: src/main_loop/command.c tests/support/terminal_redirects.h $(HFILES) | $(HDIR)
	$(CC) $(CFLAGS) $(HREDIRECT) -Dshow_recorded_messages=headless_show_recorded_messages -DC_MOUSE_TRAVEL_DELAY=headless_travel_delay -c src/main_loop/command.c -o $@

$(HDIR)/headless-rest.o: src/player_action/rest.c tests/support/terminal_redirects.h $(HFILES) | $(HDIR)
	$(CC) $(CFLAGS) $(HREDIRECT) -Dheadless_get_string=headless_rest_get_string -c src/player_action/rest.c -o $@

.PHONY: test-movement
test-movement: $(filter-out src/main.o,$(OBJFILES)) $(HDIR)/headless-light.o
	cargo build
	$(CC) $(LINKCFLAGS) tests/movement_ffi.c tests/support/terminal.c $(HDIR)/headless-light.o $(filter-out src/main.o src/dungeon/light.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/movement-ffi-test
	./target/debug/movement-ffi-test

.PHONY: test-mouse-input
test-mouse-input: $(filter-out src/main.o,$(OBJFILES))
	cargo build
	$(CC) $(LINKCFLAGS) tests/mouse_input.c $(filter-out src/main.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/mouse-input-test
	./target/debug/mouse-input-test

.PHONY: test-messages
test-messages: $(filter-out src/main.o src/io.o,$(OBJFILES)) $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o
	cargo build --features terminal-test-support
	$(CC) $(LINKCFLAGS) tests/message_ffi.c tests/support/terminal.c tests/support/terminal_test.c $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/message-ffi-test
	./target/debug/message-ffi-test

.PHONY: test-effects
test-effects: $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/player.o,$(OBJFILES)) $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HDIR)/headless-player.o
	cargo build
	$(CC) $(LINKCFLAGS) tests/effects_ffi.c tests/support/terminal.c $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HDIR)/headless-player.o $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/player.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/effects-ffi-test
	./target/debug/effects-ffi-test

.PHONY: test-hunger
test-hunger: $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/player.o,$(OBJFILES)) $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HDIR)/headless-player.o
	cargo build
	$(CC) $(LINKCFLAGS) tests/hunger_ffi.c tests/support/terminal.c $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HDIR)/headless-player.o $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/player.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/hunger-ffi-test
	./target/debug/hunger-ffi-test

.PHONY: test-quaff
test-quaff: $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/player.o src/inventory/inven.o,$(OBJFILES)) $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HDIR)/headless-player.o $(HDIR)/headless-inven.o
	cargo build
	$(CC) $(LINKCFLAGS) tests/quaff_ffi.c tests/support/terminal.c $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HDIR)/headless-player.o $(HDIR)/headless-inven.o $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/player.o src/inventory/inven.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/quaff-ffi-test
	./target/debug/quaff-ffi-test

.PHONY: test-eat
test-eat: $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/player.o src/inventory/inven.o,$(OBJFILES)) $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HDIR)/headless-player.o $(HDIR)/headless-inven.o
	cargo build
	$(CC) $(LINKCFLAGS) tests/eat_ffi.c tests/support/terminal.c $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HDIR)/headless-player.o $(HDIR)/headless-inven.o $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/player.o src/inventory/inven.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/eat-ffi-test
	./target/debug/eat-ffi-test

.PHONY: test-headless
test-headless: $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/main_loop/main_loop.o src/main_loop/command.o,$(OBJFILES)) $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HDIR)/headless-loop.o $(HDIR)/headless-command-turn.o
	cargo build --features save-test-support
	$(CC) $(LINKCFLAGS) tests/headless_turn.c tests/support/headless_scenario.c tests/support/terminal.c $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HDIR)/headless-loop.o $(HDIR)/headless-command-turn.o $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/main_loop/main_loop.o src/main_loop/command.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/headless-turn-test
	./target/debug/headless-turn-test

.PHONY: test-headless-interaction
test-headless-interaction: $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/main_loop/main_loop.o src/main_loop/command.o,$(OBJFILES)) $(HDIR)/headless-io-prt.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HLOOPOBJS) $(HDIR)/headless-rest.o
	cargo build --features save-test-support
	$(CC) $(LINKCFLAGS) tests/headless_interaction.c tests/support/terminal.c $(HDIR)/headless-io-prt.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HLOOPOBJS) $(HDIR)/headless-rest.o $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/main_loop/main_loop.o src/main_loop/command.o src/player_action/rest.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/headless-interaction-test
	./target/debug/headless-interaction-test

.PHONY: test-headless-persistence
test-headless-persistence: $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/main_loop/main_loop.o src/main_loop/command.o,$(OBJFILES)) $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HLOOPOBJS)
	cargo build --features save-test-support
	$(CC) $(LINKCFLAGS) tests/headless_persistence.c tests/support/terminal.c $(HDIR)/headless-io.o $(HDIR)/headless-screen.o $(HDIR)/headless-light.o $(HLOOPOBJS) $(filter-out src/main.o src/io.o src/screen.o src/dungeon/light.o src/main_loop/main_loop.o src/main_loop/command.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/headless-persistence-test
	./target/debug/headless-persistence-test

.PHONY: test-save
test-save: $(filter-out src/main.o,$(OBJFILES))
	cargo build --features save-test-support
	$(CC) $(LINKCFLAGS) tests/save_apply_ffi.c $(filter-out src/main.o,$(OBJFILES)) target/debug/libomoria.a $(LDFLAGS) -o target/debug/save-apply-ffi-test
	./target/debug/save-apply-ffi-test

# Set to 0 only when characterizing the legacy C dispatcher (unchecked
# pointers). The normal check gate covers migrated effects and null rejection.
TRAP_TEST_MIGRATED ?= 1
TRAP_WRAPS = search_off rest_off change_trap lite_spot damroll take_hit lose_stat \
	check_store_hours_and_enter \
	check_store_hours enter_trading_post enter_bank enter_casino enter_fortress \
	enter_house pusht place_random_dungeon_item summon_land_monster \
	summon_water_monster

.PHONY: test-traps
ifeq ($(shell uname -s),Linux)
test-traps: $(filter-out src/main.o,$(OBJFILES))
	cargo build
	$(CC) $(LINKCFLAGS) -DTRAP_TEST_GNU_WRAP=1 -DTRAP_TEST_MIGRATED=$(TRAP_TEST_MIGRATED) tests/trap_dispatch_ffi.c $(filter-out src/main.o,$(OBJFILES)) target/debug/libomoria.a $(foreach symbol,$(TRAP_WRAPS),-Wl,--wrap=$(symbol)) $(LDFLAGS) -o target/debug/trap-dispatch-ffi-test
	./target/debug/trap-dispatch-ffi-test
else
test-traps:
	@echo "SKIP test-traps: C caller doubles require Linux GNU ld --wrap; Rust tests and mixed build remain enabled."
endif

.PHONY: check
check:
	cargo fmt --check
	cargo clippy --all-targets
	cargo test
	$(MAKE) omoria
	$(MAKE) test-movement
	$(MAKE) test-mouse-input
	$(MAKE) test-messages
	$(MAKE) test-effects
	$(MAKE) test-hunger
	$(MAKE) test-quaff
	$(MAKE) test-eat
	$(MAKE) test-save
	$(MAKE) test-traps
	$(MAKE) test-headless
	$(MAKE) test-headless-interaction
	$(MAKE) test-headless-persistence


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
