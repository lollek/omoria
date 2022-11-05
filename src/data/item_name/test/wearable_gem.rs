use crate::{
    data::item_name::generate,
    generate_item::{self, template::ValuableTemplate},
    identification,
    model::item_subtype::{ItemSubType, WearableGemSubType},
};

#[test]
fn test_gem_of_teleportation() {
    let item = generate_item::generate(Box::new(ValuableTemplate::GemOfTeleportation), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(
        ItemSubType::WearableGem(WearableGemSubType::GemOfTeleportation),
        true,
    );
    assert_eq!(generate(&item), "gem of teleportation");
}

#[test]
fn test_gem_of_resist_cold() {
    let item = generate_item::generate(Box::new(ValuableTemplate::GemOfResistCold), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(
        ItemSubType::WearableGem(WearableGemSubType::GemOfResistCold),
        true,
    );
    assert_eq!(generate(&item), "gem of resist cold");
}

#[test]
fn test_gem_of_resist_acid() {
    let item = generate_item::generate(Box::new(ValuableTemplate::GemOfResistAcid), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(
        ItemSubType::WearableGem(WearableGemSubType::GemOfResistAcid),
        true,
    );
    assert_eq!(generate(&item), "gem of resist acid");
}

#[test]
fn test_gem_of_see_invisible() {
    let item = generate_item::generate(Box::new(ValuableTemplate::GemOfSeeInvisible), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(
        ItemSubType::WearableGem(WearableGemSubType::GemOfSeeInvisible),
        true,
    );
    assert_eq!(generate(&item), "gem of see invisible");
}

#[test]
fn test_gem_of_stealth() {
    let item = generate_item::generate(Box::new(ValuableTemplate::GemOfStealth), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(
        ItemSubType::WearableGem(WearableGemSubType::GemOfStealth),
        true,
    );
    assert_eq!(generate(&item), "gem of stealth");
}

#[test]
fn test_gem_of_slow_digestion() {
    let item = generate_item::generate(Box::new(ValuableTemplate::GemOfSlowDigestion), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(
        ItemSubType::WearableGem(WearableGemSubType::GemOfSlowDigestion),
        true,
    );
    assert_eq!(generate(&item), "gem of slow digestion");
}

#[test]
fn test_gem_of_lordly_protection_fire() {
    let item = generate_item::generate(Box::new(ValuableTemplate::GemOfProtectFire), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(
        ItemSubType::WearableGem(WearableGemSubType::GemOfProtectFire),
        true,
    );
    assert_eq!(generate(&item), "gem of lordly protection (FIRE)");
}
