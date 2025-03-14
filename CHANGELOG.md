# Changelog

## 5-alpha523

February 2025

* Add lots of names to new UI
* Monster changes
  * All spiders are now imps
  * Removed some not-so-fun monsters
* Fix bug where it was not clear that your current item was cursed
* Starting items are much more generous

## 5-alpha522

January 2025
 
* Give earlier error messages if master file is not found
* Automatically create and initialize masters file if it does not exist
* Fix issue were days and seconds sometimes switched places in game logic
* Add axe, helm and wand names to new UI
* Fix bug where cloaks became a kind of bracer
* Fix critical bug when porting items to rust. flag1 and flag2 variables were switched, causing all manners of strange bugs.
* Changes to how rivers are created, since they caused infinite loops and segfaults. Still, unsure if the code is correct.
* Capitalize light sources in equipment list
* Make throwing more viable by making thrown objects fall to the ground even when hitting.
* Fix infinite loop if we wanted a monster attribute which was not applied to any monster at that level

## 5-alpha521

* Lots of automatic code cleanup. What could possibly go wrong

## 5.2.0-alpha

25 October 2022

* Lots of internal changes
* Fix crash bug when viewing ones inventory
* Add debug-feature to view one's inventory with both I and Alt-I, which have different code implementations. Will remove one after all code has been migrated.

## 5.1.1-alpha

13 February 2022

* Only internal changes

## 5.1.0-alpha

12 February 2022

* Massive code changes to items. Added, removed and modified a lot of items.
  Also removed "colors". E.g. potions are now only "potions", not "red potion".
  This is not final, and there will be a lot of changes going forward.

## 5.0.4

9 October 2021

* Remove the functionality where you can give monsters custom names. Seemed like
an unncessary hack.
* Better error handling for monster.dat. Now you will get an error message if
  you break it.

## 5.0.3

4 July 2021

Remove numpad keys for movement. Will maybe reuse those buttons of other things
in the future

Pressing . waits one turn

## 5.0.2

27 June 2021

Make mana gain per level same as magic stat modifier. If needed I will rescale
mana cost for spells instead (and gain from potions) since this makes it easier
to understand

Change so all magic types only use one stat each, giving the following:
* Arcane -> Intelligence
* Divine -> Wisdom
* Nature -> Wisdom
* Song -> Charisma
* Chakra -> Wisdom

## 5.0.1

25 June 2021

If you shoot in a direction and miss the first monster, your missile may now
hit the second monster standing behind it.


## 5.0.0
No changes, but since I'm almost a year into this I should start using version
numbers

## 5.0.0.pre-alpha

### May 2019
* Some slight changes to starting items. You now start with more stuff to avoid
  unplayable starts where you cannot buy anything useful.

* Removed all maps from the game. The whole point about this game is that moria
  is unmappable, so it's weird that you can map out floors at all.

### April 2019
* Add barbarian
Alternative to fighter, who can rage in fights. Cannot use heavy armor. May be
more buggy than other classes at first

* Changed stats
No longer goes from 3 -> 17 -> 18/00 -> 18/99. Now instead goes from 0 and up,
where 18/00 is 18 and 18/99 is around 24-25. Now stuff is generally calculated
the 3e dnd way, where modifiers = (stat - 10) / 2. Before it was totally up in
the air, where the cut-off-points usually were in between skill levels. After
this change, most modifiers from stats are somewhere between half as good and
twice as good as before, depending on skills, levels and stats.

* Removed titles
Some were cool, but mostly it was annoying having to come up with titles for new
classes, also a lot of them were immersion-breaking.

* Your race no longer has any effect on your health points
Now comes fully from your class

### February 2019

* Add restrictions on what druids can wear (Can be seen through Alt-C)
- Weapons: Club, Dagger, Quarterstaff, Sling
- Armor, Shields, Misc: Only stuff without large amounts of metal in it

* Moved druid magic from 'p' to 'm'

* Changes to how 'P' is displayed

* Fix crash on displaying skeletons

* Lots of code cleanup, A.K.A introducing bugs

### December 2018
* WIP - Begun work on a new way of displaying item names. Will make names more
  easy to understand without resorting to the manual.

* Some things are no longer stackable (Torches, valuable jewelry, amulets)
  This feels more natural and balanced to me, and I would like resource management
  to be a bigger part of the game, which this is a step towards.

### Unknown date

