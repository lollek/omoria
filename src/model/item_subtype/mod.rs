use crate::model::ItemType;

mod ammo;
mod amulet;
mod armor;
mod bag;
mod book;
mod chest;
mod chime;
mod dagger;
mod food;
mod gem;
mod hafted_weapon;
mod horn;
mod jewelry;
mod light_source;
mod lodging;
mod maul;
mod misc_object;
mod misc_usable;
mod pick;
mod pole_arm;
mod potion;
mod ranged_weapon;
mod ring;
mod rod;
mod scroll;
mod staff;
mod sword;
mod wand;
mod wearable_gem;

pub use ammo::*;
pub use amulet::*;
pub use armor::*;
pub use bag::*;
pub use book::*;
pub use chest::*;
pub use chime::*;
pub use dagger::*;
pub use food::*;
pub use gem::*;
pub use hafted_weapon::*;
pub use horn::*;
pub use jewelry::*;
pub use light_source::*;
pub use lodging::*;
pub use maul::*;
pub use misc_object::*;
pub use misc_usable::*;
pub use pick::*;
pub use pole_arm::*;
pub use potion::*;
pub use ranged_weapon::*;
pub use ring::*;
pub use rod::*;
pub use scroll::*;
pub use staff::*;
pub use sword::*;
pub use wand::*;
pub use wearable_gem::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemSubType {
    MiscObject(MiscObjectSubType),
    Chest(ChestSubType),
    MiscUsable(MiscUsableSubType),
    Jewelry(JewelrySubType),
    Gem(GemSubType),
    Bag(BagSubType),
    WearableGem(WearableGemSubType),

    SlingAmmo(SlingAmmoSubType),
    Bolt(BoltSubType),
    Arrow(ArrowSubType),
    Spike(SpikeSubType),

    LightSource(LightSourceSubType),

    RangedWeapon(RangedWeaponSubType),
    HaftedWeapon(HaftedWeaponSubType),
    PoleArm(PoleArmSubType),
    Dagger(DaggerSubType),
    Sword(SwordSubType),
    Pick(PickSubType),
    Maul(MaulSubType),

    GemHelm(GemHelmSubType),
    Boots(BootsSubType),
    Gloves(GlovesSubType),
    Cloak(CloakSubType),
    Helm(HelmSubType),
    Shield(ShieldSubType),
    HardArmor(HardArmorSubType),
    SoftArmor(SoftArmorSubType),
    Bracers(BracersSubType),
    Belt(BeltSubType),

    Amulet(AmuletSubType),

    Ring(RingSubType),

    Staff(StaffSubType),

    Rod(RodSubType),

    Wand(WandSubType),
    Scroll1(Scroll1SubType),
    Scroll2(Scroll2SubType),

    Potion1(Potion1SubType),
    Potion2(Potion2SubType),
    FlaskOfOil(FlaskOfOilSubType),

    Food(FoodSubType),
    JunkFood(JunkFoodSubType),

    Chime(ChimeSubType),
    Horn(HornSubType),

    MagicBook(MagicBookSubType),
    PrayerBook(PrayerBookSubType),
    Instrument(InstrumentSubType),
    SongBook(SongBookSubType),

    LodgingAtInn(LodgingAtInnSubType),
}

impl ItemSubType {
    pub fn get_type(&self) -> ItemType {
        match self {
            ItemSubType::MiscObject(_) => ItemType::MiscObject,
            ItemSubType::Chest(_) => ItemType::Chest,
            ItemSubType::MiscUsable(_) => ItemType::MiscUsable,
            ItemSubType::Jewelry(_) => ItemType::Jewelry,
            ItemSubType::Gem(_) => ItemType::Gem,
            ItemSubType::Bag(_) => ItemType::Bag,
            ItemSubType::WearableGem(_) => ItemType::WearableGem,
            ItemSubType::SlingAmmo(_) => ItemType::SlingAmmo,
            ItemSubType::Bolt(_) => ItemType::Bolt,
            ItemSubType::Arrow(_) => ItemType::Arrow,
            ItemSubType::Spike(_) => ItemType::Spike,
            ItemSubType::LightSource(_) => ItemType::LightSource,
            ItemSubType::RangedWeapon(_) => ItemType::RangedWeapon,
            ItemSubType::HaftedWeapon(_) => ItemType::HaftedWeapon,
            ItemSubType::PoleArm(_) => ItemType::PoleArm,
            ItemSubType::Dagger(_) => ItemType::Dagger,
            ItemSubType::Sword(_) => ItemType::Sword,
            ItemSubType::Pick(_) => ItemType::Pick,
            ItemSubType::Maul(_) => ItemType::Maul,
            ItemSubType::GemHelm(_) => ItemType::GemHelm,
            ItemSubType::Boots(_) => ItemType::Boots,
            ItemSubType::Gloves(_) => ItemType::Gloves,
            ItemSubType::Cloak(_) => ItemType::Cloak,
            ItemSubType::Helm(_) => ItemType::Helm,
            ItemSubType::Shield(_) => ItemType::Shield,
            ItemSubType::HardArmor(_) => ItemType::HardArmor,
            ItemSubType::SoftArmor(_) => ItemType::SoftArmor,
            ItemSubType::Bracers(_) => ItemType::Bracers,
            ItemSubType::Belt(_) => ItemType::Belt,
            ItemSubType::Amulet(_) => ItemType::Amulet,
            ItemSubType::Ring(_) => ItemType::Ring,
            ItemSubType::Staff(_) => ItemType::Staff,
            ItemSubType::Rod(_) => ItemType::Rod,
            ItemSubType::Wand(_) => ItemType::Wand,
            ItemSubType::Scroll1(_) => ItemType::Scroll1,
            ItemSubType::Scroll2(_) => ItemType::Scroll2,
            ItemSubType::Potion1(_) => ItemType::Potion1,
            ItemSubType::Potion2(_) => ItemType::Potion2,
            ItemSubType::FlaskOfOil(_) => ItemType::FlaskOfOil,
            ItemSubType::Food(_) => ItemType::Food,
            ItemSubType::JunkFood(_) => ItemType::JunkFood,
            ItemSubType::Chime(_) => ItemType::Chime,
            ItemSubType::Horn(_) => ItemType::Horn,
            ItemSubType::MagicBook(_) => ItemType::MagicBook,
            ItemSubType::PrayerBook(_) => ItemType::PrayerBook,
            ItemSubType::Instrument(_) => ItemType::Instrument,
            ItemSubType::SongBook(_) => ItemType::SongBook,
            ItemSubType::LodgingAtInn(_) => ItemType::LodgingAtInn,
        }
    }
}
