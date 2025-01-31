#pragma once
/*
 *	{ Dungeon generation values					}
 *	{ Note: The entire design of dungeon can be changed by only	}
 *	{	slight adjustments here.				}
 */
#define DUN_TUN_RND 36  /*{ Random direction (4 is min)		} */
#define DUN_TUN_CHG 70  /*{ Chance of changing direction (99 max)} */
#define DUN_TUN_FND 12  /*{ Distance for auto find to kick in	} */
#define DUN_TUN_CON 15  /*{ Chance of extra tunneling		} */
#define DUN_ROO_MEA 32  /*{ Mean of # of rooms, standard dev=2	} */
#define DUN_TUN_PEN 25  /*{ % chance of room doors		} */
#define DUN_TUN_JCT 15  /*{ % chance of doors at tunnel junctons	} */
#define DUN_STR_DEN 5   /*{ Density of streamers			} */
#define DUN_STR_RNG 2   /*{ Width of streamers			} */
#define DUN_STR_MAG 3   /*{ Number of magma streamers		} */
#define DUN_STR_QUA 2   /*{ Number of quartz streamers		} */
#define DUN_WTR_DEN 5   /*{ Density of water			} */
#define DUN_WTR_WIDTH 4 /*{ Width of river			} */
#define DUN_RIVERS 3    /*{ Number of rivers			} */
#define DUN_RIV_LEN 35  /*{ Maximum river length			} */
#define DUN_POOLS 3     /*{ Number of pools			} */
static const long dun_str_mc = 95;      // 1/x chance of treasure per magma
static const long dun_str_qc = 55;      // 1/x chance of treasure per quartz
static const long dun_unusual = 100;    // Level/x chance of unusual room
static const long treas_room_alloc = 7; // Amount of objects for rooms
static const long treas_any_alloc = 2;  // Amount of objects for corridors
static const long treas_gold_alloc = 2; // Amount of gold (and gems)

// Town level
#define TOT_STORES (MAX_STORES + MAX_UNNAMED)
#define MAX_HOUSE1 4     /*{ # of generic houses in town	} */
#define MAX_HOUSE2 8     /*{ # of small houses in town } */
#define DAY_MUGGING 50   /*{ 1/x chance that page gets mugged (day)} */
#define NIGHT_MUGGING 15 /*{ 1/x chance that page gets mugged (night)} */

