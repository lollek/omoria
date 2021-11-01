use misc;
use model;

pub trait Template {
    fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: self.item_type() as u8,
            flags: self.flags1(),
            flags2: self.flags2(),
            p1: self.p1(),
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subtype(),
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
    fn item_type(&self) -> model::ItemType;
    fn flags1(&self) -> u64;
    fn flags2(&self) -> u64;
    fn p1(&self) -> i64;
    fn cost(&self) -> i64;
    fn subtype(&self) -> i64;
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

/*

use template;

pub enum Template {
    Ammo(template::AmmunitionTemplate),
    Amulet(template::AmuletTemplate),
    Armor(template::ArmorTemplate),
    Axe(template::AxeTemplate),
    Bag(template::BagTemplate),
    Belt(template::BeltTemplate),
    Boots(template::BootsTemplate),
    Bow(template::BowTemplate),
    Bracers(template::BracersTemplate),
    Chest(template::ChestTemplate),
    Chime(template::ChimeTemplate),
    Cloak(template::CloakTemplate),
    Crossbow(template::CrossbowTemplate),
    Dagger(template::DaggerTemplate),
    Food(template::FoodTemplate),
    Gloves(template::GlovesTemplate),
    Helm(template::HelmTemplate),
    Horn(template::HornTemplate),
    JunkFood(template::JunkFoodTemplate),
    LightSource(template::LightSourceTemplate),
    Mace(template::MaceTemplate),
    Misc(template::MiscTemplate),
    MiscUsable(template::MiscUsableTemplate),
    Pick(template::PickTemplate),
    Polearm(template::PolearmTemplate),
    Potion(template::PotionTemplate),
    Ring(template::RingTemplate),
    Scroll(template::ScrollTemplate),
    Shield(template::ShieldTemplate),
    Sling(template::SlingTemplate),
    Staff(template::StaffTemplate),
    Sword(template::SwordTemplate),
    Valuable(template::ValuableTemplate),
    Wand(template::WandTemplate),
}
*/
