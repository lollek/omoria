use model::{ Spell };

pub fn spell(level: usize) -> Spell {
    match level {
        0  => Spell { name: "Self-Healing", level: 3, mana: 3, fail: 50 },
        1  => Spell { name: "Courage", level: 5, mana: 5, fail: 50 },
        2  => Spell { name: "Slow Poison", level: 7, mana: 7, fail: 50 },
        3  => Spell { name: "Negate Hunger", level: 9, mana: 9, fail: 50 },
        4  => Spell { name: "Sense Enemies", level: 11, mana: 11, fail: 50 },
        5  => Spell { name: "Self-Healing II", level: 13, mana: 13, fail: 50 },
        6  => Spell { name: "Night Vision", level: 15, mana: 15, fail: 50 },
        7  => Spell { name: "Poison Immunity", level: 17, mana: 17, fail: 50 },
        8  => Spell { name: "See Invisible", level: 19, mana: 19, fail: 50 },
        9  => Spell { name: "Advanced Self-Healing", level: 21, mana: 21, fail: 50 },
        10 => Spell { name: "Resist Petrification", level: 23, mana: 23, fail: 50 },
        11 => Spell { name: "Stealth", level: 25, mana: 25, fail: 50 },
        12 => Spell { name: "Free Action", level: 27, mana: 27, fail: 50 },
        13 => Spell { name: "Improved Speed", level: 29, mana: 29, fail: 50 },
        _  => Spell { name: "", level: 99, mana: 99, fail: 0 },
    }
}
