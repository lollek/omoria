pub use self::chime:: {
    ChimeTemplate, generate_chime,
};
pub use self::horn:: {
    HornTemplate, generate_horn,
};
pub use self::staff:: {
    StaffTemplate, generate_staff,
};
pub use self::wand:: {
    WandTemplate, generate_wand,
};

pub use self::main::generate_magic_item;

mod chime;
mod horn;
mod main;
mod staff;
mod wand;
