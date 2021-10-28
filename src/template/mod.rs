pub use self::armor::{
    generate_armor_types,
    ArmorTemplate, generate_armor,
    BeltTemplate, generate_belt,
    BootsTemplate, generate_boots,
    BracersTemplate, generate_bracers,
    CloakTemplate, generate_cloak,
    GlovesTemplate, generate_gloves,
    HelmTemplate, generate_helm,
    ShieldTemplate, generate_shield,
};
pub use self::potion::{
    PotionTemplate, generate_potion
};
pub use self::scroll::{
    ScrollTemplate, generate_scroll
};
pub use self::dungeon_items::{
    generate_dungeon_items,
    generate_chest, ChestTemplate,
    generate_misc, MiscTemplate,
};
pub use self::usable::{
    generate_usable,
    generate_ammunition, AmmunitionTemplate,
    generate_light_source, LightSourceTemplate,
    generate_pick, PickTemplate,
    generate_bag, BagTemplate,
    generate_misc_usable, MiscUsableTemplate,
};
pub use self::jewelry::{
    generate_jewelry,
    generate_amulet, AmuletTemplate,
    generate_valuable, ValuableTemplate,
    generate_ring, RingTemplate,
};
pub use self::magic_item:: {
    generate_magic_item,
    ChimeTemplate, generate_chime,
    HornTemplate, generate_horn,
    StaffTemplate, generate_staff,
    WandTemplate, generate_wand,
};
pub use self::weapon:: {
    generate_weapon,
    AxeTemplate, generate_axe,
    BowTemplate, generate_bow,
    CrossbowTemplate, generate_crossbow,
    DaggerTemplate, generate_dagger,
    MaceTemplate, generate_mace,
    PolearmTemplate, generate_polearm,
    SlingTemplate, generate_sling,
    SwordTemplate, generate_sword,
};

mod armor;
mod dungeon_items;
mod jewelry;
mod magic_item;
mod potion;
mod scroll;
mod usable;
mod weapon;

pub mod food;
pub mod magic_token;