Add some colors to parts of the game
* Health and mana will move between 3 colors depending on high/medium/low

Removed gaining experience from using spells for the first time.

Added a menu when starting the game, where you can select which character to
play. Making it easier to switch between characters.

Removed insurance. Seems like a weird functionality to me. For posterity, this was the
help-page for insurance:

     The Insurance Shop is where you buy insurance for your items
      and character, so that if the system should  crash,  or  if  your
      terminal quits on you, you can cash in your insurance and get the
      character back.  Note the insurance  will only work when you save
      the character after buying it.

Changed how scores are calculated. Now deepest level * 1000 + experience. But
will tweak this more later.

Removed manually setting difficulty to 1-5. No longer needed since I will only
use one difficulty always. This should also make it possible to balance.

## Original omoria PROGRESS-file

### 4.85.22

Aug 20, 2003

Fixed printing of water in the P command and fixed a buffer overflow
if you change OUTPAGE_WIDTH cover the full map.

Proved that I still maintain the code if someone emails me a problem.
Thanks Robin!  :)

### 4.85.21

May 28, 2002

Additional patches from John Guthrie (from Oct 2001, work got busy):

Correct the Reputation and Swimming values in the character printouts.
Fix the learn_* functions to not segfault when picking new spells.
Fix more problems with enchant armor.
Spell dispel correctly.

Also, NetBSD patches from Ben Wong:

io.c
This patch is necessary since on NetBSD, using setgid is final; if you
give up privileges you can't get them back. Setegid is the way to go
since it only changes your effective gid. I believe this change should
work fine under Linux as well.  [ kertes: I tested this breifly, and
had no trouble. ]

term.c
This patch fixes a few problems in term.c. Primarily, it makes it
#include <sgtty.h> instead of <termios.h>. It also has some of the
#ifdef code cleaned up a bit since I couldn't figure out the multiply
nested conditional macros. Finally, it fixes a non-portable way of
using curses: I believe using stdscr->_cury is deprecated. I replaced
it with getcury(stdscr), which hopefully will work everywhere.

unix.c
This patch is necessary since on NetBSD, the symbol "unix" is no
longer automatically defined by the compiler. However, __NetBSD__ is.
Again, this should work fine under Linux. (If the #includes cause a
problem, they can be removed; they are only to prevent warnings from
gcc).

Also includes his patches for the Makefile and configure.h in the NetBSD
directory.

Added some code to bother to check the length of the DATA_FILE_PATH rather
than just trusting people to notice that segfaults are caused by long
paths.  And I upped the max path length to 170 chars from 60 to accommodate
NetBSD packages installed without root access.  I hope that is plenty, it
was certainly lazy.  But at least it checks for overflows now!


### 4.85.20

June 6, 2001

Patches from John Guthrie to fix various things:

Fix help message about what flags exist.
Fix *Enchant Armor* scrolls so they actually enchant armor.
Fix a bunch of item prices in the stores that depended on how identified the item is.
Fix mhelp.pl to be smarter about figuring out how many lines are on the screen.
Fix gain_mana() so it does the right thing on even level ups.


### 4.85.19

Oct 26 2000

Applied some patches from cloister.  Here are his notes:

i found three bugs related to timekeeping when spending time in the inn plus a
couple of other little bugs.

1. careful observation of the dates in the game reveals that spending time in
   the inn, no matter how many days you pay for (1, 3, 7), always gets you only
   1 calendar day, due to a bug in misc.c's add_days() function.  you did,
   however, get the correct number of loops through store_maint, the correct
   adjustments to player flags, etc.  it's just py.misc.cur_age that wasn't
   updated correctly.  fixed add_days.
2. having fixed add_days, there was a bug in spend_time() when you buy 3 days
   in the inn.  check out the add_days() call that was there.  i have no idea
   what the original programmer could have been thinking...
3. having fixed #2, the p1 value on the "3 days in the inn" item in values.h
   was also wrong.  it was set to '30', not 3.  my only guess is that
   originally someone wanted to have an option for spending a month in the
   inn (which, now that i think about it, would also explain bug number 2, if
   the person was trying to emulate--albeit poorly--months of slightly
   differing length).
4. the druid Ring of Fire spell didn't work analagously to the mage version or
   the version you get when you use the "broken set of teeth from a dragon"
   item.  fixed.
