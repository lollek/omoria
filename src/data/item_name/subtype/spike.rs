use crate::conversion::item_subtype::from_i64;
use crate::data::item_name::helpers::{full_number_of, plural_s};
use crate::model::item_subtype::{ItemSubType, SpikeSubType};
use crate::model::Item;
use std::borrow::Cow;

pub fn spike(item: &Item) -> String {
    vec![
        full_number_of(item),
        match from_i64(item.item_type(), item.subval) {
            Some(subtype) => match subtype {
                ItemSubType::Spike(SpikeSubType::IronSpike) => {
                    Cow::from(format!("iron spike{}", plural_s(item)))
                }
                t => panic!("Expected, got {:?}", t),
            },
            None => Cow::from("alien item"),
        },
    ]
    .join("")
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::ItemQuality;
    use crate::generate_item::template::MiscUsableTemplate;

    #[test]
    fn test_spike() {
        let mut item = generate_item::generate(Box::new(MiscUsableTemplate::IronSpike), 0, ItemQuality::Normal);
        item.number = 0;
        assert_eq!(generate(&item), "no more iron spikes");

        item.number = 1;
        assert_eq!(generate(&item), "1 iron spike");

        item.number = 2;
        assert_eq!(generate(&item), "2 iron spikes");
    }
}
