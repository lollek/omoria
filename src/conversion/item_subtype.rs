use crate::model::{item_subtype::ItemSubType, ItemType};

pub mod light_source;
pub mod misc_item;
pub mod chest;
pub mod misc_usable;
pub mod jewelry;
pub mod gem;
pub mod bag;
pub mod wearable_gem;
pub mod sling_ammo;
pub mod bolt;
pub mod arrow;
pub mod spike;

pub fn to_usize(item_subtype: ItemSubType) -> usize {
    match item_subtype {
        ItemSubType::MiscObject(subtype) => misc_item::to_usize(subtype),
        ItemSubType::Chest(subtype) => chest::to_usize(subtype), 
        ItemSubType::MiscUsable(subtype) => misc_usable::to_usize(subtype),
        ItemSubType::Jewelry(subtype) => jewelry::to_usize(subtype),
        ItemSubType::Gem(subtype) => gem::to_usize(subtype),
        ItemSubType::Bag(subtype) => bag::to_usize(subtype),
        ItemSubType::WearableGem(subtype) => wearable_gem::to_usize(subtype),
        ItemSubType::SlingAmmo(subtype) => sling_ammo::to_usize(subtype),
        ItemSubType::Bolt(subtype) => bolt::to_usize(subtype),
        ItemSubType::Arrow(subtype) => arrow::to_usize(subtype),
        ItemSubType::Spike(subtype) => spike::to_usize(subtype),
        ItemSubType::LightSource(subtype) => light_source::to_usize(subtype),
//        ItemSubType::RangedWeapon(RangedWeaponSubType),
//        ItemSubType::HaftedWeapon(HaftedWeaponSubType),
//        ItemSubType::PoleArm(PoleArmSubType),
//        ItemSubType::Dagger(DaggerSubType),
//        ItemSubType::Sword(SwordSubType),
//        ItemSubType::Pick(PickSubType),
//        ItemSubType::Maul(MaulSubType),
//        ItemSubType::GemHelm(GemHelmSubType),
//        ItemSubType::Boots(BootsSubType),
//        ItemSubType::Gloves(GlovesSubType),
//        ItemSubType::Cloak(CloakSubType),
//        ItemSubType::Helm(HelmSubType),
//        ItemSubType::Shield(ShieldSubType),
//        ItemSubType::HardArmor(HardArmorSubType),
//        ItemSubType::SoftArmor(SoftArmorSubType),
//        ItemSubType::Bracers(BracersSubType),
//        ItemSubType::Belt(BeltSubType),
//        ItemSubType::Amulet(AmuletSubType),
//        ItemSubType::Ring(RingSubType),
//        ItemSubType::Staff(StaffSubType),
//        ItemSubType::Rod(RodSubType),
//        ItemSubType::Wand(WandSubType),
//        ItemSubType::Scroll1(Scroll1SubType),
//        ItemSubType::Scroll2(Scroll2SubType),
//        ItemSubType::Potion1(Potion1SubType),
//        ItemSubType::Potion2(Potion2SubType),
//        ItemSubType::FlaskOfOil(FlaskOfOilSubType),
//        ItemSubType::Food(FoodSubType),
//        ItemSubType::JunkFood(JunkFoodSubType),
//        ItemSubType::Chime(ChimeSubType),
//        ItemSubType::Horn(HornSubType),
//        ItemSubType::MagicBook(MagicBookSubType),
//        ItemSubType::PrayerBook(PrayerBookSubType),
//        ItemSubType::Instrument(InstrumentSubType),
//        ItemSubType::SongBook(SongBookSubType),
        _ => panic!("Unimplemented subtype: {item_subtype:?}")
    }
}

pub fn from_usize(item_type: ItemType, item_subtype: usize) -> Option<ItemSubType> {
    match item_type {
        ItemType::MiscObject => misc_item::from_usize(item_subtype).map(ItemSubType::MiscObject),
        ItemType::Chest => chest::from_usize(item_subtype).map(ItemSubType::Chest),
        ItemType::MiscUsable => misc_usable::from_usize(item_subtype).map(ItemSubType::MiscUsable),
        ItemType::Jewelry => jewelry::from_usize(item_subtype).map(ItemSubType::Jewelry),
        ItemType::Gem => gem::from_usize(item_subtype).map(ItemSubType::Gem),
        ItemType::Bag => bag::from_usize(item_subtype).map(ItemSubType::Bag),
        ItemType::WearableGem => wearable_gem::from_usize(item_subtype).map(ItemSubType::WearableGem),
        ItemType::SlingAmmo => sling_ammo::from_usize(item_subtype).map(ItemSubType::SlingAmmo),
        ItemType::Bolt => bolt::from_usize(item_subtype).map(ItemSubType::Bolt),
        ItemType::Arrow => arrow::from_usize(item_subtype).map(ItemSubType::Arrow),
        ItemType::Spike => spike::from_usize(item_subtype).map(ItemSubType::Spike),
        ItemType::LightSource => {
            light_source::from_usize(item_subtype).map(ItemSubType::LightSource)
        },
//        ItemSubType::RangedWeapon(RangedWeaponSubType),
//        ItemSubType::HaftedWeapon(HaftedWeaponSubType),
//        ItemSubType::PoleArm(PoleArmSubType),
//        ItemSubType::Dagger(DaggerSubType),
//        ItemSubType::Sword(SwordSubType),
//        ItemSubType::Pick(PickSubType),
//        ItemSubType::Maul(MaulSubType),
//        ItemSubType::GemHelm(GemHelmSubType),
//        ItemSubType::Boots(BootsSubType),
//        ItemSubType::Gloves(GlovesSubType),
//        ItemSubType::Cloak(CloakSubType),
//        ItemSubType::Helm(HelmSubType),
//        ItemSubType::Shield(ShieldSubType),
//        ItemSubType::HardArmor(HardArmorSubType),
//        ItemSubType::SoftArmor(SoftArmorSubType),
//        ItemSubType::Bracers(BracersSubType),
//        ItemSubType::Belt(BeltSubType),
//        ItemSubType::Amulet(AmuletSubType),
//        ItemSubType::Ring(RingSubType),
//        ItemSubType::Staff(StaffSubType),
//        ItemSubType::Rod(RodSubType),
//        ItemSubType::Wand(WandSubType),
//        ItemSubType::Scroll1(Scroll1SubType),
//        ItemSubType::Scroll2(Scroll2SubType),
//        ItemSubType::Potion1(Potion1SubType),
//        ItemSubType::Potion2(Potion2SubType),
//        ItemSubType::FlaskOfOil(FlaskOfOilSubType),
//        ItemSubType::Food(FoodSubType),
//        ItemSubType::JunkFood(JunkFoodSubType),
//        ItemSubType::Chime(ChimeSubType),
//        ItemSubType::Horn(HornSubType),
//        ItemSubType::MagicBook(MagicBookSubType),
//        ItemSubType::PrayerBook(PrayerBookSubType),
//        ItemSubType::Instrument(InstrumentSubType),
//        ItemSubType::SongBook(SongBookSubType),
        _ => panic!("Unhandled item type {:?}", item_type),
    }
}
