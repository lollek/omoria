use crate::{
    data::item_name::generate,
    generate_item::{self, template::ValuableTemplate},
    identification,
    model::item_subtype::{GemSubType, ItemSubType},
};

#[test]
fn test_gem_of_detect_monster() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::GemOfDetectMonsters), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(ItemSubType::Gem(GemSubType::GemOfDetectMonsters), true);
    assert_eq!(generate(&item), "gem of detect monsters");

    item.set_identified(true);
    item.p1 = 42;
    assert_eq!(generate(&item), "gem of detect monsters (42 charges)");
}

#[test]
fn test_gem_of_dispel_evil() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::GemOfDispelEvil), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(ItemSubType::Gem(GemSubType::GemOfDispelEvil), true);
    assert_eq!(generate(&item), "gem of dispel evil");

    item.set_identified(true);
    item.p1 = 42;
    assert_eq!(generate(&item), "gem of dispel evil (42 charges)");
}

#[test]
fn test_gem_of_darkness() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::GemOfDarkness), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(ItemSubType::Gem(GemSubType::GemOfDarkness), true);
    assert_eq!(generate(&item), "gem of darkness");

    item.set_identified(true);
    item.p1 = 42;
    assert_eq!(generate(&item), "gem of darkness (42 charges)");
}

#[test]
fn test_gem_of_acid_balls() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::GemOfAcidBalls), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(ItemSubType::Gem(GemSubType::GemOfAcidBalls), true);
    assert_eq!(generate(&item), "gem of acid balls");

    item.set_identified(true);
    item.p1 = 42;
    assert_eq!(generate(&item), "gem of acid balls (42 charges)");
}

#[test]
fn test_gem_of_detect_invisible() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::GemOfDetectInvisible), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(ItemSubType::Gem(GemSubType::GemOfDetectInvisible), true);
    assert_eq!(generate(&item), "gem of detect invisible");

    item.set_identified(true);
    item.p1 = 42;
    assert_eq!(generate(&item), "gem of detect invisible (42 charges)");
}

#[test]
fn test_gem_of_identify() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::GemOfIdentify), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(ItemSubType::Gem(GemSubType::GemOfIdentify), true);
    assert_eq!(generate(&item), "gem of identify");

    item.set_identified(true);
    item.p1 = 42;
    assert_eq!(generate(&item), "gem of identify (42 charges)");
}

#[test]
fn test_gem_of_light() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::GemOfLight), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(ItemSubType::Gem(GemSubType::GemOfLight), true);
    assert_eq!(generate(&item), "gem of light");

    item.set_identified(true);
    item.p1 = 42;
    assert_eq!(generate(&item), "gem of light (42 charges)");
}

#[test]
fn test_gem_of_summoning() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::GemOfSummoning), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(ItemSubType::Gem(GemSubType::GemOfSummoning), true);
    assert_eq!(generate(&item), "gem of summoning");

    item.set_identified(true);
    item.p1 = 42;
    assert_eq!(generate(&item), "gem of summoning (42 charges)");
}

#[test]
fn test_gem_of_remove_curse() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::GemOfRemoveCurse), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(ItemSubType::Gem(GemSubType::GemOfRemoveCurse), true);
    assert_eq!(generate(&item), "gem of remove curse");

    item.set_identified(true);
    item.p1 = 42;
    assert_eq!(generate(&item), "gem of remove curse (42 charges)");
}

#[test]
fn test_gem_of_annihilation() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::GemOfAnnihilation), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(ItemSubType::Gem(GemSubType::GemOfAnnihilation), true);
    assert_eq!(generate(&item), "gem of annihilation");

    item.set_identified(true);
    item.p1 = 42;
    assert_eq!(generate(&item), "gem of annihilation (42 charges)");
}

#[test]
fn test_gem_of_recall() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::GemOfRecall), 0);
    assert_eq!(generate(&item), "finely cut gem");

    identification::set_identified(ItemSubType::Gem(GemSubType::GemOfRecall), true);
    assert_eq!(generate(&item), "gem of recall");

    item.set_identified(true);
    item.p1 = 42;
    assert_eq!(generate(&item), "gem of recall (42 charges)");
}

#[test]
fn test_fine_agate() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::FineAgate), 0);
    assert_eq!(generate(&item), "finely cut agate");

    item.number = 3;
    assert_eq!(generate(&item), "3 finely cut agates");
}

#[test]
fn test_fine_diamond() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::FineDiamond), 0);
    assert_eq!(generate(&item), "finely cut diamond");

    item.number = 3;
    assert_eq!(generate(&item), "3 finely cut diamonds");
}

#[test]
fn test_rough_diamond() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::RoughDiamond), 0);
    assert_eq!(generate(&item), "roughly cut diamond");

    item.number = 3;
    assert_eq!(generate(&item), "3 roughly cut diamonds");
}

#[test]
fn test_rough_sapphire() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::RoughSapphire), 0);
    assert_eq!(generate(&item), "roughly cut sapphire");

    item.number = 3;
    assert_eq!(generate(&item), "3 roughly cut sapphires");
}

#[test]
fn test_fine_sapphire() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::FineSapphire), 0);
    assert_eq!(generate(&item), "finely cut sapphire");

    item.number = 3;
    assert_eq!(generate(&item), "3 finely cut sapphires");
}

#[test]
fn test_small_bag_of_opals() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::SmallBagOfOpals), 0);
    assert_eq!(generate(&item), "small bag of opals");

    item.number = 3;
    assert_eq!(generate(&item), "3 small bags of opals");
}

#[test]
fn test_small_bag_of_sapphires() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::SmallBagOfSapphires), 0);
    assert_eq!(generate(&item), "small bag of sapphires");

    item.number = 3;
    assert_eq!(generate(&item), "3 small bags of sapphires");
}

#[test]
fn test_small_pouch_of_diamonds() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::SmallPouchOfDiamonds), 0);
    assert_eq!(generate(&item), "small pouch of diamonds");

    item.number = 3;
    assert_eq!(generate(&item), "3 small pouches of diamonds");
}

#[test]
fn test_large_sack_of_pearls() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::LargeSackOfPearls), 0);
    assert_eq!(generate(&item), "large sack of pearls");

    item.number = 3;
    assert_eq!(generate(&item), "3 large sacks of pearls");
}

#[test]
fn test_large_sack_of_sapphires() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::LargeSackOfSapphires), 0);
    assert_eq!(generate(&item), "large sack of sapphires");

    item.number = 3;
    assert_eq!(generate(&item), "3 large sacks of sapphires");
}

#[test]
fn test_large_pouch_of_diamonds() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::LargePouchOfDiamonds), 0);
    assert_eq!(generate(&item), "large pouch of diamonds");

    item.number = 3;
    assert_eq!(generate(&item), "3 large pouches of diamonds");
}