5. the price when selling rings of protection and rings of increase-to-hit was
   based on the rings' +todam value.  looks like a cut-and-paste error,
   probably from when those rings were invented originally...


### 4.85.18

Oct 15 2000

correctly set random seed in casino so that it does not always play
exactly the same games after loading a saved character.

A bunch of little things I did back in March that I have forgotten about.

### 4.85.17

Feb 17 2000

cloister wrote item_guide.txt, a guide to the treasure_type struct.

replaced a bunch of constant tvals with horn, chime, instrument, etc.

fixed spelling in check for Amethyst, now they cost more.

items inside chests will be created at a level near where the chest was
found rather than at the level the chest is opened.

moved all the potion2 and scroll2 effects into flag2.

added valuable_gems to things that can be recharged.

wizard_create will let you create anything now (including traps!).

### 4.85.16

Jan 31 2000

Fixed blessed blades so they set the correct flags.

Fix weapon useage penalties for priests and rangers.  Now priests have
some and rangers do not.

Fixed number of blows per attack code, integer math can cause terrible rounding
errors.  Monks will be helped a lot by this.

Fixed genocide, it usually went into an infinite loop.

### 4.85.15

Jan 7 2000

Fixed bug in starlight/light_line that would crash the game if the light
killed a creature before all of the power was used up.

Many thanks to Marc Lehmann for his patch to fix the attr_get call with
newer curses.  If things are still broken let me know.

Don't just detrap chests when disarming them, unlock them too.

### 4.85.14

Dec 22 1999

Added a break; between Storm Giant and Cloud Giant Belts. Ooops.
Fixed % full description for empty bags of holding.  Do not strcat to
a string that has not been set yet!
Added '*' option for resting: rest until mana and hits are at maximum.
Chimes of disarming no longer cause chests to be empty.
When blindness ends show area around character.
Wand of wall building no longer pins creatures that do not move in water.
They are in a wall, not water!
Changed most of the floor_type checks to not use literals.
If a monster was in the dark, hit by something and cast teleport on itself
then the old position (shown when the monster was hit) was not redrawn.
I never noticed that at UW, but this week it happened at least 3 times. Odd.

### 4.85.13

Dec 18 1999

Fixed a huge problem where items that were removed by putting on something
else of the same type did not undo some of the magic bonuses.
Fixed a few item sets that caused crashes if DO_DEBUG was on.
Fixed an off by one problem with the material types of magic items.
Fixed some of the monster death messages.


### 4.85.12

Dec 13 1999

Fixed another bug in compact_monsters that was fixed in the 4.94 sources.
No loops in the list, but it still locked the game up for quite a while.
Show % full on bags of holding.
Interrupt rest when hunger messages are printed. This nearly killed me.
Sort magic/prayer/song books so you they are always in the same order.
Let petrification wear off over time, partly taken from the 4.94 sources.
Fixed message printed by area spells that killed things (monster names
were sometimes lost)


### 4.85.11

Dec 5 1999

After a one year break I finally started playing again.
Reannounced on Freshmeat.

Fixed a big problem in compact_monsters that would create loops in 
the monster list (m_list).  This locks up the program fairly fast.
Also added USE_CURSES_ATTRS to configure.h to make it easy for the
people who have had problems with attr_get().  Some minor graphics
fixes including one caused by using the J command (parts of the map
were not redrawn correctly).


### 4.85.10

Oct 14 1998

Now all the data files are checked before ncurses is given control of the
screen.  This should help all the people who have reported that the game
exits with no errors right after starting.
I also added a message if the help file could not be executed so the
screen does not just flash.

### 4.85.9

Oct 12 1998

Finished writing the put/take commands for bags of holding, and fixed
two bugs that showed up now that bags can be used.
Also finished the 'I' command (selective inventory) which had also been
skipped.
Staves of darkness no longer mess up the screen when used.
Enter can be used to reply to "-more-"
^M is still broken, getch returns 10 (^J) and I don't know why. Use 'V'.

### 4.85.8

Oct 05 1998

Made mhelp.pl output \n\r rather than just \n, should fix messed up output
when using help from within the game.
Replaced index call with strchr
Fixed bug in bi__insure_all_items that made the game look like it had hung.

### 4.85.7

Sep 29 1998

Should fix a problem in monk.c where a for loop should have begun at 0.
Added break; to paladin, druid and priest spells so only one is cast
at a time.  I don't think I play them very often!

