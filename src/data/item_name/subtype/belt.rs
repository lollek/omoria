use crate::conversion::item_subtype;
use crate::model::{Item, ItemType};
use crate::model::item_subtype::{BeltSubType, ItemSubType};

pub fn belt(item: &Item) -> String {
    match item_subtype::from_i64(ItemType::Belt, item.subval) {
        Some(item_subtype) => match item_subtype {
            ItemSubType::Belt(belt_subtype) => match belt_subtype {
                BeltSubType::Sash => "sash",
                BeltSubType::LightBelt => "light belt",
                BeltSubType::Belt => "belt",
                BeltSubType::HeavyBelt => "heavy belt",
                BeltSubType::LightPlatedBelt => "light plated belt",
                BeltSubType::SharkskinBelt => "sharkskin belt",
                BeltSubType::DemonhideBelt => "demonhide belt",
                BeltSubType::WyrmhideBelt => "wyrmhide belt",
            }
            _ => "alien belt",
        }
        None => "alien belt",
    }.to_string()
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::template::BeltTemplate;

    #[test]
    fn test_names() {
        for (template, expected_name) in [
            (Box::new(BeltTemplate::Sash), "sash"),
            (Box::new(BeltTemplate::LightBelt), "light belt"),
            (Box::new(BeltTemplate::Belt), "belt"),
            (Box::new(BeltTemplate::HeavyBelt), "heavy belt"),
            (Box::new(BeltTemplate::LightPlatedBelt), "light plated belt"),
            (Box::new(BeltTemplate::SharkskinBelt), "sharkskin belt"),
            (Box::new(BeltTemplate::DemonhideBelt), "demonhide belt"),
            (Box::new(BeltTemplate::WyrmhideBelt), "wyrmhide belt"),
        ] {
            let item = generate_item::generate(template, 0);
            assert_eq!(generate(&item), expected_name);
        }
    }
}