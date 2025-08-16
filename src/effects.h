#pragma once

/*{ Corrode the unsuspecting person's armor               -RAK-   }*/
void corrode_gas(char const *kb_str);

/*{ Poison gas the poor adventurer...                               -RAK-   }*/
void poison_gas(long dam, char const *kb_str);

/*{ Burn the adventurer up...                                   -RAK-   }*/
void fire_dam(long dam, char const *kb_str);

/*{ Throw acid on the hapless victim                      -RAK-   }*/
void acid_dam(long dam, char const *kb_str);

/*{ Freeze the adventurer...                                   -RAK-   }*/
void cold_dam(long dam, char const *kb_str);

/*{ Lightning bolt the sucker away...                     -RAK-   }*/
void light_dam(long dam, char const *kb_str);
