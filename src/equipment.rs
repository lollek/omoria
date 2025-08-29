use crate::data;
use crate::model::Item;

pub const EQUIP_MAX: usize = 15;

extern "C" {
    static mut equipment: [Item; EQUIP_MAX];
}

#[derive(Copy, Clone)]
pub enum Slot {
    Primary = 0,
    Helm = 1,
    Amulet = 2,
    Armor = 3,
    Belt = 4,
    Shield = 5,
    Gloves = 6,
    Bracers = 7,
    RightRing = 8,
    LeftRing = 9,
    Boots = 10,
    Cloak = 11,
    Light = 12,
    Secondary = 13,
}

impl Slot {
    pub fn name(&self) -> &'static str {
        match self {
            Slot::Primary => "Main hand",
            Slot::Helm => "Helm",
            Slot::Amulet => "Amulet",
            Slot::Armor => "Armor",
            Slot::Belt => "Belt",
            Slot::Shield => "Shield",
            Slot::Gloves => "Gloves",
            Slot::Bracers => "Bracers",
            Slot::RightRing => "Right ring",
            Slot::LeftRing => "Left ring",
            Slot::Boots => "Boots",
            Slot::Cloak => "Cloak",
            Slot::Light => "Light source",
            Slot::Secondary => "Backup weapon",
        }
    }
}

impl From<usize> for Slot {
    fn from(i: usize) -> Self {
        match i {
            0 => Slot::Primary,
            1 => Slot::Helm,
            2 => Slot::Amulet,
            3 => Slot::Armor,
            4 => Slot::Belt,
            5 => Slot::Shield,
            6 => Slot::Gloves,
            7 => Slot::Bracers,
            8 => Slot::RightRing,
            9 => Slot::LeftRing,
            10 => Slot::Boots,
            11 => Slot::Cloak,
            12 => Slot::Light,
            13 => Slot::Secondary,
            _ => panic!(),
        }
    }
}

pub fn slots_iter() -> impl Iterator<Item = usize> {
    (Slot::Primary as usize)..(Slot::Secondary as usize + 1)
}

pub unsafe fn get_item(slot: Slot) -> *const Item {
    &equipment[slot as usize]
}

pub fn get_name(slot: Slot) -> String {
    let item = unsafe { equipment[slot as usize] };
    match item.tval {
        0 => "".to_string(),
        _ => data::item_name::generate(&item),
    }
}
