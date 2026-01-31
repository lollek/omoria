use crate::generate_item::item_template::default_create;
use crate::generate_item::ItemQuality;
use super::super::item_template::ItemTemplate;
use crate::model::{self, item_subtype::{CloakSubType, ItemSubType}, Item, WornFlag1};
use crate::rng::randint;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum CloakTemplate {
    LightCloak,
    HeavyCloak,
    SharkskinCloak,
    DemonhideCloak,
    WyrmhideCloak,
}

impl CloakTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(CloakTemplate::LightCloak),
            Box::new(CloakTemplate::HeavyCloak),
            Box::new(CloakTemplate::SharkskinCloak),
            Box::new(CloakTemplate::DemonhideCloak),
            Box::new(CloakTemplate::WyrmhideCloak),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        CloakTemplate::vec().into_iter()
    }
}

impl ItemTemplate for CloakTemplate {
    fn create(&self, item_quality: ItemQuality, _item_level: u8) -> Item {
        let mut item = default_create(self, item_quality);
        match item_quality {
            ItemQuality::Cursed => {
                match randint(3) {
                    1 => { // of Irritation
                        item.set_cursed(true);
                        item.apply_wornflag1(WornFlag1::AggravateMonsters);
                        item.ac = 0;
                        item.toac = -randint(1) as i16;
                        item.tohit = -randint(1) as i16;
                        item.todam = -randint(1) as i16;
                        item.cost = 0;
                    },
                    2 => {  // of Vulnerability
                        item.set_cursed(true);
                        item.ac = 0;
                        item.toac = -randint(10) as i16;
                        item.cost = 0;
                    },
                    3|_ => { // of Enveloping
                        item.set_cursed(true);
                        item.toac = -randint(1) as i16;
                        item.tohit = -1 -randint(3) as i16;
                        item.todam = -1 -randint(3) as i16;
                        item.cost = 0;
                    }
                }
            }
            ItemQuality::Magic => {
                item.toac = randint(2) as i16;
                item.cost += item.toac as i64 * 10_000;
            }
            ItemQuality::Special => {
                match randint(9) {
                    1..=4 => { // of Protection
                        item.toac += 1 + randint(3) as i16;
                        item.cost += 25_000 + item.toac as i64 * 10_000;
                    },
                    5..=8 => { // of Stealth
                        item.apply_wornflag1(WornFlag1::Stealth);
                        item.toac += 1 + randint(1) as i16;
                        item.p1 = randint(3);
                        item.cost += item.p1 * 50_000 + item.toac as i64 * 10_000
                    },
                    9|_ => { // of Elvenkind
                        item.apply_wornflag1(WornFlag1::GivesCharisma);
                        item.apply_wornflag1(WornFlag1::ResistStatDrain);
                        item.apply_wornflag1(WornFlag1::SeeInvisible);
                        item.apply_wornflag1(WornFlag1::Stealth);
                        item.p1 = 2;
                        item.cost += 200_000;
                    }
                }
            }
            _ => {}
        }
        item
    }

    fn name(&self) -> &str {
        match self {
            CloakTemplate::LightCloak => "Light Cloak^ [%P6,%P4]",
            CloakTemplate::HeavyCloak => "Heavy Cloak^ [%P6,%P4]",
            CloakTemplate::SharkskinCloak => "Sharkskin Cloak^ [%P6,%P4]",
            CloakTemplate::DemonhideCloak => "Demonhide Cloak^ [%P6,%P4]",
            CloakTemplate::WyrmhideCloak => "Wyrmhide Cloak^ [%P6,%P4]",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Cloak
    }

    fn flags1(&self) -> u64 {
        0
    }
    fn flags2(&self) -> u64 {
        0
    }
    fn p1(&self) -> i64 {
        0
    }

    fn cost(&self) -> i64 {
        match self {
            CloakTemplate::LightCloak => 3,
            CloakTemplate::HeavyCloak => 10,
            CloakTemplate::SharkskinCloak => 500,
            CloakTemplate::DemonhideCloak => 800,
            CloakTemplate::WyrmhideCloak => 1500,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            CloakTemplate::LightCloak => ItemSubType::Cloak(CloakSubType::LightCloak),
            CloakTemplate::HeavyCloak => ItemSubType::Cloak(CloakSubType::HeavyCloak),
            CloakTemplate::SharkskinCloak => ItemSubType::Cloak(CloakSubType::SharkskinCloak),
            CloakTemplate::DemonhideCloak => ItemSubType::Cloak(CloakSubType::DemonhideCloak),
            CloakTemplate::WyrmhideCloak => ItemSubType::Cloak(CloakSubType::WyrmhideCloak),
        }
    }

    fn weight(&self) -> u16 {
        match self {
            CloakTemplate::LightCloak => 10,
            CloakTemplate::HeavyCloak => 40,
            CloakTemplate::SharkskinCloak => 100,
            CloakTemplate::DemonhideCloak => 100,
            CloakTemplate::WyrmhideCloak => 100,
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
        match self {
            CloakTemplate::LightCloak => 1,
            CloakTemplate::HeavyCloak => 2,
            CloakTemplate::SharkskinCloak => 4,
            CloakTemplate::DemonhideCloak => 6,
            CloakTemplate::WyrmhideCloak => 8,
        }
    }

    fn modifier_to_ac(&self) -> i16 {
        0
    }
    fn damage(&self) -> &str {
        "0d0"
    }

    fn item_level(&self) -> u8 {
        match self {
            CloakTemplate::LightCloak => 0,
            CloakTemplate::HeavyCloak => 0,
            CloakTemplate::SharkskinCloak => 30,
            CloakTemplate::DemonhideCloak => 40,
            CloakTemplate::WyrmhideCloak => 50,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
