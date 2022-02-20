use libc;
use model;
use item_template;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Item { // treasure_type
    // Object type. Literally, is what kind of object it is.
    pub tval: libc::uint8_t,

    // Flags define an item's properties. the meaning of flag values can
    // depend on the item's tval.  note that the original game designers
    // decided to use bitwise flags (a good design), but came up with more
    // than 32 different properties for some item types (e.g.  potions)
    // and so they had to give items a second flag member to hold them all.
    pub flags2: libc::uint64_t,
    pub flags: libc::uint64_t,

    // A catch-all member for associating some value of interest
    // with a particular item.  e.g. for wands, staves, gems,
    // it's the number of charges, while for certain magical
    // weapons, it can be a damage multiplier
    pub p1: libc::int64_t,

    // The item's nominal cost in iron pieces.
    pub cost: libc::int64_t,

    // A sub-category value.  the semantics of subval also depend
    // in part on the value of tval.
    pub subval: libc::int64_t,

    // Weight in some ill-defined unit, possibly gold pieces, but
    // possibly in fifths of a gold piece.
    pub weight: libc::uint16_t,

    // for countable items (arrows, scrolls, etc), how many of
    // them you have.
    pub number: libc::uint16_t,

    // the next four all apply to items that can be worn or wielded (i.e. weapons
    // and armor)
    pub tohit: libc::int16_t,       // An item's magical + to hit
    pub todam: libc::int16_t,       // An item's magical + to damage
    pub ac: libc::int16_t,          // An item's normal armor class value.
    pub toac: libc::int16_t,        // An item's magical + to AC.

    // the amount of damage an item does to monster.  everything
    // should have a damage value, even if it's just "0d0".
    pub damage: model::Damage,

    // a vague measurement of how strong an item's magic is.
    pub level: libc::int8_t,

    // is this item identified?
    pub identified: libc::uint8_t,
}

impl Item {

    pub fn item_type(&self) -> model::ItemType {
        model::ItemType::from(self.tval)
    }

    pub fn subtype(&self) -> Box<dyn item_template::ItemTemplate> {
        let subval = self.subval;
        match self.item_type() {
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

    pub fn set_identified(&self, yn: bool) {
        self.identified = if yn { 255 } else { 0 };
        if yn {
            self.item_type().set_identified(true);
        }
    }

    pub fn is_identified(&self) -> bool {
        self.identified != 0
    }
}
