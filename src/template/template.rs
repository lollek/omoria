use model;

pub trait Template {
    fn create(&self) -> model::Item;

    fn name(&self) -> &str;
    fn item_type(&self) -> model::ItemType;
    fn flags1(&self) -> i64;
    fn flags2(&self) -> i64;
    fn p1(&self) -> u64;
    fn cost(&self) -> i64;
    fn subtype(&self) -> i64;
    fn weight(&self) -> u16;
    fn number(&self) -> u16;
    fn modifier_to_hit(&self) -> u16;
    fn modifier_to_damage(&self) -> u16;
    fn base_ac(&self) -> u16;
    fn modifier_to_ac(&self) -> u16;
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
