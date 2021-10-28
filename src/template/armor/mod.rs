pub use self::main::generate_armor_types;
pub use self::armor::{ ArmorTemplate, generate_armor };
pub use self::belt::{ BeltTemplate, generate_belt };
pub use self::boots::{ BootsTemplate, generate_boots };
pub use self::bracers::{ BracersTemplate, generate_bracers };
pub use self::cloak::{ CloakTemplate, generate_cloak };
pub use self::gloves::{ GlovesTemplate, generate_gloves };
pub use self::helm::{ HelmTemplate, generate_helm };
pub use self::shield::{ ShieldTemplate, generate_shield };

mod armor;
mod belt;
mod boots;
mod bracers;
mod cloak;
mod gloves;
mod helm;
mod main;
mod shield;
