use crate::conversion::item_subtype::from_i64;
use crate::data::item_name::helpers::no_more;
use crate::model::item_subtype::{ItemSubType, MiscObjectSubType};
use crate::model::{Item, ItemType};
use std::borrow::Cow;

pub fn misc_object(item: &Item) -> String {
    vec![
        no_more(item),
        Cow::Borrowed(match from_i64(ItemType::MiscObject, item.subval) {
            Some(subtype) => match subtype {
                ItemSubType::MiscObject(MiscObjectSubType::RatSkeleton) => "rat skeleton",
                ItemSubType::MiscObject(MiscObjectSubType::GiantCentipedeSkeleton) => {
                    "giant centipede skeleton"
                }
                ItemSubType::MiscObject(MiscObjectSubType::EmptyBottle) => "empty bottle",
                ItemSubType::MiscObject(MiscObjectSubType::PotteryShard) => "shards of pottery",
                ItemSubType::MiscObject(MiscObjectSubType::HumanSkeleton) => "human skeleton",
                ItemSubType::MiscObject(MiscObjectSubType::DwarfSkeleton) => "dwarf skeleton",
                ItemSubType::MiscObject(MiscObjectSubType::ElfSkeleton) => "elf skeleton",
                ItemSubType::MiscObject(MiscObjectSubType::GnomeSkeleton) => "gnome skeleton",
                ItemSubType::MiscObject(MiscObjectSubType::BrokenTeeth) => "broken set of teeth",
                ItemSubType::MiscObject(MiscObjectSubType::LargeBrokenBone) => "large broken bone",
                ItemSubType::MiscObject(MiscObjectSubType::BrokenStick) => "broken stick",
                t => panic!("Expected misc object, got {:?}", t),
            },
            None => "alien object",
        }),
    ]
    .join("")
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::ItemQuality;
    use crate::generate_item::template::MiscTemplate;

    #[test]
    fn test_misc_object_type_rat_skeleton() {
        let mut item = generate_item::generate(Box::new(MiscTemplate::RatSkeleton), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "rat skeleton");

        item.number = 0;
        assert_eq!(generate(&item), "no more rat skeleton");

        item.number = 5;
        assert_eq!(generate(&item), "rat skeleton");
    }

    #[test]
    fn test_misc_object_type_giant_centipede_skeleton() {
        let mut item = generate_item::generate(Box::new(MiscTemplate::GiantCentipedeSkeleton), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "giant centipede skeleton");

        item.number = 0;
        assert_eq!(generate(&item), "no more giant centipede skeleton");

        item.number = 5;
        assert_eq!(generate(&item), "giant centipede skeleton");
    }

    #[test]
    fn test_misc_object_type_empty_bottle() {
        let mut item = generate_item::generate(Box::new(MiscTemplate::EmptyBottle), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "empty bottle");

        item.number = 0;
        assert_eq!(generate(&item), "no more empty bottle");

        item.number = 5;
        assert_eq!(generate(&item), "empty bottle");
    }

    #[test]
    fn test_misc_object_type_shards_of_pottery() {
        let mut item = generate_item::generate(Box::new(MiscTemplate::PotteryShard), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "shards of pottery");

        item.number = 0;
        assert_eq!(generate(&item), "no more shards of pottery");

        item.number = 5;
        assert_eq!(generate(&item), "shards of pottery");
    }

    #[test]
    fn test_misc_object_type_human_skeleton() {
        let mut item = generate_item::generate(Box::new(MiscTemplate::HumanSkeleton), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "human skeleton");

        item.number = 0;
        assert_eq!(generate(&item), "no more human skeleton");

        item.number = 5;
        assert_eq!(generate(&item), "human skeleton");
    }

    #[test]
    fn test_misc_object_type_dwarf_skeleton() {
        let mut item = generate_item::generate(Box::new(MiscTemplate::DwarfSkeleton), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "dwarf skeleton");

        item.number = 0;
        assert_eq!(generate(&item), "no more dwarf skeleton");

        item.number = 5;
        assert_eq!(generate(&item), "dwarf skeleton");
    }

    #[test]
    fn test_misc_object_type_elf_skeleton() {
        let mut item = generate_item::generate(Box::new(MiscTemplate::ElfSkeleton), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "elf skeleton");

        item.number = 0;
        assert_eq!(generate(&item), "no more elf skeleton");

        item.number = 5;
        assert_eq!(generate(&item), "elf skeleton");
    }

    #[test]
    fn test_misc_object_type_gnome_skeleton() {
        let mut item = generate_item::generate(Box::new(MiscTemplate::GnomeSkeleton), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "gnome skeleton");

        item.number = 0;
        assert_eq!(generate(&item), "no more gnome skeleton");

        item.number = 5;
        assert_eq!(generate(&item), "gnome skeleton");
    }

    #[test]
    fn test_misc_object_type_broken_set_of_teeth() {
        let mut item = generate_item::generate(Box::new(MiscTemplate::BrokenTeeth), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "broken set of teeth");

        item.number = 0;
        assert_eq!(generate(&item), "no more broken set of teeth");

        item.number = 5;
        assert_eq!(generate(&item), "broken set of teeth");
    }

    #[test]
    fn test_misc_object_type_large_broken_bone() {
        let mut item = generate_item::generate(Box::new(MiscTemplate::LargeBrokenBone), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "large broken bone");

        item.number = 0;
        assert_eq!(generate(&item), "no more large broken bone");

        item.number = 5;
        assert_eq!(generate(&item), "large broken bone");
    }

    #[test]
    fn test_misc_object_type_broken_stick() {
        let mut item = generate_item::generate(Box::new(MiscTemplate::BrokenStick), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "broken stick");

        item.number = 0;
        assert_eq!(generate(&item), "no more broken stick");

        item.number = 5;
        assert_eq!(generate(&item), "broken stick");
    }
}
