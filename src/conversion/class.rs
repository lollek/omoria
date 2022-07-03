use model;

pub fn from_usize(pos: usize) -> Option<model::Class> {
    match pos {
        0 => Some(model::Class::Fighter),
        1 => Some(model::Class::Wizard),
        2 => Some(model::Class::Cleric),
        3 => Some(model::Class::Rogue),
        4 => Some(model::Class::Ranger),
        5 => Some(model::Class::Paladin),
        6 => Some(model::Class::Druid),
        7 => Some(model::Class::Bard),
        8 => Some(model::Class::Adventurer),
        9 => Some(model::Class::Monk),
        10 => Some(model::Class::Barbarian),
        _ => None,
    }
}

pub fn to_usize(class: model::Class) -> usize {
    match class {
        model::Class::Fighter => 0,
        model::Class::Wizard => 1,
        model::Class::Cleric => 2,
        model::Class::Rogue => 3,
        model::Class::Ranger => 4,
        model::Class::Paladin => 5,
        model::Class::Druid => 6,
        model::Class::Bard => 7,
        model::Class::Adventurer => 8,
        model::Class::Monk => 9,
        model::Class::Barbarian => 10,
    }
}
