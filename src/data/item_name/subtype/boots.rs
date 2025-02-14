use crate::conversion::item_subtype;
use crate::model::{Item, ItemType};
use crate::model::item_subtype::{BootsSubType, ItemSubType};

pub fn boots(item: &Item) -> String {
    match item_subtype::from_i64(ItemType::Boots, item.subval) {
        Some(item_subtype) => match item_subtype {
            ItemSubType::Boots(boots_subtype) => match boots_subtype {
                BootsSubType::SoftLeatherShoes => "soft leather shoes",
                BootsSubType::SoftLeatherBoots => "soft leather boots",
                BootsSubType::HardLeatherBoots => "hard leather boots",
                BootsSubType::Sandals => "sandals",
                BootsSubType::ChainBoots => "chain boots",
                BootsSubType::LightPlatedBoots => "light plated boots",
                BootsSubType::SharkskinBoots => "sharkskin boots",
                BootsSubType::DemonhideBoots => "demonhide boots",
                BootsSubType::WyrmhideBoot => "wyrmhide boots",
            }
            _ => "alien boots",
        }
        None => "alien boots",
    }.to_string()
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::template::{BootsTemplate, MiscUsableTemplate};

    #[test]
    fn test_names() {
        for (template, expected_name) in [
            (Box::new(BootsTemplate::SoftLeatherShoes), "soft leather shoes"),
            (Box::new(BootsTemplate::SoftLeatherBoots), "soft leather boots"),
            (Box::new(BootsTemplate::HardLeatherBoots), "hard leather boots"),
            (Box::new(BootsTemplate::Sandals), "sandals"),
            (Box::new(BootsTemplate::ChainBoots), "chain boots"),
            (Box::new(BootsTemplate::LightPlatedBoots), "light plated boots"),
            (Box::new(BootsTemplate::SharkskinBoots), "sharkskin boots"),
            (Box::new(BootsTemplate::DemonhideBoots), "demonhide boots"),
            (Box::new(BootsTemplate::WyrmhideBoot), "wyrmhide boots"),
        ] {
            let item = generate_item::generate(template, 0);
            assert_eq!(generate(&item), expected_name);
        }
    }
}