pub use self::axe::{
    AxeTemplate, generate_axe,
};
pub use self::bow::{
    BowTemplate, generate_bow,
};
pub use self::crossbow::{
    CrossbowTemplate, generate_crossbow,
};
pub use self::dagger::{
    DaggerTemplate, generate_dagger,
};
pub use self::mace::{
    MaceTemplate, generate_mace,
};
pub use self::polearm::{
    PolearmTemplate, generate_polearm,
};
pub use self::sling::{
    SlingTemplate, generate_sling,
};
pub use self::sword::{
    SwordTemplate, generate_sword,
};
pub use self::main::generate_weapon;

mod axe;
mod bow;
mod crossbow;
mod dagger;
mod mace;
mod main;
mod polearm;
mod sling;
mod sword;
