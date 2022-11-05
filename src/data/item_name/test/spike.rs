use crate::{
    data::item_name::generate,
    generate_item::{self, template::MiscUsableTemplate},
};

#[test]
fn test_spike() {
    let mut item = generate_item::generate(Box::new(MiscUsableTemplate::IronSpike), 0);
    item.number = 0;
    assert_eq!(generate(&item), "no more iron spike");

    item.number = 1;
    assert_eq!(generate(&item), "1 iron spike");

    item.number = 2;
    assert_eq!(generate(&item), "2 iron spikes");
}
