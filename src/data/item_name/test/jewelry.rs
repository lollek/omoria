use crate::{
    data::item_name::generate,
    generate_item::{self, template::ValuableTemplate},
};

#[test]
fn test_jewelry_small_gold_pendant() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::SmallGoldPendant), 0);
    assert_eq!(generate(&item), "small gold pendant");

    item.number = 0;
    assert_eq!(generate(&item), "no more small gold pendants");

    item.number = 5;
    assert_eq!(generate(&item), "5 small gold pendants");
}

#[test]
fn test_jewelry_small_mithril_pendant() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::SmallMithrilPendant), 0);
    assert_eq!(generate(&item), "small mithril pendant");

    item.number = 0;
    assert_eq!(generate(&item), "no more small mithril pendants");

    item.number = 5;
    assert_eq!(generate(&item), "5 small mithril pendants");
}

#[test]
fn test_jewelry_large_mithril_garter_belt() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::LargeMithrilGarterBelt), 0);
    assert_eq!(generate(&item), "large mithril garter belt");

    item.number = 0;
    assert_eq!(generate(&item), "no more large mithril garter belts");

    item.number = 5;
    assert_eq!(generate(&item), "5 large mithril garter belts");
}

#[test]
fn test_jewelry_small_silver_pendant() {
    let mut item = generate_item::generate(Box::new(ValuableTemplate::SmallSilverPendant), 0);
    assert_eq!(generate(&item), "small silver pendant");

    item.number = 0;
    assert_eq!(generate(&item), "no more small silver pendants");

    item.number = 5;
    assert_eq!(generate(&item), "5 small silver pendants");
}
