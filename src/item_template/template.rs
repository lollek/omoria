use misc;
use model;
use item_template;

pub trait ItemTemplate {
    fn create(&self) -> model::Item {
        model::Item {
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

impl dyn ItemTemplate {
    pub fn from(item_type: model::ItemType, subval: i64) -> Box<dyn ItemTemplate> {
        match item_type {
            model::ItemType::MiscObject(_) => item_template::MiscTemplate::from(subval),
            model::ItemType::Chest(_) => item_template::ChestTemplate::from(subval),
            model::ItemType::MiscUsable(_) => item_template::MiscUsableTemplate::from(subval),
            model::ItemType::Spike(_) => item_template::MiscUsableTemplate::from(subval),
            model::ItemType::FlaskOfOil(_) => item_template::MiscUsableTemplate::from(subval),
            model::ItemType::Jewelry(_) => item_template::JewelryTemplate::from(subval),
            model::ItemType::Bag(_) => item_template::BagTemplate::from(subval),
            model::ItemType::Gem(_) => item_template::GemTemplate::from(subval),
            model::ItemType::WearableGem(_) => item_template::WearableGemTemplate::from(subval),
            model::ItemType::SlingAmmo(_) => item_template::AmmunitionTemplate::from(subval),
            model::ItemType::Bolt(_) => item_template::AmmunitionTemplate::from(subval),
            model::ItemType::Arrow(_) => item_template::AmmunitionTemplate::from(subval),
            model::ItemType::LightSource(_) => item_template::LightSourceTemplate::from(subval),

            model::ItemType::Bow(_) => item_template::BowTemplate::from(subval),
            model::ItemType::Crossbow(_) => item_template::CrossbowTemplate::from(subval),
            model::ItemType::Sling(_) => item_template::SlingTemplate::from(subval),

            model::ItemType::Axe(_) => item_template::AxeTemplate::from(subval),
            model::ItemType::Polearm(_) => item_template::PolearmTemplate::from(subval),
            model::ItemType::Dagger(_) => item_template::DaggerTemplate::from(subval),
            model::ItemType::Sword(_) => item_template::SwordTemplate::from(subval),
            model::ItemType::Pick(_) => item_template::PickTemplate::from(subval),
            model::ItemType::Mace(_) => item_template::MaceTemplate::from(subval),

            model::ItemType::Boots(_) => item_template::BootsTemplate::from(subval),
            model::ItemType::Gloves(_) => item_template::GlovesTemplate::from(subval),
            model::ItemType::Cloak(_) => item_template::CloakTemplate::from(subval),
            model::ItemType::Helm(_) => item_template::HelmTemplate::from(subval),
            model::ItemType::Shield(_) => item_template::ShieldTemplate::from(subval),
            model::ItemType::HardArmor(_) => item_template::HardArmorTemplate::from(subval),
            model::ItemType::SoftArmor(_) => item_template::SoftArmorTemplate::from(subval),
            model::ItemType::Bracers(_) => item_template::BracersTemplate::from(subval),
            model::ItemType::Belt(_) => item_template::BeltTemplate::from(subval),

            model::ItemType::Amulet(_) => item_template::AmuletTemplate::from(subval),
            model::ItemType::Ring(_) => item_template::RingTemplate::from(subval),

            model::ItemType::Staff(_) => item_template::StaffTemplate::from(subval),
            model::ItemType::Wand(_) => item_template::WandTemplate::from(subval),

            model::ItemType::Scroll(_) => item_template::ScrollTemplate::from(subval),
            model::ItemType::Potion(_) => item_template::PotionTemplate::from(subval),
            model::ItemType::Food(_) => item_template::FoodTemplate::from(subval),
            model::ItemType::JunkFood(_) => item_template::JunkFoodTemplate::from(subval),

            model::ItemType::Chime(_) => item_template::ChimeTemplate::from(subval),
            model::ItemType::Horn(_) => item_template::HornTemplate::from(subval),

            model::ItemType::MagicBook(_) => item_template::MagicBookTemplate::from(subval),
            model::ItemType::PrayerBook(_) => item_template::PrayerBookTemplate::from(subval),
            model::ItemType::Instrument(_) => item_template::InstrumentTemplate::from(subval),
            model::ItemType::SongBook(_) => item_template::SongBookTemplate::from(subval),

            // Not Items, but yeah
            model::ItemType::LodgingAtInn(_) => item_template::LodgingAtInnTemplate::from(subval),
            model::ItemType::Money(_) => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::UnseenTrap(_) => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::SeenTrap(_) => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::Rubble(_) => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::OpenDoor(_) => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::ClosedDoor(_) => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::UpStaircase(_) => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::DownStaircase(_) => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::SecretDoor(_) => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::EntranceToStore(_) => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::UpSteepStaircase(_) => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::DownSteepStaircase(_) => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::Whirlpool(_) => item_template::DungeonFeatureTemplate::from(subval),
        }
    }
}
