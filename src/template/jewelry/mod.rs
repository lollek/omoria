pub use self::amulet::{
    AmuletTemplate, generate_amulet
};

pub use self::valuable::{
    ValuableTemplate, generate_valuable
};

pub use self::ring::{
    RingTemplate, generate_ring
};

pub use self::main::generate_jewelry;


mod amulet;
mod valuable;
mod ring;
mod main;
