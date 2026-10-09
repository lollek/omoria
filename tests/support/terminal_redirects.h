#pragma once

#include "terminal.h"

#define inkey headless_inkey
#define inkey_delay headless_inkey_delay
#define put_buffer headless_put_buffer
#define put_buffer_attr headless_put_buffer_attr
#define Erase_Line headless_erase_line
#define prt headless_prt
#define Print headless_print
#define print_str headless_print_str
#define print_chstr headless_print_chstr
#define move_cursor headless_move_cursor
#define C_print_equipment_block headless_print_equipment_block
#define C_print_stat_block headless_print_stat_block
#define C_clear_screen headless_clear_screen
#define flush headless_flush
#undef refresh
#define refresh() headless_refresh()
#define get_com headless_get_com
#define get_yes_no headless_get_yes_no
#define Get_String headless_get_string