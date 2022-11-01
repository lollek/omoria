use crate::{
    data::item_name::generate,
    generate_item::{self, template::MiscTemplate},
};

#[test]
fn test_misc_object_type_rat_skeleton() {
    let mut item = generate_item::generate(Box::new(MiscTemplate::RatSkeleton), 0);
    assert_eq!(generate(&item), "rat skeleton");

    item.number = 0;
    assert_eq!(generate(&item), "no more rat skeleton");

    item.number = 5;
    assert_eq!(generate(&item), "rat skeleton");
}

#[test]
fn test_misc_object_type_giant_centipede_skeleton() {
    let mut item = generate_item::generate(Box::new(MiscTemplate::GiantCentipedeSkeleton), 0);
    assert_eq!(generate(&item), "giant centipede skeleton");

    item.number = 0;
    assert_eq!(generate(&item), "no more giant centipede skeleton");

    item.number = 5;
    assert_eq!(generate(&item), "giant centipede skeleton");
}

#[test]
fn test_misc_object_type_empty_bottle() {
    let mut item = generate_item::generate(Box::new(MiscTemplate::EmptyBottle), 0);
    assert_eq!(generate(&item), "empty bottle");

    item.number = 0;
    assert_eq!(generate(&item), "no more empty bottle");

    item.number = 5;
    assert_eq!(generate(&item), "empty bottle");
}

#[test]
fn test_misc_object_type_shards_of_pottery() {
    let mut item = generate_item::generate(Box::new(MiscTemplate::PotteryShard), 0);
    assert_eq!(generate(&item), "shards of pottery");

    item.number = 0;
    assert_eq!(generate(&item), "no more shards of pottery");

    item.number = 5;
    assert_eq!(generate(&item), "shards of pottery");
}

#[test]
fn test_misc_object_type_human_skeleton() {
    let mut item = generate_item::generate(Box::new(MiscTemplate::HumanSkeleton), 0);
    assert_eq!(generate(&item), "human skeleton");

    item.number = 0;
    assert_eq!(generate(&item), "no more human skeleton");

    item.number = 5;
    assert_eq!(generate(&item), "human skeleton");
}

#[test]
fn test_misc_object_type_dwarf_skeleton() {
    let mut item = generate_item::generate(Box::new(MiscTemplate::DwarfSkeleton), 0);
    assert_eq!(generate(&item), "dwarf skeleton");

    item.number = 0;
    assert_eq!(generate(&item), "no more dwarf skeleton");

    item.number = 5;
    assert_eq!(generate(&item), "dwarf skeleton");
}

#[test]
fn test_misc_object_type_elf_skeleton() {
    let mut item = generate_item::generate(Box::new(MiscTemplate::ElfSkeleton), 0);
    assert_eq!(generate(&item), "elf skeleton");

    item.number = 0;
    assert_eq!(generate(&item), "no more elf skeleton");

    item.number = 5;
    assert_eq!(generate(&item), "elf skeleton");
}

#[test]
fn test_misc_object_type_gnome_skeleton() {
    let mut item = generate_item::generate(Box::new(MiscTemplate::GnomeSkeleton), 0);
    assert_eq!(generate(&item), "gnome skeleton");

    item.number = 0;
    assert_eq!(generate(&item), "no more gnome skeleton");

    item.number = 5;
    assert_eq!(generate(&item), "gnome skeleton");
}

#[test]
fn test_misc_object_type_broken_set_of_teeth() {
    let mut item = generate_item::generate(Box::new(MiscTemplate::BrokenTeeth), 0);
    assert_eq!(generate(&item), "broken set of teeth");

    item.number = 0;
    assert_eq!(generate(&item), "no more broken set of teeth");

    item.number = 5;
    assert_eq!(generate(&item), "broken set of teeth");
}

#[test]
fn test_misc_object_type_large_broken_bone() {
    let mut item = generate_item::generate(Box::new(MiscTemplate::LargeBrokenBone), 0);
    assert_eq!(generate(&item), "large broken bone");

    item.number = 0;
    assert_eq!(generate(&item), "no more large broken bone");

    item.number = 5;
    assert_eq!(generate(&item), "large broken bone");
}

#[test]
fn test_misc_object_type_broken_stick() {
    let mut item = generate_item::generate(Box::new(MiscTemplate::BrokenStick), 0);
    assert_eq!(generate(&item), "broken stick");

    item.number = 0;
    assert_eq!(generate(&item), "no more broken stick");

    item.number = 5;
    assert_eq!(generate(&item), "broken stick");
}
