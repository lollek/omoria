use crate::conversion::item_subtype::from_i64;
use crate::data::item_name::helpers::maybe_number_of;
use crate::model::{Item, ItemType};
use crate::{
    identification,
    model::item_subtype::{ItemSubType, WearableGemSubType},
};
use std::borrow::Cow;

pub fn wearable_gem(item: &Item) -> String {
    let subtype = from_i64(ItemType::WearableGem, item.subval)
        .unwrap_or_else(|| panic!("Subtype for item is not a wearable gem? {:?}", item));

    let mut parts = vec![];

    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }
    parts.push(Cow::Borrowed(if identification::is_identified(subtype) {
        "gem"
    } else {
        "finely cut gem"
    }));

    if identification::is_identified(subtype) {
        parts.push(Cow::from(match subtype {
            ItemSubType::WearableGem(WearableGemSubType::GemOfTeleportation) => " of teleportation",
            ItemSubType::WearableGem(WearableGemSubType::GemOfResistCold) => " of resist cold",
            ItemSubType::WearableGem(WearableGemSubType::GemOfResistAcid) => " of resist acid",
            ItemSubType::WearableGem(WearableGemSubType::GemOfSeeInvisible) => " of see invisible",
            ItemSubType::WearableGem(WearableGemSubType::GemOfStealth) => " of stealth",
            ItemSubType::WearableGem(WearableGemSubType::GemOfSlowDigestion) => {
                " of slow digestion"
            }
            ItemSubType::WearableGem(WearableGemSubType::GemOfProtectFire) => {
                " of lordly protection (FIRE)"
            }
            _ => "",
        }))
    }

    parts.join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::ItemQuality;
    use crate::generate_item::template::ValuableTemplate;

    #[test]
    fn test_gem_of_teleportation() {
        let item = generate_item::generate(Box::new(ValuableTemplate::GemOfTeleportation), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "finely cut gem");

        identification::set_identified(
            ItemSubType::WearableGem(WearableGemSubType::GemOfTeleportation),
            true,
        );
        assert_eq!(generate(&item), "gem of teleportation");
    }

    #[test]
    fn test_gem_of_resist_cold() {
        let item = generate_item::generate(Box::new(ValuableTemplate::GemOfResistCold), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "finely cut gem");

        identification::set_identified(
            ItemSubType::WearableGem(WearableGemSubType::GemOfResistCold),
            true,
        );
        assert_eq!(generate(&item), "gem of resist cold");
    }

    #[test]
    fn test_gem_of_resist_acid() {
        let item = generate_item::generate(Box::new(ValuableTemplate::GemOfResistAcid), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "finely cut gem");

        identification::set_identified(
            ItemSubType::WearableGem(WearableGemSubType::GemOfResistAcid),
            true,
        );
        assert_eq!(generate(&item), "gem of resist acid");
    }

    #[test]
    fn test_gem_of_see_invisible() {
        let item = generate_item::generate(Box::new(ValuableTemplate::GemOfSeeInvisible), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "finely cut gem");

        identification::set_identified(
            ItemSubType::WearableGem(WearableGemSubType::GemOfSeeInvisible),
            true,
        );
        assert_eq!(generate(&item), "gem of see invisible");
    }

    #[test]
    fn test_gem_of_stealth() {
        let item = generate_item::generate(Box::new(ValuableTemplate::GemOfStealth), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "finely cut gem");

        identification::set_identified(
            ItemSubType::WearableGem(WearableGemSubType::GemOfStealth),
            true,
        );
        assert_eq!(generate(&item), "gem of stealth");
    }

    #[test]
    fn test_gem_of_slow_digestion() {
        let item = generate_item::generate(Box::new(ValuableTemplate::GemOfSlowDigestion), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "finely cut gem");

        identification::set_identified(
            ItemSubType::WearableGem(WearableGemSubType::GemOfSlowDigestion),
            true,
        );
        assert_eq!(generate(&item), "gem of slow digestion");
    }

    #[test]
    fn test_gem_of_lordly_protection_fire() {
        let item = generate_item::generate(Box::new(ValuableTemplate::GemOfProtectFire), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "finely cut gem");

        identification::set_identified(
            ItemSubType::WearableGem(WearableGemSubType::GemOfProtectFire),
            true,
        );
        assert_eq!(generate(&item), "gem of lordly protection (FIRE)");
    }
}
