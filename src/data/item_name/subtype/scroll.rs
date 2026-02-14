use std::borrow::Cow;

use crate::conversion::item_subtype;
use crate::data::item_name::helpers::{maybe_number_of, plural_s};
use crate::identification;
use crate::model::item_subtype::{ItemSubType, Scroll1SubType};
use crate::model::{Item, ItemType};

pub fn scroll(item: &Item) -> String {
    let Some(ItemSubType::Scroll1(scroll_subtype)) =
        item_subtype::from_i64(ItemType::Scroll1, item.subval)
    else {
        return "alien scroll".to_string();
    };

    let scroll_key = ItemSubType::Scroll1(scroll_subtype);
    let known_type = identification::is_identified(scroll_key) || item.is_identified();

    let mut parts: Vec<Cow<'static, str>> = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }

    if !known_type {
        parts.push(Cow::Borrowed("unknown scroll"));
        parts.push(plural_s(item));
        return parts.join("");
    }

    parts.push(Cow::Borrowed("scroll"));
    parts.push(plural_s(item));
    parts.push(Cow::Borrowed(scroll_suffix(scroll_subtype)));

    parts.join("")
}

fn scroll_suffix(subtype: Scroll1SubType) -> &'static str {
    match subtype {
        Scroll1SubType::AggravateMonster => " of aggravate monster",
        Scroll1SubType::Blessing => " of blessing",
        Scroll1SubType::CreateFood => " of create food",
        Scroll1SubType::CurseArmor => " of curse armor",
        Scroll1SubType::CurseWeapon => " of curse weapon",
        Scroll1SubType::Darkness => " of darkness",
        Scroll1SubType::Destruction => " of destruction",
        Scroll1SubType::DetectInvisible => " of detect invisible",
        Scroll1SubType::DispelUndead => " of dispel undead",
        Scroll1SubType::DoorCreation => " of door creation",
        Scroll1SubType::DoorStairLocation => " of door/stair location",
        Scroll1SubType::EnchantArmor => " of enchant armor",
        Scroll1SubType::EnchantWeapon => " of enchant weapon",
        Scroll1SubType::EnchantWeaponToDam => " of enchant weapon to-dam",
        Scroll1SubType::EnchantWeaponToHit => " of enchant weapon to-hit",
        Scroll1SubType::FeignDeath => " of feign death",
        Scroll1SubType::Genocide => " of genocide",
        Scroll1SubType::HolyChant => " of holy chant",
        Scroll1SubType::HolyPrayer => " of holy prayer",
        Scroll1SubType::Identify => " of identify",
        Scroll1SubType::Light => " of light",
        Scroll1SubType::MagicMapping => " of magic mapping",
        Scroll1SubType::MakeMunchies => " of make munchies",
        Scroll1SubType::MassGenocide => " of mass genocide",
        Scroll1SubType::MonsterConfusion => " of monster confusion",
        Scroll1SubType::ObjectDetection => " of object detection",
        Scroll1SubType::PhaseDoor => " of phase door",
        Scroll1SubType::ProtectionFromEvil => " of protection from evil",
        Scroll1SubType::Recharging => " of recharging",
        Scroll1SubType::RemoveCurse => " of remove curse",
        Scroll1SubType::RuneOfProtection => " of rune of protection",
        Scroll1SubType::SleepMonster => " of sleep monster",
        Scroll1SubType::SummonMonster => " of summon monster",
        Scroll1SubType::SummonUndead => " of summon undead",
        Scroll1SubType::Teleport => " of teleport",
        Scroll1SubType::TeleportLevel => " of teleport level",
        Scroll1SubType::TrapCreation => " of trap creation",
        Scroll1SubType::TrapDetection => " of trap detection",
        Scroll1SubType::TrapDoorDestruction => " of trap/door destruction",
        Scroll1SubType::TreasureDetection => " of treasure detection",
        Scroll1SubType::Wishing => " of wishing",
        Scroll1SubType::WordOfRecall => " of word-of-recall",
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion;
    use crate::data::item_name::generate;
    use crate::identification;
    use crate::model::item_subtype::{ItemSubType, Scroll1SubType};
    use crate::model::{Item, ItemType};
    use serial_test::serial;

    fn base_item() -> Item {
        let mut item = Item::default();
        item.tval = ItemType::Scroll1.into();
        item
    }

    fn subval(t: Scroll1SubType) -> i64 {
        conversion::item_subtype::to_usize(&ItemSubType::Scroll1(t)) as i64
    }

    #[test]
    #[serial]
    fn test_scroll_unidentified_unknown_subtype() {
        let mut item = base_item();
        item.subval = subval(Scroll1SubType::RemoveCurse);
        item.number = 1;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Scroll1(Scroll1SubType::RemoveCurse), false);
        assert_eq!(generate(&item), "unknown scroll");
    }

    #[test]
    #[serial]
    fn test_scroll_known_subtype_but_not_identified() {
        let mut item = base_item();
        item.subval = subval(Scroll1SubType::RemoveCurse);
        item.number = 1;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Scroll1(Scroll1SubType::RemoveCurse), true);
        assert_eq!(generate(&item), "scroll of remove curse");

        // Avoid leaking global state.
        identification::set_identified(ItemSubType::Scroll1(Scroll1SubType::RemoveCurse), false);
    }

    #[test]
    #[serial]
    fn test_scroll_identified() {
        let mut item = base_item();
        item.subval = subval(Scroll1SubType::RemoveCurse);
        item.number = 1;
        item.set_identified(true);

        assert_eq!(generate(&item), "scroll of remove curse");
    }

    #[test]
    #[serial]
    fn test_scroll_multiple_prefix() {
        let mut item = base_item();
        item.subval = subval(Scroll1SubType::RemoveCurse);
        item.number = 2;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Scroll1(Scroll1SubType::RemoveCurse), false);
        assert_eq!(generate(&item), "2 unknown scrolls");
    }

    #[test]
    #[serial]
    fn test_scroll_none_prefix() {
        let mut item = base_item();
        item.subval = subval(Scroll1SubType::RemoveCurse);
        item.number = 0;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Scroll1(Scroll1SubType::RemoveCurse), false);
        assert_eq!(generate(&item), "no more unknown scrolls");
    }
}
