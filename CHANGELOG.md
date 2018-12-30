# Changelog
This is the changelog for version 5+. For info on versions older than that, see
the PROGRESS-file.

## 5.0.0.pre-alpha

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

