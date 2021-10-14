# Structure

Work-in-progress.

My plan for how to structure logic, and dependencies.

I will use this project to test how project / code structure affects the code,
so the layout might not always be optimal.

## Modules

The code should, when possible, be divided into modules with a clear interface,
making it easy to reason what functions should be callable from the outside.
Code should be divided into data models and logic, and not use "classes" with
functions, as it seems preferable to decouple the logic from the data structure.

Modules;
* Init - Handle initialization of data. Might call other module's init functions
  as well.
* Pregame - Things which happen before the game really begins.
* Save - All logic for saving and retrieving characters.

```
/src/init           - Object and logic initialization
- Can only be used by the main()-function
- Can use any file in /src/logic or /src/model

/src/logic          - Game logic
- Can use /src/logic and /src/model

/src/model          - Types, classes, structs
- Can use other classes from /src/model
- Can not use anything from /src/logic

/src/pregame       - Startup-menu, etc
- The public pregame function can only be called from pregame/main.c
- pregame/main.h can only be called from the main()-function
- pregame/*.c can use any file in /src/logic or /src/model

/src/template       - Templates of models
```

## Namespaces

* dbg = src/debug.h
* init = src/init/\*
* pregame = src/pre-game/\*
