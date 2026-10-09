# Headless-Turn Seam Inventory (SEAM1)

> **Historical.** This is the pre-implementation research. The headless
> harnesses have since landed (`make test-headless*`; see the
> [testing guide](../testing.md)). **R**/**A** markers and
> "not executed" claims reflect the state when it was written.

Research only. No headless harness or production seam is implemented by this
document. **R** means verified by source reading; **A** means an assumption
requiring an executable harness check. Line numbers refer to this checkout.

## Bounded Scope

Inventory the setup and one iteration of
[main_loop__0](../../src/main_loop/main_loop.c#L821), movement through
[command](../../src/main_loop/command.c#L36), and one inventory interaction.
Do not call the outer `main_loop`: it handles death and generates another map.

**R:** A quiet dungeon still calls `prt_equipment`, `prt_stat_block`,
`print_null`, and `inkey`, and always rolls `randint(MAX_MALLOC_CHANCE)`.
State setup alone therefore cannot establish headless execution.

**A:** A bounded command source, terminal doubles, and a controlled C-facing
RNG should suffice for the restricted scenario. This is not runtime evidence.

The inventory covers all external-effect call sites in the loop and the
selected movement/pickup path. Conditional branches are identified below,
but their recursive combat, spell, store, trap, and map-generation subtrees
are not a supported headless scenario. **A:** any expansion into those
subtrees needs another inventory and executable checks.

## Minimal Setup

These are proposed fixture constraints, not an executed fixture. Their
branch effects are **R**; their combined sufficiency is **A**.

* Use a valid interior position, dry open floor in `earth_set`, no monsters
	(`muptr=0`), and no object on the starting cell. Provide valid panel bounds,
	dimensions, and a safe surrounding map region, not the 3-by-3 wall-only
	fixture from the existing movement harness.
* Use `dun_level=1`, `turn=1`, speed zero, healthy full HP and mana, and
	`foodc=10000`. Zero timed effects, resting, paralysis, teleportation,
	hunger-item, regeneration, and search/run state. Disable rage and rage
	exhaustion in Rust player state as well as clearing C flags.
* Keep `blind` greater than the entire bounded run's tick count plus one and
	pre-set `IS_BLIND`. This avoids blindness onset/expiry messages, the search
	RNG, and room lighting, including during the startup stationary move.
	Blindness still decrements and still draws player symbols.
* Light fuel zero and `light_on=false` avoid light-status transitions. Use
	`wizard1=true` solely to bypass operating-hours/file checks; do not dispatch
	wizard commands. Alternatively keep it false and double the environmental
	kickout calls in the harness build.
* Begin message capture before loop entry, reset message/history globals,
	install HT1 doubles, and seed RNG1. **A:** choose a seed whose allocation
	roll is not 1 for every intended tick; verify the resulting `muptr` remains
	zero. Empty monsters at entry do not prevent allocation later.

## Loop and Terminal Inventory

Every row is **R** unless explicitly marked **A**. "Yes" means reached with
the minimal setup above, including startup; "No" states the controlling
condition that avoids it. Proposed seams are recommendations, not implemented.

| Source and call | Minimal reachability | Proposed seam |
| --- | --- | --- |
| [main_loop.c](../../src/main_loop/main_loop.c#L39): coordinate `randint` loop | No: both coordinates supplied, neither is -1. | Explicit coordinates; RNG1 if testing placement. |
| [main_loop.c](../../src/main_loop/main_loop.c#L821): `player_action_move(5)`, `creatures(false)` | Yes, before the first tick. `cave_flag=false` forces initial `prt_map`. | Keep real movement and creature code; intercept drawing, not occupancy or lighting mutations. |
| [main_loop.c](../../src/main_loop/main_loop.c#L55): town `store_maint`, `draw_cave` | No: dungeon level nonzero. Otherwise sunrise/sunset at hours 6/18 and seconds 0. | Avoid town; a store harness would also need controlled maintenance RNG. |
| [main_loop.c](../../src/main_loop/main_loop.c#L859): `water_move`, `adv_time(true)` | Yes at speed zero; empty monster traversal has no effect, game age advances. | Keep game clock. [misc.c](../../src/misc.c#L401) uses no wall clock; it calls `prt_stat_block` on every 100th game second. |
| [main_loop.c](../../src/main_loop/main_loop.c#L870): `kick__kickout_player_if_time`; [hour check](../../src/main_loop/main_loop.c#L84): `kick__should_kickout` | No in the first proposed tick (turn becomes 2); scheduled at turn modulo 10/100 equal to 1. Wizard setup bypasses environmental work even then. | State avoidance, or `-D` redirects of both kickout calls in the harness loop object. |
| [main_loop.c](../../src/main_loop/main_loop.c#L876): `randint(MAX_MALLOC_CHANCE)`, `generate_land_monster` | RNG always; allocation only on 1. | RNG1; assert no allocation for the chosen seed. Do not double the entire creature turn. |
| [main_loop.c](../../src/main_loop/main_loop.c#L102): `prt_stat_block` when `print_stat!=0`; [tick](../../src/main_loop/main_loop.c#L882): `prt_equipment` | Equipment always; updated stats conditional. | Preserve C screen logic with redirected terminal leaves, including Rust HUD entry calls. |
| [main_loop.c](../../src/main_loop/main_loop.c#L108): light `randint(5)`, messages, `dungeon_light_move` | No: no light fuel and light off. Faint fuel, burnout, or activation reach these. | Avoid light changes; RNG1 plus drawing doubles for later light scenarios. |
| [main_loop.c](../../src/main_loop/main_loop.c#L188): blindness `prt_map`, `prt_blind`, `search_off`, `player_action_move(5)` | No transitions: status already blind and duration cannot expire. | State setup; rendering doubles still cover `prt_blind` from stat blocks. |
| [main_loop.c](../../src/main_loop/main_loop.c#L210): confusion `prt_confused`, messages, stationary movement | No: confusion zero. | State avoidance; preserve real transitions if later tested. |
| [main_loop.c](../../src/main_loop/main_loop.c#L243): fire/frost/blade `explode`, `randint` | No: all rings zero. | State avoidance; expanded explosion scenario needs its own inventory. |
| [main_loop.c](../../src/main_loop/main_loop.c#L293): stealth/charm/hoarse transitions | No: timers zero; messages and charm's stationary move otherwise. | State avoidance; message capture and drawing doubles if enabled. |
| [main_loop.c](../../src/main_loop/main_loop.c#L334): fear `prt_afraid`, messages, stationary movement | No: fear zero. | State avoidance or existing drawing/capture seams. |
| [main_loop.c](../../src/main_loop/main_loop.c#L364): poison `prt_poisoned`, `take_hit`, messages | No: poison zero. | State avoidance; damage path requires `flush` interception and death assertions. |
| [main_loop.c](../../src/main_loop/main_loop.c#L417): fast/slow `change_speed`, messages, stationary movement | No: both timers zero. | Avoid effects that change movement/monster scheduling. |
| [main_loop.c](../../src/main_loop/main_loop.c#L499): rest `usleep(500)`, soul-sword `randint(10)`/`bother`, `inkey_delay`, `rest_off` | No: rest zero. | For later rest coverage, double delay/input; do not use real sleeps. Soul sword aggravates monsters. |
| [io.c](../../src/io.c#L137): `inkey_delay` | No without rest. Current implementation calls `refresh`, then writes zero; it does not read a key. | Scripted nonblocking poll with an explicit no-key result, separately from command input. |
| [main_loop.c](../../src/main_loop/main_loop.c#L539): hallucination `draw_cave`; petrify `rest_off`/`search_off` | No: image/paralysis zero. | State avoidance; redraw must cover `refresh` and `C_clear_screen` if enabled. |
| [main_loop.c](../../src/main_loop/main_loop.c#L600): invulnerability/hero/superhero/blessed `prt_stat_block`, messages, stationary movement | No: timers zero. | Avoid transitions; otherwise drawing/capture doubles, with HP assertions for heroism. |
| [main_loop.c](../../src/main_loop/main_loop.c#L719): detect-invisible expiry `py_bonuses`; infravision message; recall changes level | No: timers zero. | Avoid recalculation and level exit; do not treat recall as a bounded-loop stop mechanism. |
| [main_loop.c](../../src/main_loop/main_loop.c#L771): `prt_stat_block` | Yes on every ordinary non-running tick, even with unchanged HP/mana. | Required rendering doubles. |
| [main_loop.c](../../src/main_loop/main_loop.c#L919), [ability.rs](../../src/flow/ability.rs#L56): passive rage messages/stat recalculation | Called, but no transitions with rage and exhaustion disabled. | Set Rust player state, not only C flags. |
| [main_loop.c](../../src/main_loop/main_loop.c#L936): random teleport `randint(100)`/`teleport(40)`; [post-command](../../src/main_loop/main_loop.c#L958): `teleport(100)` | No: teleport ability/flag false. | Avoid teleport; later coverage needs RNG and destination/light effects. |
| [main_loop.c](../../src/main_loop/main_loop.c#L943): `print_null`, `inkey`, `erase_line` | Cursor and input yes; erasure when prior `msg_flag` true, even under capture. | `print_null` is a macro to `move_cursor` in [term.h](../../src/term.h#L17); redirect `move_cursor`, `inkey`, and `Erase_Line`. |
| [main_loop.c](../../src/main_loop/main_loop.c#L964), [creature.c](../../src/creature.c#L1718): `creatures(true)` | Called; empty list returns after `get_player_move_rate`. | Keep production call. [creature.c](../../src/creature.c#L44) still rolls `randint(5)` on non-earth terrain even with no monsters; dry floor avoids it. |
| [main_loop.c](../../src/main_loop/main_loop.c#L970): final `search_off` | No: search disabled. | Preserve cleanup when HL1 adds return paths; rendering doubles if search supported. |

The other counter updates (resist lightning, monster protection, magic
protection, petrification resistance, evil protection, heat/cold resistance)
only adjust explicit globals in this loop; zero timers avoid their changes
([main_loop.c](../../src/main_loop/main_loop.c#L229), **R**).

### Rendering Leaves HT1 Must Cover

These source paths are **R**. The proposed macro/link configuration is **A**
until compiled and executed. Do not replace gameplay-bearing movement or
lighting routines with no-ops.

| Source | Reachability and seam |
| --- | --- |
| [screen.c](../../src/screen.c#L57): `prt_equipment` -> `C_print_equipment_block`; [stats](../../src/screen.c#L164): `prt_stat_block` -> `C_print_stat_block` | Both reached. Redirect the two C-to-Rust HUD calls in a harness-specific screen object. [heads_up_display.rs](../../src/user_interface/heads_up_display.rs#L33) directly uses Rust ncurses and is not intercepted by a C `put_buffer` double. |
| [screen.c](../../src/screen.c#L189): all status `prt_*` functions | Reached through stat blocks: hunger, blind, confused, afraid, poisoned, search, rest, quest, light. Redirect `put_buffer`, `put_buffer_attr`, and `prt` calls in the screen object. Attribute printing otherwise calls curses in [term.c](../../src/term.c#L39). |
| [screen.c](../../src/screen.c#L91): `prt_map` -> `erase_line`, `print_chstr` | Startup reaches map rendering; dirty-line erasure depends on `used_line`. Redirect `Erase_Line` and `print_chstr`, preserving `redraw`/`used_line` updates in `prt_map`. |
| [light.c](../../src/dungeon/light.c#L197): blind light move -> `print(' ')`, `print('@')` | Reached at startup and successful move. Redirect `Print`, the macro target in [io.h](../../src/io.h#L18). This branch may clear temporary light and static `light_flag`. |
| [light.c](../../src/dungeon/light.c#L17): normal/run `print_chstr`; [unlit](../../src/dungeon/light.c#L236): `lite_spot`, `unlite_spot`, `print`; [room](../../src/dungeon/light.c#L289): `print_chstr` | Not used by the blind setup. Redirect rendering leaves for later sighted movement; hallucination adds `randint(12)` and `randint(95)` at line 66. |
| [io.c](../../src/io.c#L282): `print_str`, `print_chstr`; [term.c](../../src/term.c#L115): `Print`; [cursor](../../src/term.c#L139): `move_cursor` | Map/string rendering otherwise reaches `mvaddchstr`/`mvaddch`/`move`. Redirect callers of these helpers; they are not all covered by `put_buffer`. |
| [term_extern.rs](../../src/term_extern.rs#L12): `prt`/`prt_`, `put_buffer`/`put_buffer_`; [term.rs](../../src/term.rs#L97): line clearing | Real implementations reach Rust ncurses. Redirect C callers, not just libcurses names. If retained, `prt` may emit an empty message on the message row; do not silently change expected capture semantics. |
| [screen.c](../../src/screen.c#L80): `draw_cave` -> `C_clear_screen`, stats/map/search/equipment, `refresh` | Not reached under minimal state, but reached by redraw and effect/town branches. Avoid those commands; fail loudly on unexpected redraw or provide explicit harness-only screen redirects. |

Reuse the caller-side macro technique from
[test-messages](../../Makefile#L64) and
[message_ffi.c](../../tests/message_ffi.c#L17). **A:** compile isolated harness
objects for the loop, screen, light, and message caller redirects as needed,
excluding their normal objects from the harness link. Do not merely add a
second definition of a symbol already exported by C or the Rust static library.
Apply a macro only where it redirects a call, not where it renames the real
definition into the double's symbol. `erase_line`, `print`, and `print_null`
are already macros: redirect their targets, not the macros themselves.

## Input, Dispatch, and Movement

All dispatch and branch statements here are **R**.

| Source and command | Reachability and seam |
| --- | --- |
| [command.c](../../src/main_loop/command.c#L370): `b,h,j,k,l,n,u,y` | Directions 1,4,2,8,6,3,9,7 respectively; no direct prompt. Use lower-case `l` (direction 6) for pickup. Numeric keypad characters are not switch cases here; inject dispatch characters, not `'6'`. |
| [command.c](../../src/main_loop/command.c#L78): CTRL-B/H/J/K/L/N/U/Y; [upper-case](../../src/main_loop/command.c#L274): B/H/J/K/L/N/U/Y | Set `find_flag=true`, then move; running reuses `com_val` without `inkey`. Avoid run for HL1-HL3; a command-source limit alone cannot bound it. |
| [command.c](../../src/main_loop/command.c#L146): ALT movement | Reads an additional `inkey` before dispatching the lower-case direction. Avoid ALT, or explicitly budget a second scripted key. |
| [command.c](../../src/main_loop/command.c#L265): `.` | Stationary move plus `usleep(10)` and `flush` -> curses `refresh` in [term.c](../../src/term.c#L96). Avoid; do not assume it is a terminal-free wait command. |
| [command.c](../../src/main_loop/command.c#L286): `E`; [drop](../../src/main_loop/command.c#L376): `d`; [quaff](../../src/main_loop/command.c#L423): `q` | Not selected. Eating reaches `get_item` in [eat.c](../../src/player_action/eat.c#L54); dropping reaches it in [drop.c](../../src/player_action/drop.c#L35). They need selection input/rendering; pickup avoids both. |
| [command.c](../../src/main_loop/command.c#L466): unknown command | Sets `reset_flag=true` and calls `prt`; repeats input within the same game tick. Strict command allowlist and fail-on-exhaustion. |
| [move.c](../../src/player_action/move.c#L305): `C_player_move_direction`, `C_player_move_resolve` | Always. [step.rs](../../src/player_action/movement/step.rs#L137) constructs `thread_rng` directly; confused directions draw through `randint_with_rng`, not the C-facing RNG wrapper. Keep confusion zero; RNG1 alone does not prove confusion determinism. |
| [move.c](../../src/player_action/move.c#L318): monster/blocked/out-of-bounds branches | No monster on target; blocked/edge commands set `reset_flag` and can retry indefinitely in the inner command loop. Door/rubble emit messages. Avoid attack; test blocked movement separately with finite scripted input. |
| [move.c](../../src/player_action/move.c#L370): `move_creature`, panel `prt_map`, run `area_affect`, search `randint(player_fos())`/`player_action_search` | Successful pickup reaches occupancy and panel checks; blind setup skips search and room lighting, no run skips area effects. Initial panel redraw cannot be avoided by pre-setting `cave_flag`, which entry resets. |
| [move.c](../../src/player_action/move.c#L384): `carry`, `dungeon_light_move`, room lighting | Pickup reaches carry and blind light drawing. Empty floor skips carry; `blind>0` skips room light. Keep real lighting/occupancy logic and double only drawing. |

**R:** [main_loop.c](../../src/main_loop/main_loop.c#L926) increments
`turn_counter` per command attempt, not per consumed turn; `turn` increments
once per outer iteration. Initialization also moves in place before either
script or tick. **A:** HL1 should check tick limits before the next tick's
updates, and propagate command-source exhaustion out of the inner retry loop.
Do not synthesize NUL, ESC, or `@` on exhaustion: these are gameplay commands,
not a safe return signal. Keep command input separate from prompt input so an
unexpected prompt cannot consume the next movement command. Preserve relevant
input effects such as clearing `msg_flag` in [term.c](../../src/term.c#L86).

## Wall Clock, Files, Exit, and Death

All source paths/conditions below are **R**; interception proposals need runtime
verification (**A**).

| Source | Minimal reachability | Proposed seam |
| --- | --- | --- |
| [kickout.c](../../src/kickout.c#L24): `time(NULL)`, `localtime`; [lock check](../../src/kickout.c#L62): `fopen(KICKOUT_FILE)`/`fclose` | No with wizard setup. Open hours alone do not avoid the file check. [configure.h](../../src/configure.h#L14) points to the real data directory. | Avoid via state, or redirect both loop-facing environmental checks. |
| [kickout.c](../../src/kickout.c#L38): messages, up to ten `sav__save_character` attempts, `exit_game` | No unless kickout is requested. | Never exercise against production persistence; fail loudly if reached in HL1-HL3. |
| [io.c](../../src/io.c#L129): `exit_game` -> terminal cleanup, stdout flush, `exit(0)` | No in successful scenario. | Harness-only noreturn fail-fast redirect at reachable call sites, never a returning stub. |
| [term.c](../../src/term.c#L56): `inkey` EOF -> save/death/exit | No with scripted source. Real input would call `refresh`/`getch` and may panic-save. | Do not invoke real terminal input; exhaustion returns through HL1's bounded API, not this EOF path. |
| [player.c](../../src/player.c#L121): `take_hit` -> `flush`, status redraw or death/moria flags | No: no poison, trap, or monster damage. | Drawing/flush double; assert no death for pickup, never use death to stop the loop. |
| [hunger.c](../../src/player/hunger.c#L34): hunger redraw/messages, starvation; fainting RNG | No: food safely above 2000 throughout the bounded run, hunger-item off. | Explicit food and weight; RNG1 for future starvation scenario. Food digestion still occurs. |
| [main_loop.c](../../src/main_loop/main_loop.c#L975): `upon_death(true)`/`generate_map` | Outside `main_loop__0`; never call outer loop for HL1. | Bounded inner entry, no outer death/map orchestration. |
| [death.c](../../src/death.c#L157): respawn prompt, master update, character deletion, tomb, exit | No without outer death handling. Tomb reaches `fopen(DEATH_FILE)` at [line 134](../../src/death.c#L134), wall-clock `show_current_time`, and `time`/`ctime` at [line 190](../../src/death.c#L190). | Reject death handling; later death coverage needs persistence, time, input, and terminal injection. |
| [command.c](../../src/main_loop/command.c#L68): NUL/CTRL-C/`@` -> `death_by_quitting` | No with movement allowlist. [death.c](../../src/death.c#L197) prompts, flushes, erases, refreshes, and may set death. | Reject these keys; do not use quit as harness teardown. |
| [command.c](../../src/main_loop/command.c#L206): ALT-s save/exit; [ALT-t](../../src/main_loop/command.c#L227): `show_current_time` | No with movement allowlist. [misc.c](../../src/misc.c#L1837) uses `time`/`localtime`; ALT-d uses game age instead. | Exclude real save/time commands until injected; HL4 owns persistence coverage. |
| [c.c](../../src/c.c#L45): inventory `safe_malloc` failure -> `memory_error` | Allocation yes; failure not expected. [memory_error](../../src/c.c#L32) calls `endwin` and `exit_game`. | Fail-fast redirect of the failure boundary if testing allocation failure; no curses cleanup in harness. |
| [debug.h](../../src/debug.h#L6), [debug.c](../../src/debug.c#L95): `MSG` -> `dbg__log_msg`/`fprintf`/`fflush(debug_file)` | ENTER/LEAVE are no-ops even with DO_DEBUG=1; MSG diagnostics are conditional, not expected in valid setup. | Harness-only diagnostic double to stderr with failure, rather than opening a debug file or dereferencing NULL `debug_file`. |

**R:** [random_extern.rs](../../src/random_extern.rs#L6) exposes `randint`,
`rand_rep`, and `randnor`; their current wrappers use `thread_rng` in
[random.rs](../../src/rng/random.rs#L11). The minimal path needs the allocation
roll only; other loop rolls are listed above. **A:** RNG1 must establish which
Rust callers its scoped override controls; C symbol interception cannot be
assumed to catch direct Rust RNG calls.

## Recommended HL3 Interaction: Ration Pickup

Recommendation: dispatch lower-case `l` onto a single ration on dry open floor.
This is an inventory interaction through production `command()` and `carry`,
not a direct call to an inventory helper. It avoids combat, traps, money,
selection prompts, eating effects, and identification randomness.

### Additional Global State

Each dependency is **R**; the proposed complete fixture is **A** until HL3 runs.

| State | Required setup and source |
| --- | --- |
| Cave and position | Starting cell `cptr=1`, `tptr=0`; adjacent target open, dry, `cptr=0`, `tptr=k` for one valid nonzero object slot. Keep panel dimensions and bounds valid. [move.c](../../src/player_action/move.c#L370) transfers occupancy before pickup and updates coordinates afterward. |
| `t_list[k]`, `blank_treasure`, `tcptr` | Item `tval=Food` (80), `subval=307` (ration), `number=1`, positive fixed weight, no effect flags; use an initialized record compatible with Rust `Item`. [food conversion](../../src/conversion/item_subtype/food.rs#L28) verifies subtype; [constants.h](../../src/constants.h#L197) verifies type. `k` must be allocated, not already on the initialized object free list. [pusht](../../src/misc.c#L1260) clears the item and links the slot back through `p1`. |
| Inventory pointers and counters | Initially `inventory_list=NULL`, `inven_ctr=0`, `inven_weight=0`; reset `inven_temp` and list cursors such as `cur_inven`. [carry](../../src/player_action/move.c#L32) copies the floor record before `pusht`; [add_inven_item](../../src/inventory/inven.c#L64) allocates one node, updates weight/count, and initializes its flags/links. |
| Carry capacity | Initialize Rust player's current strength, weight, and extra bulk carry so item weight is comfortably below `C_player_max_bulk()*100`. [inven_check_weight](../../src/inventory/inven.c#L1494) enforces this; [max_bulk](../../src/player/data.rs#L564) reads those fields. `inven_check_num` currently returns true, not a capacity limit. |
| Messages and control flags | Active capture; initially clean `msg_flag`, history/last-message state, `reset_flag`, and `com_val`; loop controls find/search and tick counters. Keep the minimal flags above throughout the run. [carry](../../src/player_action/move.c#L36) clears find mode and [io.c](../../src/io.c#L231) records messages under capture. |

### Expected Acceptance Assertions

The following expectations are **R** deductions from the cited branches,
**not executed C-caller evidence**:

* Position and player occupancy move to the target, old occupancy clears,
	target `tptr` becomes zero, and `reset_flag` stays false.
* One inventory node contains the ration with count 1; `inven_ctr` becomes 1
	and `inven_weight` increases by exactly its weight. The object slot is
	returned to the free list. [inven_carry](../../src/inventory/inven.c#L188)
	uses the real copied record, not a generated random item.
* Captured messages are exactly `You have ration of food. (1a)` in order,
	with no additional messages. [carry](../../src/player_action/move.c#L71)
	assigns page 1/item a for the first node;
	[food naming](../../src/data/item_name/subtype/food.rs#L5) and
	[quantity formatting](../../src/data/item_name/helpers.rs#L22) omit an
	article/count prefix for a single ration.
* HP remains full; food and game age advance as normal. A real consumed
	tick must include the post-command creature call, not stop immediately
	after dispatch. **A:** HL1's limit semantics must make this test possible.
* Run twice from restored C and Rust state, reseed RNG, free the allocated
	inventory node, end capture, and restore doubles. Static loop caches are
	reset by entry; static light state must be normalized by real lighting
	or explicitly shown not to affect the chosen blind path. **A:** repeated
	runs are the evidence that restoration is complete, not this checklist.

## Verification Boundary

Source reading is not C-caller or headless gameplay coverage. Script exhaustion,
turn limits, restoration, exact messages, and repeated scenarios remain for
HT1 and HL1-HL5 to test.

SEAM1 validation checks local links, source anchors, required inventory topics,
and whitespace only. RED/GREEN gameplay tests and `make check` are not evidence
for this documentation-only task and were not run.