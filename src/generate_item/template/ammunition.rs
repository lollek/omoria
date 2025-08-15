use super::super::item_template::ItemTemplate;
use crate::generate_item::item_template::default_create;
use crate::generate_item::ItemQuality;
use crate::misc::rs2item_name;
use crate::model::{self, item_subtype::{ArrowSubType, BoltSubType, ItemSubType, SlingAmmoSubType}, Item, WornFlag1, WornFlag2};
use crate::random::randint;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AmmunitionTemplate {
    Arrow,
    Bolt,
    RoundedPebble,
    IronShot,
}

impl AmmunitionTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(AmmunitionTemplate::Arrow),
            Box::new(AmmunitionTemplate::Bolt),
            Box::new(AmmunitionTemplate::RoundedPebble),
            Box::new(AmmunitionTemplate::IronShot),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        AmmunitionTemplate::vec().into_iter()
    }

    fn apply_ammo_flame_tongue(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} of Fire", self.name()));
        item.apply_wornflag1(WornFlag1::SlayFire);
        item.tohit += 2;
        item.todam += 4;
        item.cost += 2_500;
    }

    fn apply_ammo_slaying(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} of Slaying", self.name()));
        item.tohit += 5;
        item.todam += 5;
        item.cost += 2_000;
    }

    fn apply_ammo_slay_dragon(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} of Dragon Slaying", self.name()));
        item.apply_wornflag1(WornFlag1::SlayDragon);
        item.tohit += 10;
        item.todam += 10;
        item.cost += 3_500;
    }

    fn apply_ammo_slay_evil(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} of Slay Evil", self.name()));
        item.apply_wornflag1(WornFlag1::SlayEvil);
        item.tohit += 3;
        item.todam += 3;
        item.cost += 2_500;
    }

    fn apply_ammo_slay_monster(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} of Slay Monster", self.name()));
        item.apply_wornflag1(WornFlag1::SlayMonster);
        item.tohit += 2;
        item.todam += 2;
        item.cost += 3_000;
    }
}

impl ItemTemplate for AmmunitionTemplate {
    fn create(&self, item_quality: ItemQuality, _item_level: u8) -> Item {
        let mut item = default_create(self, item_quality);
        if self == &AmmunitionTemplate::Arrow || self == &AmmunitionTemplate::Bolt {
            if item_quality == ItemQuality::Cursed {
                item.set_cursed(true);
                item.cost = 0;
                item.tohit = -randint(5) as i16;
                item.todam = -randint(5) as i16;
            } else if item_quality == ItemQuality::Magic {
                item.tohit = randint(4) as i16;
                item.todam = randint(4) as i16;
            } else if item_quality == ItemQuality::Special {
                item.tohit = randint(4) as i16;
                item.todam = randint(4) as i16;
                match randint(10) {
                    1..=3 => self.apply_ammo_slaying(&mut item),
                    4..=5 => self.apply_ammo_flame_tongue(&mut item),
                    6..=7 => self.apply_ammo_slay_evil(&mut item),
                    8..=9 => self.apply_ammo_slay_monster(&mut item),
                    10|_ => self.apply_ammo_slay_dragon(&mut item),
                }
            }
        }
        item
    }

    fn name(&self) -> &str {
        match self {
            AmmunitionTemplate::Arrow => "& Arrow~^ (%P2,%P3)",
            AmmunitionTemplate::Bolt => "& Bolt~^ (%P2,%P3)",
            AmmunitionTemplate::RoundedPebble => "& Rounded Pebble~^ (%P2,%P3)",
            AmmunitionTemplate::IronShot => "& Iron Shot~^ (%P2,%P3)",
        }
    }

    fn item_type(&self) -> model::ItemType {
        match self {
            AmmunitionTemplate::Arrow => model::ItemType::Arrow,
            AmmunitionTemplate::Bolt => model::ItemType::Bolt,
            AmmunitionTemplate::RoundedPebble => model::ItemType::SlingAmmo,
            AmmunitionTemplate::IronShot => model::ItemType::SlingAmmo,
        }
    }

    fn flags1(&self) -> u64 {
        0
    }
    fn flags2(&self) -> u64 {
        match self {
            AmmunitionTemplate::Arrow => WornFlag2::Sharp as u64,
            AmmunitionTemplate::Bolt => WornFlag2::Sharp as u64,
            AmmunitionTemplate::RoundedPebble => 0,
            AmmunitionTemplate::IronShot => 0,
        }
    }
    fn p1(&self) -> i64 {
        0
    }

    fn cost(&self) -> i64 {
        match self {
            AmmunitionTemplate::Arrow => 1,
            AmmunitionTemplate::Bolt => 1,
            AmmunitionTemplate::RoundedPebble => 1,
            AmmunitionTemplate::IronShot => 2,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            AmmunitionTemplate::Arrow => ItemSubType::Arrow(ArrowSubType::Arrow),
            AmmunitionTemplate::Bolt => ItemSubType::Bolt(BoltSubType::Bolt),
            AmmunitionTemplate::RoundedPebble => {
                ItemSubType::SlingAmmo(SlingAmmoSubType::RoundedPebble)
            }
            AmmunitionTemplate::IronShot => ItemSubType::SlingAmmo(SlingAmmoSubType::IronShot),
        }
    }

    fn weight(&self) -> u16 {
        match self {
            AmmunitionTemplate::Arrow => 2,
            AmmunitionTemplate::Bolt => 3,
            AmmunitionTemplate::RoundedPebble => 4,
            AmmunitionTemplate::IronShot => 5,
        }
    }

    fn number(&self) -> u16 {
        1
    }
    fn modifier_to_hit(&self) -> i16 {
        0
    }
    fn modifier_to_damage(&self) -> i16 {
        0
    }
    fn base_ac(&self) -> i16 {
        0
    }
    fn modifier_to_ac(&self) -> i16 {
        0
    }

    fn damage(&self) -> &str {
        match self {
            AmmunitionTemplate::Arrow => "3d4",
            AmmunitionTemplate::Bolt => "3d5",
            AmmunitionTemplate::RoundedPebble => "3d2",
            AmmunitionTemplate::IronShot => "3d3",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            AmmunitionTemplate::Arrow => 2,
            AmmunitionTemplate::Bolt => 3,
            AmmunitionTemplate::RoundedPebble => 0,
            AmmunitionTemplate::IronShot => 3,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
