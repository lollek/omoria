use misc;
use model;
use item_template;

pub trait ItemTemplate {
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

impl dyn ItemTemplate {
    pub fn from(item_type: model::ItemType, subval: i64) -> Box<dyn ItemTemplate> {
        match item_type {
            model::ItemType::MiscObject => item_template::MiscTemplate::from(subval),
            model::ItemType::Chest => item_template::ChestTemplate::from(subval),
            model::ItemType::MiscUsable => item_template::MiscUsableTemplate::from(subval),
            model::ItemType::Spike => item_template::MiscUsableTemplate::from(subval),
            model::ItemType::FlaskOfOil => item_template::MiscUsableTemplate::from(subval),
            model::ItemType::Jewelry => item_template::JewelryTemplate::from(subval),
            model::ItemType::Bag => item_template::BagTemplate::from(subval),
            model::ItemType::Gem => item_template::GemTemplate::from(subval),
            model::ItemType::WearableGem => item_template::WearableGemTemplate::from(subval),
            model::ItemType::SlingAmmo => item_template::AmmunitionTemplate::from(subval),
            model::ItemType::Bolt => item_template::AmmunitionTemplate::from(subval),
            model::ItemType::Arrow => item_template::AmmunitionTemplate::from(subval),
            model::ItemType::LightSource => item_template::LightSourceTemplate::from(subval),

            model::ItemType::Bow => item_template::BowTemplate::from(subval),
            model::ItemType::Crossbow => item_template::CrossbowTemplate::from(subval),
            model::ItemType::Sling => item_template::SlingTemplate::from(subval),

            model::ItemType::Axe => item_template::AxeTemplate::from(subval),
            model::ItemType::Polearm => item_template::PolearmTemplate::from(subval),
            model::ItemType::Dagger => item_template::DaggerTemplate::from(subval),
            model::ItemType::Sword => item_template::SwordTemplate::from(subval),
            model::ItemType::Pick => item_template::PickTemplate::from(subval),
            model::ItemType::Mace => item_template::MaceTemplate::from(subval),

            model::ItemType::Boots => item_template::BootsTemplate::from(subval),
            model::ItemType::Gloves => item_template::GlovesTemplate::from(subval),
            model::ItemType::Cloak => item_template::CloakTemplate::from(subval),
            model::ItemType::Helm => item_template::HelmTemplate::from(subval),
            model::ItemType::Shield => item_template::ShieldTemplate::from(subval),
            model::ItemType::HardArmor => item_template::HardArmorTemplate::from(subval),
            model::ItemType::SoftArmor => item_template::SoftArmorTemplate::from(subval),
            model::ItemType::Bracers => item_template::BracersTemplate::from(subval),
            model::ItemType::Belt => item_template::BeltTemplate::from(subval),

            model::ItemType::Amulet => item_template::AmuletTemplate::from(subval),
            model::ItemType::Ring => item_template::RingTemplate::from(subval),

            model::ItemType::Staff => item_template::StaffTemplate::from(subval),
            model::ItemType::Wand => item_template::WandTemplate::from(subval),

            model::ItemType::Scroll => item_template::ScrollTemplate::from(subval),
            model::ItemType::Potion => item_template::PotionTemplate::from(subval),
            model::ItemType::Food => item_template::FoodTemplate::from(subval),
            model::ItemType::JunkFood => item_template::JunkFoodTemplate::from(subval),

            model::ItemType::Chime => item_template::ChimeTemplate::from(subval),
            model::ItemType::Horn => item_template::HornTemplate::from(subval),

            model::ItemType::MagicBook => item_template::MagicBookTemplate::from(subval),
            model::ItemType::PrayerBook => item_template::PrayerBookTemplate::from(subval),
            model::ItemType::Instrument => item_template::InstrumentTemplate::from(subval),
            model::ItemType::SongBook => item_template::SongBookTemplate::from(subval),

            // Not Items, but yeah
            model::ItemType::LodgingAtInn => item_template::LodgingAtInnTemplate::from(subval),
            model::ItemType::Money => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::UnseenTrap => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::SeenTrap => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::Rubble => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::OpenDoor => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::ClosedDoor => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::UpStaircase => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::DownStaircase => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::SecretDoor => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::EntranceToStore => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::UpSteepStaircase => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::DownSteepStaircase => item_template::DungeonFeatureTemplate::from(subval),
            model::ItemType::Whirlpool => item_template::DungeonFeatureTemplate::from(subval),
        }
    }
}
