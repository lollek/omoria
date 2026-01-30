use std::borrow::Cow;

use crate::conversion::item_subtype;
use crate::data::item_name::helpers::{maybe_number_of, plural_s};
use crate::identification;
use crate::model::item_subtype::{ChimeSubType, ItemSubType};
use crate::model::{Item, ItemType};

pub fn chime(item: &Item) -> String {
    let Some(ItemSubType::Chime(chime_subtype)) = item_subtype::from_i64(ItemType::Chime, item.subval) else {
        return "alien chime".to_string();
    };

    let chime_key = ItemSubType::Chime(chime_subtype);
    let known_type = identification::is_identified(chime_key) || item.is_identified();

    let mut parts: Vec<Cow<'static, str>> = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }

    if !known_type {
        parts.push(Cow::Borrowed("unknown chime"));
        parts.push(plural_s(item));
        return parts.join("");
    }

    parts.push(Cow::Borrowed("chime"));
    parts.push(plural_s(item));
    parts.push(Cow::Borrowed(chime_suffix(chime_subtype)));

    // Charge count should only be visible when the specific item is identified.
    if item.is_identified() {
        parts.push(Cow::from(format!(" ({} charges)", item.p1)));
    }

    parts.join("")
}

fn chime_suffix(subtype: ChimeSubType) -> &'static str {
    match subtype {
        ChimeSubType::ChimeOfLight => " of light",
        ChimeSubType::ChimeOfDetectDoorsStairs => " of detect doors/stairs",
        ChimeSubType::ChimeOfDetectTraps => " of detect traps",
        ChimeSubType::ChimeOfTeleportation => " of teleportation",
        ChimeSubType::ChimeOfThunderblast => " of thunderblasts",
        ChimeSubType::ChimeOfSummonMonster => " of summon monster",
        ChimeSubType::ChimeOfDisarming => " of disarming",
        ChimeSubType::ChimeOfAggravation => " of aggravation",
        ChimeSubType::ChimeOfSlowMonster => " of slow monster",
        ChimeSubType::ChimeOfSootheMonster => " of soothe monster",
        ChimeSubType::ChimeOfCureLightWound => " of cure light wound",
        ChimeSubType::ChimeOfChanging => " of changing",
        ChimeSubType::ChimeOfRemoveCurse => " of remove curse",
        ChimeSubType::ChimeOfCuring => " of curing",
        ChimeSubType::ChimeOfDispelEvil => " of dispel evil",
        ChimeSubType::ChimeOfDarkness => " of darkness",
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion;
    use crate::data::item_name::generate;
    use crate::identification;
    use crate::model::item_subtype::{ChimeSubType, ItemSubType};
    use crate::model::{Item, ItemType};
    use serial_test::serial;

    fn base_item() -> Item {
        let mut item = Item::default();
        item.tval = conversion::item_type::to_usize(ItemType::Chime) as u8;
        item
    }

    fn subval(t: ChimeSubType) -> i64 {
        conversion::item_subtype::to_usize(&ItemSubType::Chime(t)) as i64
    }

    #[test]
    #[serial]
    fn test_chime_unidentified_unknown_subtype() {
        let mut item = base_item();
        item.subval = subval(ChimeSubType::ChimeOfLight);
        item.number = 1;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Chime(ChimeSubType::ChimeOfLight), false);
        assert_eq!(generate(&item), "unknown chime");
    }

    #[test]
    #[serial]
    fn test_chime_known_subtype_but_not_identified() {
        let mut item = base_item();
        item.subval = subval(ChimeSubType::ChimeOfLight);
        item.number = 1;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Chime(ChimeSubType::ChimeOfLight), true);
        assert_eq!(generate(&item), "chime of light");

        // Avoid leaking global state.
        identification::set_identified(ItemSubType::Chime(ChimeSubType::ChimeOfLight), false);
    }

    #[test]
    #[serial]
    fn test_chime_identified() {
        let mut item = base_item();
        item.subval = subval(ChimeSubType::ChimeOfLight);
        item.number = 1;
        item.set_identified(true);

        assert_eq!(generate(&item), "chime of light (0 charges)");
    }

    #[test]
    #[serial]
    fn test_chime_multiple_prefix() {
        let mut item = base_item();
        item.subval = subval(ChimeSubType::ChimeOfLight);
        item.number = 2;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Chime(ChimeSubType::ChimeOfLight), false);
        assert_eq!(generate(&item), "2 unknown chimes");
    }

    #[test]
    #[serial]
    fn test_chime_none_prefix() {
        let mut item = base_item();
        item.subval = subval(ChimeSubType::ChimeOfLight);
        item.number = 0;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Chime(ChimeSubType::ChimeOfLight), false);
        assert_eq!(generate(&item), "no more unknown chimes");
    }

    #[test]
    #[serial]
    fn test_chime_identified_includes_charges_even_zero() {
        let mut item = base_item();
        item.subval = subval(ChimeSubType::ChimeOfLight);
        item.number = 1;
        item.p1 = 0;
        item.set_identified(true);

        assert_eq!(generate(&item), "chime of light (0 charges)");
    }

    #[test]
    #[serial]
    fn test_chime_identified_includes_charges() {
        let mut item = base_item();
        item.subval = subval(ChimeSubType::ChimeOfLight);
        item.number = 1;
        item.p1 = 3;
        item.set_identified(true);

        assert_eq!(generate(&item), "chime of light (3 charges)");
    }

    #[test]
    #[serial]
    fn test_chime_known_subtype_does_not_show_charges_when_not_identified() {
        let mut item = base_item();
        item.subval = subval(ChimeSubType::ChimeOfLight);
        item.number = 1;
        item.p1 = 3;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Chime(ChimeSubType::ChimeOfLight), true);
        assert_eq!(generate(&item), "chime of light");

        // Avoid leaking global state.
        identification::set_identified(ItemSubType::Chime(ChimeSubType::ChimeOfLight), false);
    }
}
