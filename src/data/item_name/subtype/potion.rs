use std::borrow::Cow;

use crate::conversion::item_subtype;
use crate::data::item_name::helpers::{maybe_number_of, plural_s};
use crate::identification;
use crate::model::item_subtype::{ItemSubType, Potion1SubType};
use crate::model::{Item, ItemType};

pub fn potion(item: &Item) -> String {
    let Some(ItemSubType::Potion1(potion_subtype)) = item_subtype::from_i64(ItemType::Potion1, item.subval) else {
        return "alien potion".to_string();
    };

    let potion_key = ItemSubType::Potion1(potion_subtype);
    let known_type = identification::is_identified(potion_key) || item.is_identified();

    let mut parts: Vec<Cow<'static, str>> = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }

    if !known_type {
        parts.push(Cow::Borrowed("unknown potion"));
        parts.push(plural_s(item));
        return parts.join("");
    }

    parts.push(Cow::Borrowed("potion"));
    parts.push(plural_s(item));
    parts.push(Cow::Borrowed(potion_suffix(potion_subtype)));

    parts.join("")
}

fn potion_suffix(subtype: Potion1SubType) -> &'static str {
    match subtype {
        Potion1SubType::GainStrength => " of gain strength",
        Potion1SubType::Poison => " of poison",
        Potion1SubType::RestoreStrength => " of restore strength",
        Potion1SubType::GainIntelligence => " of gain intelligence",
        Potion1SubType::LoseIntelligence => " of lose intelligence",
        Potion1SubType::RestoreIntelligence => " of restore intelligence",
        Potion1SubType::GainWisdom => " of gain wisdom",
        Potion1SubType::LoseWisdom => " of lose wisdom",
        Potion1SubType::RestoreWisdom => " of restore wisdom",
        Potion1SubType::Charisma => " of charisma",
        Potion1SubType::Ugliness => " of ugliness",
        Potion1SubType::RestoreCharisma => " of restore charisma",
        Potion1SubType::CureLightWounds => " of cure light wounds",
        Potion1SubType::CureSeriousWounds => " of cure serious wounds",
        Potion1SubType::CureCriticalWounds => " of cure critical wounds",
        Potion1SubType::Healing => " of healing",
        Potion1SubType::GainConstitution => " of gain constitution",
        Potion1SubType::GainExperience => " of gain experience",
        Potion1SubType::Sleep => " of sleep",
        Potion1SubType::Blindness => " of blindness",
        Potion1SubType::Confusion => " of confusion",
        Potion1SubType::HasteSelf => " of haste self",
        Potion1SubType::Slowness => " of slowness",
        Potion1SubType::SlimeMoldJuice => " of slime mold juice",
        Potion1SubType::AppleJuice => " of apple juice",
        Potion1SubType::Water => " of water",
        Potion1SubType::GainDexterity => " of gain dexterity",
        Potion1SubType::RestoreDexterity => " of restore dexterity",
        Potion1SubType::RestoreConstitution => " of restore constitution",
        Potion1SubType::Learning => " of learning",
        Potion1SubType::LoseMemories => " of lose memories",
        Potion1SubType::SaltWater => " of salt water",
        Potion1SubType::Invulnerability => " of invulnerability",
        Potion1SubType::Heroism => " of heroism",
        Potion1SubType::SuperHeroism => " of super heroism",
        Potion1SubType::Boldliness => " of boldliness",
        Potion1SubType::RestoreLifeLevels => " of restore life levels",
        Potion1SubType::ResistHeat => " of resist heat",
        Potion1SubType::ResistCold => " of resist cold",
        Potion1SubType::DetectInvisible => " of detect invisible",
        Potion1SubType::SlowPoison => " of slow poison",
        Potion1SubType::NeutralizePoison => " of neutralize poison",
        Potion1SubType::RestoreMana => " of restore mana",
        Potion1SubType::InfraVision => " of infra-vision",
        Potion1SubType::FleaBile => " of flea bile",
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion;
    use crate::data::item_name::generate;
    use crate::identification;
    use crate::model::item_subtype::{ItemSubType, Potion1SubType};
    use crate::model::{Item, ItemType};
    use serial_test::serial;

    fn base_item() -> Item {
        let mut item = Item::default();
        item.tval = ItemType::Potion1.into();
        item
    }

    fn subval(t: Potion1SubType) -> i64 {
        conversion::item_subtype::to_usize(&ItemSubType::Potion1(t)) as i64
    }

    #[test]
    #[serial]
    fn test_potion_unidentified_unknown_subtype() {
        let mut item = base_item();
        item.subval = subval(Potion1SubType::Healing);
        item.number = 1;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Potion1(Potion1SubType::Healing), false);
        assert_eq!(generate(&item), "unknown potion");
    }

    #[test]
    #[serial]
    fn test_potion_known_subtype_but_not_identified() {
        let mut item = base_item();
        item.subval = subval(Potion1SubType::Healing);
        item.number = 1;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Potion1(Potion1SubType::Healing), true);
        assert_eq!(generate(&item), "potion of healing");

        // Avoid leaking global state.
        identification::set_identified(ItemSubType::Potion1(Potion1SubType::Healing), false);
    }

    #[test]
    #[serial]
    fn test_potion_identified() {
        let mut item = base_item();
        item.subval = subval(Potion1SubType::Healing);
        item.number = 1;
        item.set_identified(true);

        assert_eq!(generate(&item), "potion of healing");
    }

    #[test]
    #[serial]
    fn test_potion_multiple_prefix() {
        let mut item = base_item();
        item.subval = subval(Potion1SubType::Healing);
        item.number = 2;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Potion1(Potion1SubType::Healing), false);
        assert_eq!(generate(&item), "2 unknown potions");
    }

    #[test]
    #[serial]
    fn test_potion_none_prefix() {
        let mut item = base_item();
        item.subval = subval(Potion1SubType::Healing);
        item.number = 0;
        item.set_identified(false);

        identification::set_identified(ItemSubType::Potion1(Potion1SubType::Healing), false);
        assert_eq!(generate(&item), "no more unknown potions");
    }
}
