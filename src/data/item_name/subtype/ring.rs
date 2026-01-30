use std::borrow::Cow;

use crate::conversion::item_subtype;
use crate::data::item_name::helpers::{maybe_number_of, plural_s, toac_bonus};
use crate::identification;
use crate::model::item_subtype::{ItemSubType, RingSubType};
use crate::model::{Item, ItemType};

pub fn ring(item: &Item) -> String {
    let Some(ItemSubType::Ring(ring_subtype)) = item_subtype::from_i64(ItemType::Ring, item.subval) else {
        return "alien ring".to_string();
    };

    let ring_key = ItemSubType::Ring(ring_subtype);
    let known_type = identification::is_identified(ring_key) || item.is_identified();

    let mut parts: Vec<Cow<'static, str>> = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }

    parts.push(Cow::Borrowed("ring"));
    parts.push(plural_s(item));

    if !known_type {
        return parts.join("");
    }

    parts.push(Cow::Borrowed(ring_suffix(ring_subtype)));

    if item.is_identified() {
        parts.push(toac_bonus(item));
    }

    parts.join("")
}

fn ring_suffix(subtype: RingSubType) -> &'static str {
    match subtype {
        RingSubType::RingOfGainStrength => " of gain strength",
        RingSubType::RingOfGainDexterity => " of gain dexterity",
        RingSubType::RingOfGainConstitution => " of gain constitution",
        RingSubType::RingOfGainIntelligence => " of gain intelligence",
        RingSubType::RingOfSpeed1 | RingSubType::RingOfSpeed2 => " of speed",
        RingSubType::RingOfSearching => " of searching",
        RingSubType::RingOfTeleportation => " of teleportation",
        RingSubType::RingOfSlowDigestion => " of slow digestion",
        RingSubType::RingOfResistFire => " of resist fire",
        RingSubType::RingOfResistCold => " of resist cold",
        RingSubType::RingOfFeatherFalling => " of feather falling",
        RingSubType::RingOfAdornment1 | RingSubType::RingOfAdornment2 => " of adornment",
        RingSubType::RingOfWeakness => " of weakness",
        RingSubType::RingOfLordlyProtectionFire => " of lordly protection (FIRE)",
        RingSubType::RingOfLordlyProtectionAcid => " of lordly protection (ACID)",
        RingSubType::RingOfLordlyProtectionCold => " of lordly protection (COLD)",
        RingSubType::RingOfWoe => " of woe",
        RingSubType::RingOfStupidity => " of stupidity",
        RingSubType::RingOfIncreaseDamage => " of increase damage",
        RingSubType::RingOfIncreaseToHit => " of increase to-hit",
        RingSubType::RingOfProtection => " of protection",
        RingSubType::RingOfAggravateMonsters => " of aggravate monster",
        RingSubType::RingOfSeeInvisible => " of see invisible",
        RingSubType::RingOfSustainStrength => " of sustain strength",
        RingSubType::RingOfSustainIntelligence => " of sustain intelligence",
        RingSubType::RingOfSustainWisdom => " of sustain wisdom",
        RingSubType::RingOfSustainConstitution => " of sustain constitution",
        RingSubType::RingOfSustainDexterity => " of sustain dexterity",
        RingSubType::RingOfSustainCharisma => " of sustain charisma",
        RingSubType::RingOfSlaying => " of slaying",
        RingSubType::RingOfGnomekind => " of gnomekind",
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion;
    use crate::data::item_name::generate;
    use crate::identification;
    use crate::model::item_subtype::{ItemSubType, RingSubType};
    use crate::model::{Item, ItemType};
    use serial_test::serial;

    fn base_item() -> Item {
        let mut item = Item::default();
        item.tval = conversion::item_type::to_usize(ItemType::Ring) as u8;
        item.toac = 2;
        item
    }

    fn subval(t: RingSubType) -> i64 {
        conversion::item_subtype::to_usize(&ItemSubType::Ring(t)) as i64
    }

    #[test]
    #[serial]
    fn test_ring_unidentified_unknown_subtype() {
        let mut item = base_item();
        item.subval = subval(RingSubType::RingOfProtection);
        item.number = 1;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Ring(RingSubType::RingOfProtection), false);
        assert_eq!(generate(&item), "ring");
    }

    #[test]
    #[serial]
    fn test_ring_known_subtype_but_not_identified_hides_toac() {
        let mut item = base_item();
        item.subval = subval(RingSubType::RingOfProtection);
        item.number = 1;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Ring(RingSubType::RingOfProtection), true);
        assert_eq!(generate(&item), "ring of protection");

        // Avoid leaking global state.
        identification::set_identified(ItemSubType::Ring(RingSubType::RingOfProtection), false);
    }

    #[test]
    #[serial]
    fn test_ring_identified_shows_toac() {
        let mut item = base_item();
        item.subval = subval(RingSubType::RingOfProtection);
        item.number = 1;
        item.set_identified(true);

        assert_eq!(generate(&item), "ring of protection [+2]");
    }

    #[test]
    #[serial]
    fn test_ring_multiple_prefix() {
        let mut item = base_item();
        item.subval = subval(RingSubType::RingOfProtection);
        item.number = 2;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Ring(RingSubType::RingOfProtection), false);
        assert_eq!(generate(&item), "2 rings");
    }

    #[test]
    #[serial]
    fn test_ring_none_prefix() {
        let mut item = base_item();
        item.subval = subval(RingSubType::RingOfProtection);
        item.number = 0;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Ring(RingSubType::RingOfProtection), false);
        assert_eq!(generate(&item), "no more rings");
    }

    #[test]
    #[serial]
    fn test_ring_known_subtype_speed() {
        let mut item = base_item();
        item.subval = subval(RingSubType::RingOfSpeed1);
        item.number = 1;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Ring(RingSubType::RingOfSpeed1), true);
        assert_eq!(generate(&item), "ring of speed");

        // Avoid leaking global state.
        identification::set_identified(ItemSubType::Ring(RingSubType::RingOfSpeed1), false);
    }

    #[test]
    #[serial]
    fn test_ring_identified_woe_bonus() {
        let mut item = base_item();
        item.subval = subval(RingSubType::RingOfWoe);
        item.number = 1;
        item.set_identified(true);
        item.toac = -2;

        assert_eq!(generate(&item), "ring of woe [-2]");
    }
}
