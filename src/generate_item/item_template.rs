use crate::conversion::item_subtype;
use crate::conversion::item_type;
use crate::data;
use crate::misc;
use crate::model::item_subtype::ItemSubType;
use crate::model::Currency;
use crate::model::Item;
use crate::model::ItemType;

pub trait ItemTemplate {
    fn create(&self) -> Item {
        Item {
            name: misc::rs2item_name(self.name()),
            tval: item_type::to_usize(self.item_type()) as u8,
            flags: self.flags1(),
            flags2: self.flags2(),
            p1: self.p1(),
            cost: self.cost() * data::currency::value(&Currency::Gold),
            subval: item_subtype::to_usize(self.subtype()) as i64,
            weight: self.weight(),
            number: self.number(),
            tohit: self.modifier_to_hit(),
            todam: self.modifier_to_damage(),
            ac: self.base_ac(),
            toac: self.modifier_to_ac(),
            damage: misc::rs2item_damage(self.damage()),
            level: self.item_level() as i8,
            identified: if self.is_identified() { 1 } else { 0 },
        }
    }

    fn name(&self) -> &str;
    fn item_type(&self) -> ItemType;
    fn flags1(&self) -> u64;
    fn flags2(&self) -> u64;
    fn p1(&self) -> i64;
    fn cost(&self) -> i64;
    fn subtype(&self) -> ItemSubType;
    fn weight(&self) -> u16;
    fn number(&self) -> u16;
    fn modifier_to_hit(&self) -> i16;
    fn modifier_to_damage(&self) -> i16;
    fn base_ac(&self) -> i16;
    fn modifier_to_ac(&self) -> i16;
    fn damage(&self) -> &str;
    fn item_level(&self) -> u8;
    fn is_identified(&self) -> bool;
}
