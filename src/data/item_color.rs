use crate::data::item_type;
use crate::model::item_subtype::{
    AmuletSubType, FoodSubType, GemSubType, ItemSubType, MiscUsableSubType, Potion1SubType,
    RingSubType, Scroll1SubType, SwordSubType, WandSubType, WearableGemSubType,
};
use crate::model::{Item, ItemType, WornFlag1, WornFlag2};
use crate::user_interface::map_colors::{map_color_attr, MapColor};
use pancurses::chtype;
use std::convert::TryFrom;

const SCROLL_SLEEP_MONSTER_FLAG: u64 = 0x00001000;
const SCROLL_MAKE_MUNCHIES_FLAG2: u64 = 0x00000800;

pub fn color(item: &Item) -> MapColor {
    let Ok(item_type) = ItemType::try_from(item.tval) else {
        return MapColor::Item;
    };
    let family_color = item_type::color(&item_type, item.subval);

    if is_map_fixture(&item_type) {
        return family_color;
    }

    if let Some(role) = scroll_template_color(&item_type, item) {
        return role;
    }

    if is_equipment(&item_type) {
        if let Some(role) = special_attribute_color(item) {
            return role;
        }
        if let Some(role) = quality_color(item) {
            return role;
        }
    }

    item.item_subtype()
        .and_then(subtype_color)
        .unwrap_or(family_color)
}

pub fn color_key(item: &Item) -> i32 {
    color(item) as i32
}

pub fn glyph(item: &Item) -> chtype {
    let item_type = ItemType::try_from(item.tval).unwrap();
    let symbol = item_type::symbol(&item_type, item.subval);
    if matches!(item_type, ItemType::UnseenTrap | ItemType::SecretDoor) {
        symbol
    } else {
        symbol | map_color_attr(color(item))
    }
}

fn is_map_fixture(item_type: &ItemType) -> bool {
    matches!(
        item_type,
        ItemType::Money
            | ItemType::UnseenTrap
            | ItemType::SeenTrap
            | ItemType::Rubble
            | ItemType::OpenDoor
            | ItemType::ClosedDoor
            | ItemType::SecretDoor
            | ItemType::UpStaircase
            | ItemType::DownStaircase
            | ItemType::EntranceToStore
            | ItemType::UpSteepStaircase
            | ItemType::DownSteepStaircase
            | ItemType::Whirlpool
    )
}

fn scroll_template_color(item_type: &ItemType, item: &Item) -> Option<MapColor> {
    if !matches!(item_type, ItemType::Scroll1) {
        return None;
    }

    if item.flags & SCROLL_SLEEP_MONSTER_FLAG != 0 {
        Some(MapColor::ItemCombat)
    } else if item.flags2 & SCROLL_MAKE_MUNCHIES_FLAG2 != 0 {
        Some(MapColor::ItemNature)
    } else {
        None
    }
}

fn is_equipment(item_type: &ItemType) -> bool {
    matches!(
        item_type,
        ItemType::SlingAmmo
            | ItemType::Bolt
            | ItemType::Arrow
            | ItemType::Spike
            | ItemType::RangedWeapon
            | ItemType::HaftedWeapon
            | ItemType::PoleArm
            | ItemType::Dagger
            | ItemType::Sword
            | ItemType::Pick
            | ItemType::Maul
            | ItemType::GemHelm
            | ItemType::Boots
            | ItemType::Gloves
            | ItemType::Cloak
            | ItemType::Helm
            | ItemType::Shield
            | ItemType::HardArmor
            | ItemType::SoftArmor
            | ItemType::Bracers
            | ItemType::Belt
            | ItemType::Amulet
            | ItemType::Ring
            | ItemType::WearableGem
    )
}

