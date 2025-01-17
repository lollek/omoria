use crate::{
    data::item_name::generate,
    generate_item::{self, template::ChestTemplate},
};

#[test]
fn test_chest_small_wooden_chest() {
    let mut item = generate_item::generate(Box::new(ChestTemplate::SmallWoodenChest), 0);
    assert_eq!(generate(&item), "small wooden chest");

    item.number = 0;
    assert_eq!(generate(&item), "no more small wooden chest");
}

#[test]
fn test_chest_large_wooden_chest() {
    let mut item = generate_item::generate(Box::new(ChestTemplate::LargeWoodenChest), 0);
    assert_eq!(generate(&item), "large wooden chest");

    item.number = 0;
    assert_eq!(generate(&item), "no more large wooden chest");
}

#[test]
fn test_chest_small_iron_chest() {
    let mut item = generate_item::generate(Box::new(ChestTemplate::SmallIronChest), 0);
    assert_eq!(generate(&item), "small iron chest");

    item.number = 0;
    assert_eq!(generate(&item), "no more small iron chest");
}

#[test]
fn test_chest_large_iron_chest() {
    let mut item = generate_item::generate(Box::new(ChestTemplate::LargeIronChest), 0);
    assert_eq!(generate(&item), "large iron chest");

    item.number = 0;
    assert_eq!(generate(&item), "no more large iron chest");
}

#[test]
fn test_chest_small_steel_chest() {
    let mut item = generate_item::generate(Box::new(ChestTemplate::SmallSteelChest), 0);
    assert_eq!(generate(&item), "small steel chest");

    item.number = 0;
    assert_eq!(generate(&item), "no more small steel chest");
}

#[test]
fn test_chest_large_steel_chest() {
    let mut item = generate_item::generate(Box::new(ChestTemplate::LargeSteelChest), 0);
    assert_eq!(generate(&item), "large steel chest");

    item.number = 0;
    assert_eq!(generate(&item), "no more large steel chest");
}
