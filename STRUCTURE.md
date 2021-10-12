# Structure

Work-in-progress.

My plan for how to structure logic, and dependencies.

I will use this project to test how project / code structure affects the code,
so the layout might not always be optimal.
```
/src/init           - Object and logic initialization
- Can only be used by the main()-function
- Can use any file in /src/logic or /src/model

/src/pregame       - Startup-menu, etc
- The public pregame function can only be called from pregame/main.c
- pregame/main.h can only be called from the main()-function
- pregame/*.c can use any file in /src/logic or /src/model

/src/logic          - Game logic
/src/model          - Types, classes, structs
/src/template       - Templates of models
```

## Namespaces

* dbg = src/debug.h
* init = src/init/\*
* pregame = src/pre-game/\*
