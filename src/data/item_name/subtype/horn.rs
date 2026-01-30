use std::borrow::Cow;

use crate::conversion::item_subtype;
use crate::data::item_name::helpers::{maybe_number_of, plural_s};
use crate::identification;
use crate::model::item_subtype::{HornSubType, ItemSubType};
use crate::model::{Item, ItemType};

pub fn horn(item: &Item) -> String {
    let Some(ItemSubType::Horn(horn_subtype)) = item_subtype::from_i64(ItemType::Horn, item.subval) else {
        return "alien horn".to_string();
    };

    let horn_key = ItemSubType::Horn(horn_subtype);
    let known_type = identification::is_identified(horn_key) || item.is_identified();

    let mut parts: Vec<Cow<'static, str>> = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }

    if !known_type {
        parts.push(Cow::Borrowed("unknown horn"));
        parts.push(plural_s(item));
        return parts.join("");
    }

    parts.push(Cow::Borrowed("horn"));
    parts.push(plural_s(item));
    parts.push(Cow::Borrowed(horn_suffix(horn_subtype)));

    if item.is_identified() {
        parts.push(Cow::from(format!(" ({} charges)", item.p1)));
    }

    parts.join("")
}

fn horn_suffix(subtype: HornSubType) -> &'static str {
    match subtype {
        HornSubType::HornOfBubbles => " of bubbles",
        HornSubType::HornOfCalling => " of calling",
        HornSubType::HornOfSoftSounds => " of soft sounds",
        HornSubType::HornOfBlasting => " of blasting",
        HornSubType::HornOfCold => " of cold",
        HornSubType::HornOfHeat => " of heat",
        HornSubType::HornOfGas => " of gas",
        HornSubType::HornOfRecall => " of recall",
        HornSubType::HornOfChaos => " of chaos",
        HornSubType::HornOfGlue => " of glue",
        HornSubType::HornOfValhalla => " of valhalla",
        HornSubType::HornOfTritons => " of tritons",
        HornSubType::HornOfFog => " of fog",
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion;
    use crate::data::item_name::generate;
    use crate::identification;
    use crate::model::item_subtype::{HornSubType, ItemSubType};
    use crate::model::{Item, ItemType};
    use serial_test::serial;

    fn base_item() -> Item {
        let mut item = Item::default();
        item.tval = conversion::item_type::to_usize(ItemType::Horn) as u8;
        item
    }

    fn subval(t: HornSubType) -> i64 {
        conversion::item_subtype::to_usize(&ItemSubType::Horn(t)) as i64
    }

    #[test]
    #[serial]
    fn test_horn_unidentified_unknown_subtype() {
        let mut item = base_item();
        item.subval = subval(HornSubType::HornOfBubbles);
        item.number = 1;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Horn(HornSubType::HornOfBubbles), false);
        assert_eq!(generate(&item), "unknown horn");
    }

    #[test]
    #[serial]
    fn test_horn_known_subtype_but_not_identified() {
        let mut item = base_item();
        item.subval = subval(HornSubType::HornOfBubbles);
        item.number = 1;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Horn(HornSubType::HornOfBubbles), true);
        assert_eq!(generate(&item), "horn of bubbles");

        // Avoid leaking global state.
        identification::set_identified(ItemSubType::Horn(HornSubType::HornOfBubbles), false);
    }

    #[test]
    #[serial]
    fn test_horn_identified() {
        let mut item = base_item();
        item.subval = subval(HornSubType::HornOfBubbles);
        item.number = 1;
        item.set_identified(true);

        assert_eq!(generate(&item), "horn of bubbles");
    }

    #[test]
    #[serial]
    fn test_horn_multiple_prefix() {
        let mut item = base_item();
        item.subval = subval(HornSubType::HornOfBubbles);
        item.number = 2;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Horn(HornSubType::HornOfBubbles), false);
        assert_eq!(generate(&item), "2 unknown horns");
    }

    #[test]
    #[serial]
    fn test_horn_none_prefix() {
        let mut item = base_item();
        item.subval = subval(HornSubType::HornOfBubbles);
        item.number = 0;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Horn(HornSubType::HornOfBubbles), false);
        assert_eq!(generate(&item), "no more unknown horns");
    }

    #[test]
    #[serial]
    fn test_horn_identified_shows_charges() {
        let mut item = base_item();
        item.subval = subval(HornSubType::HornOfBubbles);
        item.number = 1;
        item.p1 = 3;
        item.set_identified(true);

        assert_eq!(generate(&item), "horn of bubbles (3 charges)");
    }

    #[test]
    #[serial]
    fn test_horn_known_subtype_does_not_show_charges_when_not_identified() {
        let mut item = base_item();
        item.subval = subval(HornSubType::HornOfBubbles);
        item.number = 1;
        item.p1 = 3;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Horn(HornSubType::HornOfBubbles), true);
        assert_eq!(generate(&item), "horn of bubbles");

        // Avoid leaking global state.
        identification::set_identified(ItemSubType::Horn(HornSubType::HornOfBubbles), false);
    }
}
