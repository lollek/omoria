use crate::{
    data::item_name::generate,
    generate_item::{self, template::MiscUsableTemplate},
};

#[test]
fn test_misc_usable_statue() {
    let mut item = generate_item::generate(Box::new(MiscUsableTemplate::Statue), 0);
    assert_eq!(generate(&item), "statue");

    item.number = 0;
    assert_eq!(generate(&item), "no more statue");
}

#[test]
fn test_misc_usable_silver_cross() {
    let mut item = generate_item::generate(Box::new(MiscUsableTemplate::SilverCross), 0);
    assert_eq!(generate(&item), "silver cross");

    item.number = 0;
    assert_eq!(generate(&item), "no more silver cross");
}

#[test]
fn test_misc_usable_gold_cross() {
    let mut item = generate_item::generate(Box::new(MiscUsableTemplate::GoldCross), 0);
    assert_eq!(generate(&item), "gold cross");

    item.number = 0;
    assert_eq!(generate(&item), "no more gold cross");
}

#[test]
fn test_misc_usable_mithril_cross() {
    let mut item = generate_item::generate(Box::new(MiscUsableTemplate::MithrilCross), 0);
    assert_eq!(generate(&item), "mithril cross");

    item.number = 0;
    assert_eq!(generate(&item), "no more mithril cross");
}

#[test]
fn test_misc_usable_cross() {
    let mut item = generate_item::generate(Box::new(MiscUsableTemplate::Cross), 0);
    assert_eq!(generate(&item), "cross");

    item.number = 0;
    assert_eq!(generate(&item), "no more cross");
}

#[test]
fn test_misc_usable_corked_bottle() {
    let mut item = generate_item::generate(Box::new(MiscUsableTemplate::CorkedBottle), 0);
    assert_eq!(generate(&item), "corked bottle");

    item.number = 0;
    assert_eq!(generate(&item), "no more corked bottle");
}
