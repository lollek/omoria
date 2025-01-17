use crate::{
    data::item_name::generate,
    generate_item::{self, template::BagTemplate},
    identification,
    model::item_subtype::{BagSubType, ItemSubType},
};

#[test]
fn test_bag_of_holding250() {
    let item = generate_item::generate(Box::new(BagTemplate::BagOfHolding250), 0);
    assert_eq!(generate(&item), "bag");

    identification::set_identified(ItemSubType::Bag(BagSubType::BagOfHolding250), true);
    assert_eq!(generate(&item), "bag of holding (250)");
}

#[test]
fn test_bag_of_holding500() {
    let item = generate_item::generate(Box::new(BagTemplate::BagOfHolding500), 0);
    assert_eq!(generate(&item), "bag");

    identification::set_identified(ItemSubType::Bag(BagSubType::BagOfHolding500), true);
    assert_eq!(generate(&item), "bag of holding (500)");
}

#[test]
fn test_bag_of_holding1000() {
    let item = generate_item::generate(Box::new(BagTemplate::BagOfHolding1000), 0);
    assert_eq!(generate(&item), "bag");

    identification::set_identified(ItemSubType::Bag(BagSubType::BagOfHolding1000), true);
    assert_eq!(generate(&item), "bag of holding (1000)");
}

#[test]
fn test_bag_of_holding1500() {
    let item = generate_item::generate(Box::new(BagTemplate::BagOfHolding1500), 0);
    assert_eq!(generate(&item), "bag");

    identification::set_identified(ItemSubType::Bag(BagSubType::BagOfHolding1500), true);
    assert_eq!(generate(&item), "bag of holding (1500)");
}

#[test]
fn test_bag_of_devouring() {
    let item = generate_item::generate(Box::new(BagTemplate::BagOfDevouring), 0);
    assert_eq!(generate(&item), "bag");

    identification::set_identified(ItemSubType::Bag(BagSubType::BagOfDevouring), true);
    assert_eq!(generate(&item), "bag of devouring");
}
