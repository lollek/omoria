#pragma once

#define LIGHT_RADIUS 1

/*{ Package for moving the character's light about the screen     }*/
/*{ Three cases : Normal, Finding, and Blind              -RAK-   }*/
/* (so what is that sub4 thing?) */
void dungeon_light_move(long y1, long x1, long y2, long x2);

int dungeon_light_should_light_cell(long player_row, long player_col,
                                    long target_row, long target_col,
                                    long light_radius);

  /*{ Room is lit, make it appear                           -RAK-   }*/
void dungeon_light_room(long y, long x);
