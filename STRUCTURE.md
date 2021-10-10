# Structure

Work-in-progress.

My plan for how to structure logic, and dependencies.

I will use this project to test how project / code structure affects the code,
so the layout might not always be optimal.
```
/src/init           - Object and logic initialization
- Can only be used by the main()-function
- Can use any file outside /src/init as a dependency

/src/pre-game       - Startup-menu, etc
/src/pre-game/logic
/src/pre-game/model
- Only depended on by main(), and before game starts
- Can depend on /src/logic and /src/model

/src/logic          - Game logic
/src/model          - Types, classes, structs
```

## Namespaces

* dbg = src/debug.h
* init = src/init/*
