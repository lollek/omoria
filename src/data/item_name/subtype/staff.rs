use std::borrow::Cow;

use crate::conversion::item_subtype;
use crate::data::item_name::helpers::{maybe_number_of, plural_s};
use crate::identification;
use crate::model::item_subtype::{ItemSubType, StaffSubType};
use crate::model::{Item, ItemType};

pub fn staff(item: &Item) -> String {
    let Some(ItemSubType::Staff(staff_subtype)) = item_subtype::from_i64(ItemType::Staff, item.subval)
    else {
        return "alien staff".to_string();
    };

    let staff_key = ItemSubType::Staff(staff_subtype);
    let known_type = identification::is_identified(staff_key) || item.is_identified();

    let mut parts: Vec<Cow<'static, str>> = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }

    if !known_type {
        parts.push(Cow::Borrowed("unknown staff"));
        parts.push(plural_s(item));
        return parts.join("");
    }

    parts.push(Cow::Borrowed("staff"));
    parts.push(plural_s(item));
    parts.push(Cow::Borrowed(staff_suffix(staff_subtype)));

    // Charge count should only be visible when the specific item is identified.
    if item.is_identified() {
        parts.push(Cow::from(format!(" ({} charges)", item.p1)));
    }

    parts.join("")
}

fn staff_suffix(subtype: StaffSubType) -> &'static str {
    match subtype {
        StaffSubType::StaffOfLight => " of light",
        StaffSubType::StaffOfDoorStairLocation => " of door/stair location",
        StaffSubType::StaffOfTrapLocation => " of trap location",
        StaffSubType::StaffOfTreasureLocation => " of treasure location",
        StaffSubType::StaffOfObjectLocation => " of object location",
        StaffSubType::StaffOfTeleportation => " of teleportation",
        StaffSubType::StaffOfEarthquakes => " of earthquakes",
        StaffSubType::StaffOfSummoning => " of summoning",
        StaffSubType::StaffOfDestruction => " of destruction",
        StaffSubType::StaffOfStarlite => " of starlite",
        StaffSubType::StaffOfHasteMonsters => " of haste monsters",
        StaffSubType::StaffOfSlowMonsters => " of slow monsters",
        StaffSubType::StaffOfSleepMonsters => " of sleep monsters",
        StaffSubType::StaffOfCureLightWounds => " of cure light wounds",
        StaffSubType::StaffOfDetectInvisible => " of detect invisible",
        StaffSubType::StaffOfSpeed => " of speed",
        StaffSubType::StaffOfSlowness => " of slowness",
        StaffSubType::StaffOfMassPolymorph => " of mass polymorph",
        StaffSubType::StaffOfRemoveCurse => " of remove curse",
        StaffSubType::StaffOfDetectEvil => " of detect evil",
        StaffSubType::StaffOfCuring => " of curing",
        StaffSubType::StaffOfDispelEvil => " of dispel evil",
        StaffSubType::StaffOfDarkness => " of darkness",
        StaffSubType::StaffOfIdentify => " of identify",
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion;
    use crate::data::item_name::generate;
    use crate::identification;
    use crate::model::item_subtype::{ItemSubType, StaffSubType};
    use crate::model::{Item, ItemType};
    use serial_test::serial;

    fn base_item() -> Item {
        let mut item = Item::default();
        item.tval = ItemType::Staff.into();
        item
    }

    fn subval(t: StaffSubType) -> i64 {
        conversion::item_subtype::to_usize(&ItemSubType::Staff(t)) as i64
    }

    #[test]
    #[serial]
    fn test_staff_unidentified_unknown_subtype() {
        let mut item = base_item();
        item.subval = subval(StaffSubType::StaffOfLight);
        item.number = 1;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Staff(StaffSubType::StaffOfLight), false);
        assert_eq!(generate(&item), "unknown staff");
    }

    #[test]
    #[serial]
    fn test_staff_known_subtype_but_not_identified() {
        let mut item = base_item();
        item.subval = subval(StaffSubType::StaffOfLight);
        item.number = 1;
        item.p1 = 3;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Staff(StaffSubType::StaffOfLight), true);
        assert_eq!(generate(&item), "staff of light");

        // Avoid leaking global state.
        identification::set_identified(ItemSubType::Staff(StaffSubType::StaffOfLight), false);
    }

    #[test]
    #[serial]
    fn test_staff_identified_includes_charges_even_zero() {
        let mut item = base_item();
        item.subval = subval(StaffSubType::StaffOfLight);
        item.number = 1;
        item.p1 = 0;
        item.set_identified(true);

        assert_eq!(generate(&item), "staff of light (0 charges)");
    }

    #[test]
    #[serial]
    fn test_staff_identified_includes_charges() {
        let mut item = base_item();
        item.subval = subval(StaffSubType::StaffOfLight);
        item.number = 1;
        item.p1 = 3;
        item.set_identified(true);

        assert_eq!(generate(&item), "staff of light (3 charges)");
    }

    #[test]
    #[serial]
    fn test_staff_multiple_prefix() {
        let mut item = base_item();
        item.subval = subval(StaffSubType::StaffOfLight);
        item.number = 2;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Staff(StaffSubType::StaffOfLight), false);
        assert_eq!(generate(&item), "2 unknown staffs");
    }

    #[test]
    #[serial]
    fn test_staff_none_prefix() {
        let mut item = base_item();
        item.subval = subval(StaffSubType::StaffOfLight);
        item.number = 0;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Staff(StaffSubType::StaffOfLight), false);
        assert_eq!(generate(&item), "no more unknown staffs");
    }
}
