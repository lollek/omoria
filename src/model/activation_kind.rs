/// Activation codes shared by dungeon traps and building entrances.
///
/// These are item subvals, not item types or C store identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationKind {
    OpenPit,
    Arrow,
    CoveredPit,
    TrapDoor,
    SleepGas,
    HiddenObject,
    StrengthDart,
    Teleport,
    Rockfall,
    CorrodeGas,
    SummonMonster,
    Fire,
    Acid,
    PoisonGas,
    BlindGas,
    ConfuseGas,
    SlowDart,
    ConstitutionDart,
    /// No-op activation for the secret-door/closed-door template (subval 19).
    SecretDoor,
    Chute,
    ScareMonster,
    GeneralStore,
    Armory,
    WeaponSmiths,
    Temple,
    AlchemyShop,
    MagicShop,
    Inn,
    TradingPost,
    Library,
    MusicShop,
    InsuranceShop,
    Bank,
    GemStore,
    MoneyExchange,
    Casino,
    AllNiteDeli,
    Fortress,
    BlackMarket,
    House120,
    House121,
    House122,
    Whirlpool,
    Unknown(i64),
}

impl From<i64> for ActivationKind {
    fn from(subval: i64) -> Self {
        match subval {
            1 => Self::OpenPit,
            2 => Self::Arrow,
            3 => Self::CoveredPit,
            4 => Self::TrapDoor,
            5 => Self::SleepGas,
            6 => Self::HiddenObject,
            7 => Self::StrengthDart,
            8 => Self::Teleport,
            9 => Self::Rockfall,
            10 => Self::CorrodeGas,
            11 => Self::SummonMonster,
            12 => Self::Fire,
            13 => Self::Acid,
            14 => Self::PoisonGas,
            15 => Self::BlindGas,
            16 => Self::ConfuseGas,
            17 => Self::SlowDart,
            18 => Self::ConstitutionDart,
            19 => Self::SecretDoor,
            20 => Self::Chute,
            99 => Self::ScareMonster,
            101 => Self::GeneralStore,
            102 => Self::Armory,
            103 => Self::WeaponSmiths,
            104 => Self::Temple,
            105 => Self::AlchemyShop,
            106 => Self::MagicShop,
            107 => Self::Inn,
            108 => Self::TradingPost,
            109 => Self::Library,
            110 => Self::MusicShop,
            111 => Self::InsuranceShop,
            112 => Self::Bank,
            113 => Self::GemStore,
            114 => Self::MoneyExchange,
            115 => Self::Casino,
            116 => Self::AllNiteDeli,
            117 => Self::Fortress,
            118 => Self::BlackMarket,
            120 => Self::House120,
            121 => Self::House121,
            122 => Self::House122,
            123 => Self::Whirlpool,
            _ => Self::Unknown(subval),
        }
    }
}

impl ActivationKind {
    pub const fn subval(self) -> i64 {
        match self {
            Self::OpenPit => 1,
            Self::Arrow => 2,
            Self::CoveredPit => 3,
            Self::TrapDoor => 4,
            Self::SleepGas => 5,
            Self::HiddenObject => 6,
            Self::StrengthDart => 7,
            Self::Teleport => 8,
            Self::Rockfall => 9,
            Self::CorrodeGas => 10,
            Self::SummonMonster => 11,
            Self::Fire => 12,
            Self::Acid => 13,
            Self::PoisonGas => 14,
            Self::BlindGas => 15,
            Self::ConfuseGas => 16,
            Self::SlowDart => 17,
            Self::ConstitutionDart => 18,
            Self::SecretDoor => 19,
            Self::Chute => 20,
            Self::ScareMonster => 99,
            Self::GeneralStore => 101,
            Self::Armory => 102,
            Self::WeaponSmiths => 103,
            Self::Temple => 104,
            Self::AlchemyShop => 105,
            Self::MagicShop => 106,
            Self::Inn => 107,
            Self::TradingPost => 108,
            Self::Library => 109,
            Self::MusicShop => 110,
            Self::InsuranceShop => 111,
            Self::Bank => 112,
            Self::GemStore => 113,
            Self::MoneyExchange => 114,
            Self::Casino => 115,
            Self::AllNiteDeli => 116,
            Self::Fortress => 117,
            Self::BlackMarket => 118,
            Self::House120 => 120,
            Self::House121 => 121,
            Self::House122 => 122,
            Self::Whirlpool => 123,
            Self::Unknown(subval) => subval,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ActivationKind::*;
    use super::*;

    const KNOWN: &[(i64, ActivationKind)] = &[
        (1, OpenPit),
        (2, Arrow),
        (3, CoveredPit),
        (4, TrapDoor),
        (5, SleepGas),
        (6, HiddenObject),
        (7, StrengthDart),
        (8, Teleport),
        (9, Rockfall),
        (10, CorrodeGas),
        (11, SummonMonster),
        (12, Fire),
        (13, Acid),
        (14, PoisonGas),
        (15, BlindGas),
        (16, ConfuseGas),
        (17, SlowDart),
        (18, ConstitutionDart),
        (19, SecretDoor),
        (20, Chute),
        (99, ScareMonster),
        (101, GeneralStore),
        (102, Armory),
        (103, WeaponSmiths),
        (104, Temple),
        (105, AlchemyShop),
        (106, MagicShop),
        (107, Inn),
        (108, TradingPost),
        (109, Library),
        (110, MusicShop),
        (111, InsuranceShop),
        (112, Bank),
        (113, GemStore),
        (114, MoneyExchange),
        (115, Casino),
        (116, AllNiteDeli),
        (117, Fortress),
        (118, BlackMarket),
        (120, House120),
        (121, House121),
        (122, House122),
        (123, Whirlpool),
    ];

    const UNKNOWN: &[i64] = &[i64::MIN, -1, 0, 21, 98, 100, 119, 124, 999, i64::MAX];

    #[test]
    fn activation_kind_decodes_known_legacy_subvals() {
        for &(subval, expected) in KNOWN {
            assert_eq!(ActivationKind::from(subval), expected, "subval {subval}");
        }
    }

    #[test]
    fn activation_kind_encodes_known_legacy_subvals() {
        for &(expected, kind) in KNOWN {
            assert_eq!(kind.subval(), expected, "{kind:?}");
        }
    }

    #[test]
    fn activation_kind_keeps_unhandled_subvals_unknown() {
        for &subval in UNKNOWN {
            assert_eq!(ActivationKind::from(subval), Unknown(subval));
        }
    }

    #[test]
    fn activation_kind_preserves_unknown_legacy_subvals() {
        for &subval in UNKNOWN {
            assert_eq!(ActivationKind::from(subval).subval(), subval);
        }
    }
}