### 4.85.6
* Sep 27 1998
    - Fixes a crash in the speeded up identify selection code.
    - Call endwin to clean up ncurses on exit.

### 4.85.5
* july and aug
    - I must have changed something between pl 4 and 5, but I have no idea what.
* sep 16 1998
    - announced package to freshmeat.net

### 4.85.4
* july 21 1998
    - fixed learn_spell and learn_prayer offering spell from outside correct range.
* july 24 1998
    - use Mersenne Twister random number library
* july 25 1998
    - cut about 30k from save files using run length encoding on the dungeon floor
    - always xor save file data against random numbers

### 4.85.3
* july 20 1998
    - added reduced map and better Locate from umoria
    - fixed store hours and bribes so correct index is used

### Pre 4.85

* july 18 1998
    - hmmm, done with the post.  is that everything?
* july 13 1998
    - moved everything up and left by one, icky macros do the job
      but I did not want to edit every single prt/print/..., fixed right
      edge
    - get_char bails if any lines are bad.
    - bolts and balls are visable now!
* july 12 1998
    - added encrypt and decrypt switches (-E and -D) for save files help is done
    - running out of stuff before the post has to be done :)
* july 11 1998
    - insurance is done
* july 10 1998
    - I don't seem to be keeping this log up to date at all :)
      All but about 8 files are "done".
    - No crashes in a while, except when loading damaged save files.
        encrypts save files, high scores work working on wizard
        commands (wizard passwords work now so you don't have to
        use gdb to set it)
    - Master file is in place, uses gdbm.
    - Insurance, Help and Trading Post are the only major things left
    - (want to have a black market so single system users can benifit)
* june, mid 1998
    - Jeanetta started some alpha testing.  wow, lots of bugs.
    - Actually working on the code again (had to fix J's crashes)
* march 1 1998
    - you can actually play the game now

### 1997

Bank.inc		done with all but deposit of items (it was disabled)  
Bit_Pos.mar		done  
Bit_Pos64.for		done  
Bj.inc			done  
Blow.inc		done  
Build.com		<none>  
Casino.pas		done  
Changer.inc		<none>             (code never used, bank does it...)  
Constants.inc		done  
Create.pas		done   (except "get_ssn")  
Creature.pas		done  
Death.pas		done  
Desc.pas		done  
Distance.mar		<done>  
Dungeon.pas		done           should get rid of globals...  
Eat.inc			done  
Files.pas		done  
Generate.pas		nearly done,   need to implement old_seed stuff  
Get_Account.mar		<none>  
Help.pas		done  
Horse.inc		done           (needs testing, but seems to work)  
Imoria.cld		<done>  
Imoria_Setup.com	<none>  
Insert.mar		done  
Insurance.inc		done  
Inven.pas		done  
Io.pas			done       (trap vms messages not implemented, duh)  
Magic.inc		done  
Maxmin.mar		done  
Minmax.mar		done  
Misc.pas		       already_playing  
Monk.inc		done  
Monsters.dat		<done>  
Moria.pas		done         should probably delete //ed code  
Moriadef.cld		<done>  
Moriahlp.rnh		<done, wrote perl script to display help>  
Netopen.pas		<none> stuff for post, not needed  
Play.inc		done  
Player.pas		done  
Potions.inc		done  
Prayer.inc		done  
Putqio.mar		<done>  
Quest.pas		done, added item rewards  
Randint.mar		<done>  
Randrep.mar		<done>  
Routines.inc		<done>  
Save.pas		done          should check for errors when writing  
Screen.pas		done  
Scrolls.inc		done  
Sing.inc		done  
Slots.inc		done  
Spells.inc		done  
Staffs.inc		done  
Store.pas		done          want to import shakesphere insulter  
Subquad.mar		<done>  
Termdef.pas		<none> probably not needed since I stole umoria code  
Trade.inc		done  
Types.inc		done  
Users.mar		<done>  
Uw_Id.for		<none> probably not needed  
Values.inc		done  
Variables.inc		done  
Wands.inc		done  
Wizard.pas		done  

( source can be found at wbcms.cc.buffalo.edu in maslib/games/imoria )

Started on nov 16, 1997.  Got the source, un-sharded it!  
nov 17, actually started converting stuff.  
nov 22, finished with most of the types/vars/values, time to recode!  
dec 7,  you can create a character and get initial inventory and spells!  