fn special_attribute_color(item: &Item) -> Option<MapColor> {
    let flags1 = item.flags;
    let flags2 = item.flags2;

    if flags1 & WornFlag1::Cursed as u64 != 0
        || flags1 & (WornFlag1::AggravateMonsters as u64 | WornFlag1::RandomTeleportation as u64)
            != 0
        || flags2 & (WornFlag2::BadReputation as u64 | WornFlag2::Hunger as u64) != 0
    {
        return Some(MapColor::ItemHarmful);
    }

    if flags1
        & (WornFlag1::SlayDragon as u64
            | WornFlag1::SlayMonster as u64
            | WornFlag1::SlayEvil as u64
            | WornFlag1::SlayUndead as u64
            | WornFlag1::SlayCold as u64
            | WornFlag1::SlayFire as u64)
        != 0
        || flags2
            & (WornFlag2::SlayDemon as u64
                | WornFlag2::SoulSword as u64
                | WornFlag2::SlayRegenerators as u64)
            != 0
    {
        return Some(MapColor::ItemCombat);
    }

    if flags1
        & (WornFlag1::ResistFire as u64
            | WornFlag1::ResistAcid as u64
            | WornFlag1::ResistCold as u64
            | WornFlag1::ResistStatDrain as u64
            | WornFlag1::ResistParalysis as u64
            | WornFlag1::ResistLightning as u64
            | WornFlag1::FeatherFall as u64)
        != 0
        || flags2 & WornFlag2::MagicProof as u64 != 0
    {
        return Some(MapColor::ItemDefense);
    }

    if flags1
        & (WornFlag1::GivesStrength as u64
            | WornFlag1::GivesDexterity as u64
            | WornFlag1::GivesConstitution as u64
            | WornFlag1::GivesIntelligence as u64
            | WornFlag1::GivesWisdom as u64
            | WornFlag1::GivesCharisma as u64
            | WornFlag1::Searching as u64
            | WornFlag1::SlowDigestion as u64
            | WornFlag1::Stealth as u64
            | WornFlag1::Regeneration as u64
            | WornFlag1::IncreasedSpeed as u64
            | WornFlag1::SeeInvisible as u64
            | WornFlag1::ImprovedTunneling as u64)
        != 0
        || flags2
            & (WornFlag2::ImprovedSwimming as u64
                | WornFlag2::ImprovedDisarming as u64
                | WornFlag2::ImprovedCriticals as u64
                | WornFlag2::ImprovedCarrying as u64)
            != 0
    {
        return Some(MapColor::ItemArcane);
    }

    None
}

fn quality_color(item: &Item) -> Option<MapColor> {
    let modifiers = [
        item.p1,
        item.tohit.into(),
        item.todam.into(),
        item.toac.into(),
    ];
    if modifiers.iter().any(|&modifier| modifier < 0) {
        Some(MapColor::ItemQualityBad)
    } else if modifiers.iter().any(|&modifier| modifier > 0) {
        Some(MapColor::ItemQualityGood)
    } else {
        None
    }
}

