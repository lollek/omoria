use super::{basic, element, movement, spawn, status};

mod globals;

pub(super) use self::globals::hit_trap_global;
use self::globals::GlobalTrap;

fn dispatch(context: &mut GlobalTrap, subval: i64, damage: i64, y: i64, x: i64) {
    match subval {
        1 => basic::open_pit(context, damage),
        2 => basic::arrow(context, damage),
        3 => movement::covered_pit(damage, y, x),
        4 => movement::trap_door(damage),
        5 => status::sleep_gas(),
        6 => spawn::hidden_object(y, x),
        7 => status::strength_dart(damage),
        8 => movement::teleport(),
        9 => spawn::rockfall(damage, y, x),
        10 => element::corrode_gas(),
        11 => spawn::summon_monster(y, x),
        12 => element::fire(damage),
        13 => element::acid(damage),
        14 => element::poison_gas(damage),
        15 => status::blind_gas(),
        16 => status::confuse_gas(),
        17 => status::slow_dart(damage),
        18 => status::constitution_dart(damage),
        19 | 99 => {}
        20 => movement::chute(damage),
        101..=107 => context.store((subval - 101) as libc::c_int),
        108 => {
            if context.store_open(12) {
                context.trading_post();
            }
        }
        109 => context.store(7),
        110 => context.store(8),
        111 => context.message("The insurance shop has gone out of business."),
        112 => {
            if context.store_open(14) {
                context.bank();
            }
        }
        113 => context.store(9),
        114 => {
            if context.store_open(15) {
                context.message("Oh, just go to the bloody bank!");
            }
        }
        115 => {
            if context.store_open(16) {
                context.casino();
            }
        }
        116 => context.store(10),
        117 => context.fortress(),
        118 => context.store(11),
        120..=122 => context.house(y, x),
        123 => movement::whirlpool(damage),
        _ => context.message("You got lucky: unknown trap value."),
    }
}
