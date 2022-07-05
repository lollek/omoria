#ifndef DUNGEON_LIGHT_H
#define DUNGEON_LIGHT_H

/*{ Package for moving the character's light about the screen     }*/
/*{ Three cases : Normal, Finding, and Blind              -RAK-   }*/
/* (so what is that sub4 thing?) */
void dungeon_light_move(long y1, long x1, long y2, long x2);

  /*{ Room is lit, make it appear                           -RAK-   }*/
void dungeon_light_room(long y, long x);

#endif // DUNGEON_LIGHT_H