fn subtype_color(subtype: ItemSubType) -> Option<MapColor> {
    match subtype {
        ItemSubType::Potion1(
            Potion1SubType::CureCriticalWounds
            | Potion1SubType::CureLightWounds
            | Potion1SubType::CureSeriousWounds
            | Potion1SubType::Healing
            | Potion1SubType::NeutralizePoison
            | Potion1SubType::RestoreCharisma
            | Potion1SubType::RestoreConstitution
            | Potion1SubType::RestoreDexterity
            | Potion1SubType::RestoreIntelligence
            | Potion1SubType::RestoreLifeLevels
            | Potion1SubType::RestoreMana
            | Potion1SubType::RestoreStrength
            | Potion1SubType::RestoreWisdom
            | Potion1SubType::SlowPoison,
        ) => Some(MapColor::ItemHealing),
        ItemSubType::Potion1(
            Potion1SubType::Blindness
            | Potion1SubType::Confusion
            | Potion1SubType::FleaBile
            | Potion1SubType::LoseIntelligence
            | Potion1SubType::LoseMemories
            | Potion1SubType::LoseWisdom
            | Potion1SubType::Poison
            | Potion1SubType::SaltWater
            | Potion1SubType::Sleep
            | Potion1SubType::Slowness
            | Potion1SubType::Ugliness,
        ) => Some(MapColor::ItemHarmful),
        ItemSubType::Potion1(
            Potion1SubType::AppleJuice | Potion1SubType::SlimeMoldJuice | Potion1SubType::Water,
        ) => Some(MapColor::ItemNature),
        ItemSubType::Potion1(_) => Some(MapColor::ItemArcane),
        ItemSubType::Scroll1(
            Scroll1SubType::AggravateMonster
            | Scroll1SubType::CurseArmor
            | Scroll1SubType::CurseWeapon
            | Scroll1SubType::SummonMonster
            | Scroll1SubType::SummonUndead
            | Scroll1SubType::TeleportLevel
            | Scroll1SubType::TrapCreation,
        ) => Some(MapColor::ItemHarmful),
        ItemSubType::Scroll1(
            Scroll1SubType::Destruction
            | Scroll1SubType::Genocide
            | Scroll1SubType::MassGenocide
            | Scroll1SubType::SleepMonster,
        ) => Some(MapColor::ItemCombat),
        ItemSubType::Scroll1(
            Scroll1SubType::Blessing
            | Scroll1SubType::HolyChant
            | Scroll1SubType::HolyPrayer
            | Scroll1SubType::ProtectionFromEvil
            | Scroll1SubType::RemoveCurse
            | Scroll1SubType::RuneOfProtection,
        ) => Some(MapColor::ItemDefense),
        ItemSubType::Scroll1(Scroll1SubType::CreateFood | Scroll1SubType::MakeMunchies) => {
            Some(MapColor::ItemNature)
        }
        ItemSubType::Scroll1(_) => Some(MapColor::ItemArcane),
        ItemSubType::Food(
            FoodSubType::MushroomOfCurePoison
            | FoodSubType::MushroomOfCureBlindness
            | FoodSubType::MushroomOfCureParanoia
            | FoodSubType::MushroomOfCureConfusion
            | FoodSubType::MushroomOfRestoreConstitution
            | FoodSubType::MushroomOfFirstAid
            | FoodSubType::MushroomOfMinorCures
            | FoodSubType::MushroomOfLightCures
            | FoodSubType::MushroomOfRestoring
            | FoodSubType::MushroomOfCurePoison2
            | FoodSubType::MushroomOfCureSeriousWounds,
        ) => Some(MapColor::ItemHealing),
        ItemSubType::Food(
            FoodSubType::MushroomOfPoison
            | FoodSubType::MushroomOfBlindness
            | FoodSubType::MushroomOfParanoia
            | FoodSubType::MushroomOfConfusion
            | FoodSubType::MushroomOfHallucination
            | FoodSubType::MushroomOfWeakness
            | FoodSubType::MushroomOfUnhealth
            | FoodSubType::MushroomOfPoison2
            | FoodSubType::MushroomOfHallucination2
            | FoodSubType::MushroomOfUnhealth2
            | FoodSubType::BerriesPoisonous,
        ) => Some(MapColor::ItemHarmful),
        ItemSubType::Food(_) | ItemSubType::JunkFood(_) => Some(MapColor::ItemNature),
        ItemSubType::Gem(
            GemSubType::GemOfDetectMonsters
            | GemSubType::GemOfDispelEvil
            | GemSubType::GemOfDarkness
            | GemSubType::GemOfAcidBalls
            | GemSubType::GemOfDetectInvisible
            | GemSubType::GemOfIdentify
            | GemSubType::GemOfLight
            | GemSubType::GemOfSummoning
            | GemSubType::GemOfRemoveCurse
            | GemSubType::GemOfAnnihilation
            | GemSubType::GemOfRecall,
        ) => Some(MapColor::ItemArcane),
        ItemSubType::WearableGem(WearableGemSubType::GemOfTeleportation) => {
            Some(MapColor::ItemHarmful)
        }
        ItemSubType::WearableGem(
            WearableGemSubType::GemOfResistCold
            | WearableGemSubType::GemOfResistAcid
            | WearableGemSubType::GemOfProtectFire,
        ) => Some(MapColor::ItemDefense),
        ItemSubType::WearableGem(
            WearableGemSubType::GemOfSeeInvisible | WearableGemSubType::GemOfStealth,
        ) => Some(MapColor::ItemArcane),
        ItemSubType::WearableGem(WearableGemSubType::GemOfSlowDigestion) => {
            Some(MapColor::ItemNature)
        }
        ItemSubType::Amulet(AmuletSubType::AmuletOfDoom | AmuletSubType::AmuletOfTeleportation) => {
            Some(MapColor::ItemHarmful)
        }
        ItemSubType::Amulet(AmuletSubType::AmuletOfResistAcid) => Some(MapColor::ItemDefense),
        ItemSubType::Amulet(AmuletSubType::AmuletOfSlowDigestion) => Some(MapColor::ItemNature),
        ItemSubType::Amulet(
            AmuletSubType::AmuletOfWisdom
            | AmuletSubType::AmuletOfCharisma
            | AmuletSubType::AmuletOfSearching
            | AmuletSubType::AmuletOfTheMagi,
        ) => Some(MapColor::ItemArcane),
        ItemSubType::Ring(
            RingSubType::RingOfTeleportation
            | RingSubType::RingOfWeakness
            | RingSubType::RingOfWoe
            | RingSubType::RingOfStupidity
            | RingSubType::RingOfAggravateMonsters,
        ) => Some(MapColor::ItemHarmful),
        ItemSubType::Ring(
            RingSubType::RingOfIncreaseDamage
            | RingSubType::RingOfIncreaseToHit
            | RingSubType::RingOfSlaying,
        ) => Some(MapColor::ItemCombat),
        ItemSubType::Ring(
            RingSubType::RingOfResistFire
            | RingSubType::RingOfResistCold
            | RingSubType::RingOfFeatherFalling
            | RingSubType::RingOfLordlyProtectionFire
            | RingSubType::RingOfLordlyProtectionAcid
            | RingSubType::RingOfLordlyProtectionCold
            | RingSubType::RingOfProtection
            | RingSubType::RingOfSustainStrength
            | RingSubType::RingOfSustainIntelligence
            | RingSubType::RingOfSustainWisdom
            | RingSubType::RingOfSustainConstitution
            | RingSubType::RingOfSustainDexterity
            | RingSubType::RingOfSustainCharisma,
        ) => Some(MapColor::ItemDefense),
        ItemSubType::Ring(
            RingSubType::RingOfGainStrength
            | RingSubType::RingOfGainDexterity
            | RingSubType::RingOfGainConstitution
            | RingSubType::RingOfGainIntelligence
            | RingSubType::RingOfSpeed1
            | RingSubType::RingOfSpeed2
            | RingSubType::RingOfSearching
            | RingSubType::RingOfSeeInvisible,
        ) => Some(MapColor::ItemArcane),
        ItemSubType::Ring(RingSubType::RingOfSlowDigestion) => Some(MapColor::ItemNature),
        ItemSubType::Wand(WandSubType::WandOfHealMonster) => Some(MapColor::ItemHealing),
        ItemSubType::Wand(
            WandSubType::WandOfLightningBolts
            | WandSubType::WandOfFrostBolts
            | WandSubType::WandOfFireBolts
            | WandSubType::WandOfDrainLife
            | WandSubType::WandOfMagicMissile
            | WandSubType::WandOfLightningBalls
            | WandSubType::WandOfColdBalls
            | WandSubType::WandOfFireBalls
            | WandSubType::WandOfStinkingCloud
            | WandSubType::WandOfAcidBalls,
        ) => Some(MapColor::ItemCombat),
        ItemSubType::Wand(
            WandSubType::WandOfHasteMonster
            | WandSubType::WandOfCloneMonster
            | WandSubType::WandOfPolymorph,
        ) => Some(MapColor::ItemHarmful),
        ItemSubType::Wand(_) => Some(MapColor::ItemArcane),
        ItemSubType::Sword(SwordSubType::BrokenSword) => Some(MapColor::ItemHarmful),
        ItemSubType::MiscUsable(MiscUsableSubType::IronSpike) => Some(MapColor::ItemCombat),
        ItemSubType::MiscUsable(
            MiscUsableSubType::SilverCross
            | MiscUsableSubType::GoldCross
            | MiscUsableSubType::MithrilCross
            | MiscUsableSubType::Cross,
        ) => Some(MapColor::ItemDefense),
        ItemSubType::MagicBook(_) | ItemSubType::SongBook(_) => Some(MapColor::ItemArcane),
        ItemSubType::PrayerBook(_) => Some(MapColor::ItemDefense),
        ItemSubType::Instrument(_) => Some(MapColor::ItemNature),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversion;
    use crate::model::item_subtype::{
        FoodSubType, GemSubType, Potion1SubType, RingSubType, Scroll1SubType, SwordSubType,
        WandSubType, WearableGemSubType,
    };

    fn item(item_type: ItemType, subval: i64) -> Item {
        Item {
            tval: item_type.into(),
            subval,
            ..Item::default()
        }
    }

    fn potion(subtype: Potion1SubType) -> Item {
        typed_item(ItemType::Potion1, ItemSubType::Potion1(subtype))
    }

    fn typed_item(item_type: ItemType, subtype: ItemSubType) -> Item {
        item(
            item_type,
            conversion::item_subtype::to_usize(&subtype) as i64,
        )
    }

    fn scroll_with_flags(flags: u64, flags2: u64) -> Item {
        let mut item = item(ItemType::Scroll1, 269);
        item.flags = flags;
        item.flags2 = flags2;
        item
    }

    fn sword(subtype: SwordSubType) -> Item {
        item(
            ItemType::Sword,
            conversion::item_subtype::to_usize(&ItemSubType::Sword(subtype)) as i64,
        )
    }

    #[test]
    fn subtype_semantics_resolve_to_one_role() {
        let cases = [
            (
                "healing potion",
                potion(Potion1SubType::Healing),
                MapColor::ItemHealing,
            ),
            (
                "poison potion",
                potion(Potion1SubType::Poison),
                MapColor::ItemHarmful,
            ),
            (
                "water potion",
                potion(Potion1SubType::Water),
                MapColor::ItemNature,
            ),
            (
                "curse scroll",
                typed_item(
                    ItemType::Scroll1,
                    ItemSubType::Scroll1(Scroll1SubType::CurseWeapon),
                ),
                MapColor::ItemHarmful,
            ),
            (
                "destruction scroll",
                typed_item(
                    ItemType::Scroll1,
                    ItemSubType::Scroll1(Scroll1SubType::Destruction),
                ),
                MapColor::ItemCombat,
            ),
            (
                "genocide scroll",
                typed_item(
                    ItemType::Scroll1,
                    ItemSubType::Scroll1(Scroll1SubType::Genocide),
                ),
                MapColor::ItemCombat,
            ),
            (
                "mass genocide scroll",
                typed_item(
                    ItemType::Scroll1,
                    ItemSubType::Scroll1(Scroll1SubType::MassGenocide),
                ),
                MapColor::ItemCombat,
            ),
            (
                "sleep monster scroll",
                scroll_with_flags(SCROLL_SLEEP_MONSTER_FLAG, 0),
                MapColor::ItemCombat,
            ),
            (
                "make munchies scroll",
                scroll_with_flags(0, SCROLL_MAKE_MUNCHIES_FLAG2),
                MapColor::ItemNature,
            ),
            (
                "trap door destruction scroll",
                typed_item(
                    ItemType::Scroll1,
                    ItemSubType::Scroll1(Scroll1SubType::TrapDoorDestruction),
                ),
                MapColor::ItemArcane,
            ),
            (
                "curative food",
                typed_item(
                    ItemType::Food,
                    ItemSubType::Food(FoodSubType::MushroomOfFirstAid),
                ),
                MapColor::ItemHealing,
            ),
            (
                "functional gem",
                typed_item(ItemType::Gem, ItemSubType::Gem(GemSubType::GemOfIdentify)),
                MapColor::ItemArcane,
            ),
            (
                "resistance ring",
                typed_item(
                    ItemType::Ring,
                    ItemSubType::Ring(RingSubType::RingOfResistFire),
                ),
                MapColor::ItemDefense,
            ),
            (
                "combat wand",
                typed_item(
                    ItemType::Wand,
                    ItemSubType::Wand(WandSubType::WandOfFireBalls),
                ),
                MapColor::ItemCombat,
            ),
        ];

        for (description, item, expected) in cases {
            assert_eq!(color(&item), expected, "{}", description);
        }
    }

    #[test]
    fn wearable_gem_subtypes_have_effect_colors() {
        let cases = [
            (
                WearableGemSubType::GemOfTeleportation,
                MapColor::ItemHarmful,
            ),
            (WearableGemSubType::GemOfResistCold, MapColor::ItemDefense),
            (WearableGemSubType::GemOfResistAcid, MapColor::ItemDefense),
            (WearableGemSubType::GemOfSeeInvisible, MapColor::ItemArcane),
            (WearableGemSubType::GemOfStealth, MapColor::ItemArcane),
            (WearableGemSubType::GemOfSlowDigestion, MapColor::ItemNature),
            (WearableGemSubType::GemOfProtectFire, MapColor::ItemDefense),
        ];

        for (subtype, expected) in cases {
            let item = typed_item(ItemType::WearableGem, ItemSubType::WearableGem(subtype));
            assert_eq!(color(&item), expected);
        }
    }

    #[test]
    fn wearable_gem_flags_override_quality_and_subtype() {
        let cases = [
            (
                WearableGemSubType::GemOfTeleportation,
                WornFlag1::RandomTeleportation as u64,
                MapColor::ItemHarmful,
            ),
            (
                WearableGemSubType::GemOfResistCold,
                WornFlag1::ResistCold as u64,
                MapColor::ItemDefense,
            ),
            (
                WearableGemSubType::GemOfResistAcid,
                WornFlag1::ResistAcid as u64,
                MapColor::ItemDefense,
            ),
            (
                WearableGemSubType::GemOfSeeInvisible,
                WornFlag1::SeeInvisible as u64,
                MapColor::ItemArcane,
            ),
            (
                WearableGemSubType::GemOfStealth,
                WornFlag1::Stealth as u64,
                MapColor::ItemArcane,
            ),
            (
                WearableGemSubType::GemOfSlowDigestion,
                WornFlag1::Regeneration as u64,
                MapColor::ItemArcane,
            ),
            (
                WearableGemSubType::GemOfProtectFire,
                WornFlag1::Regeneration as u64,
                MapColor::ItemArcane,
            ),
        ];

        for (subtype, flag, expected) in cases {
            let mut item = typed_item(ItemType::WearableGem, ItemSubType::WearableGem(subtype));
            item.flags = flag;
            item.p1 = 1;
            assert_eq!(color(&item), expected);
        }
    }

    #[test]
    fn equipment_quality_overrides_subtype_fallback() {
        let cases = [
            (1, 0, 0, 0, MapColor::ItemQualityGood),
            (0, -1, 0, 0, MapColor::ItemQualityBad),
            (0, 0, 1, 0, MapColor::ItemQualityGood),
            (0, 0, 0, -1, MapColor::ItemQualityBad),
        ];

        for (p1, tohit, todam, toac, expected) in cases {
            let mut item = sword(SwordSubType::BastardSword);
            item.p1 = p1;
            item.tohit = tohit;
            item.todam = todam;
            item.toac = toac;
            assert_eq!(color(&item), expected);
        }
    }

    #[test]
    fn special_attribute_combinations_use_precedence_over_quality() {
        let cases = [
            (
                WornFlag1::SlayDragon as u64 | WornFlag1::ResistFire as u64,
                0,
                MapColor::ItemCombat,
            ),
            (
                WornFlag1::ResistFire as u64 | WornFlag1::Stealth as u64,
                0,
                MapColor::ItemDefense,
            ),
            (
                WornFlag1::AggravateMonsters as u64 | WornFlag1::SlayDragon as u64,
                0,
                MapColor::ItemHarmful,
            ),
            (
                WornFlag1::Cursed as u64 | WornFlag1::SlayDragon as u64,
                WornFlag2::MagicProof as u64,
                MapColor::ItemHarmful,
            ),
            (
                0,
                WornFlag2::SlayDemon as u64 | WornFlag2::MagicProof as u64,
                MapColor::ItemCombat,
            ),
        ];

        for (flags, flags2, expected) in cases {
            let mut item = sword(SwordSubType::BastardSword);
            item.tohit = 2;
            item.flags = flags;
            item.flags2 = flags2;
            assert_eq!(color(&item), expected);
        }
    }

    #[test]
    fn subtype_fallback_and_identification_are_independent() {
        let mut item = potion(Potion1SubType::Water);
        assert_eq!(color(&item), MapColor::ItemNature);

        item.identified = 255;
        assert_eq!(color(&item), MapColor::ItemNature);

        item.subval = i64::MAX;
        assert_eq!(color(&item), MapColor::Potion);
    }

    #[test]
    fn unknown_subtypes_fall_back_to_each_item_types_family_role() {
        for raw_type in u8::MIN..=u8::MAX {
            let Ok(item_type) = ItemType::try_from(raw_type) else {
                continue;
            };
            let item = item(item_type, i64::MAX);
            let role = color(&item);
            assert_eq!(role, item_type::color(&item_type, i64::MAX), "{}", raw_type);
            assert!(MapColor::ALL.contains(&role), "{} => {:?}", raw_type, role);
        }
    }

    #[test]
    fn fixtures_keep_their_roles_and_glyph_attributes() {
        for (item_type, subval, expected) in [
            (ItemType::OpenDoor, 0, MapColor::Door),
            (ItemType::ClosedDoor, 0, MapColor::Door),
            (ItemType::UpStaircase, 0, MapColor::Stairs),
            (ItemType::DownStaircase, 0, MapColor::Stairs),
            (ItemType::UpSteepStaircase, 0, MapColor::Stairs),
            (ItemType::DownSteepStaircase, 0, MapColor::Stairs),
            (ItemType::Rubble, 0, MapColor::Rubble),
            (ItemType::SeenTrap, 1, MapColor::Trap),
            (ItemType::Money, 0, MapColor::Gold),
            (ItemType::EntranceToStore, 101, MapColor::ShopEntrance),
        ] {
            let item = item(item_type, subval);
            assert_eq!(color(&item), expected, "{item_type:?}");
            assert_eq!(
                glyph(&item),
                item_type::glyph(&item_type, subval),
                "{item_type:?} glyph changed"
            );
        }

        for item_type in [ItemType::UnseenTrap, ItemType::SecretDoor] {
            let item = item(item_type, 0);
            assert_eq!(glyph(&item), item_type::symbol(&item_type, 0));
        }
    }

    #[test]
    fn semantic_color_keys_follow_resolved_roles() {
        let healing = potion(Potion1SubType::Healing);
        let poison = potion(Potion1SubType::Poison);
        assert_eq!(color_key(&healing), MapColor::ItemHealing as i32);
        assert_ne!(color_key(&healing), color_key(&poison));
    }
}
