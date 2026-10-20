use super::{basic, element, movement, spawn, status};
use crate::model::{ActivationKind, StoreKind};

mod globals;

pub(super) use self::globals::hit_trap_global;
use self::globals::GlobalTrap;

fn dispatch(context: &mut GlobalTrap, kind: ActivationKind, damage: i64, y: i64, x: i64) {
    match kind {
        ActivationKind::OpenPit => basic::open_pit(context, damage),
        ActivationKind::Arrow => basic::arrow(context, damage),
        ActivationKind::CoveredPit => movement::covered_pit(damage, y, x),
        ActivationKind::TrapDoor => movement::trap_door(damage),
        ActivationKind::SleepGas => status::sleep_gas(),
        ActivationKind::HiddenObject => spawn::hidden_object(y, x),
        ActivationKind::StrengthDart => status::strength_dart(damage),
        ActivationKind::Teleport => movement::teleport(),
        ActivationKind::Rockfall => spawn::rockfall(damage, y, x),
        ActivationKind::CorrodeGas => element::corrode_gas(),
        ActivationKind::SummonMonster => spawn::summon_monster(y, x),
        ActivationKind::Fire => element::fire(damage),
        ActivationKind::Acid => element::acid(damage),
        ActivationKind::PoisonGas => element::poison_gas(damage),
        ActivationKind::BlindGas => status::blind_gas(),
        ActivationKind::ConfuseGas => status::confuse_gas(),
        ActivationKind::SlowDart => status::slow_dart(damage),
        ActivationKind::ConstitutionDart => status::constitution_dart(damage),
        ActivationKind::SecretDoor | ActivationKind::ScareMonster => {}
        ActivationKind::Chute => movement::chute(damage),
        ActivationKind::GeneralStore => context.store(StoreKind::GeneralStore),
        ActivationKind::Armory => context.store(StoreKind::Armory),
        ActivationKind::WeaponSmiths => context.store(StoreKind::WeaponSmiths),
        ActivationKind::Temple => context.store(StoreKind::Temple),
        ActivationKind::AlchemyShop => context.store(StoreKind::AlchemyShop),
        ActivationKind::MagicShop => context.store(StoreKind::MagicShop),
        ActivationKind::Inn => context.store(StoreKind::Inn),
        ActivationKind::TradingPost => {
            if context.store_open(StoreKind::TradingPost) {
                context.trading_post();
            }
        }
        ActivationKind::Library => context.store(StoreKind::Library),
        ActivationKind::MusicShop => context.store(StoreKind::MusicShop),
        ActivationKind::InsuranceShop => {
            context.message("The insurance shop has gone out of business.")
        }
        ActivationKind::Bank => {
            if context.store_open(StoreKind::Bank) {
                context.bank();
            }
        }
        ActivationKind::GemStore => context.store(StoreKind::GemStore),
        ActivationKind::MoneyExchange => {
            if context.store_open(StoreKind::MoneyExchange) {
                context.message("Oh, just go to the bloody bank!");
            }
        }
        ActivationKind::Casino => {
            if context.store_open(StoreKind::Casino) {
                context.casino();
            }
        }
        ActivationKind::AllNiteDeli => context.store(StoreKind::AllNiteDeli),
        ActivationKind::Fortress => context.fortress(),
        ActivationKind::BlackMarket => context.store(StoreKind::BlackMarket),
        ActivationKind::House120 | ActivationKind::House121 | ActivationKind::House122 => {
            context.house(y, x)
        }
        ActivationKind::Whirlpool => movement::whirlpool(damage),
        ActivationKind::Unknown(_) => context.message("You got lucky: unknown trap value."),
    }
}
