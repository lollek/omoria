use model::{ Spell };

pub fn spell(level: usize) -> Spell {
    match level {
        0  => Spell { name: "Detect Monsters", level: 3, mana: 2, fail: 35 },
        1  => Spell { name: "Battle Song", level: 3, mana: 2, fail: 35 },
        2  => Spell { name: "Blink", level: 3, mana: 2, fail: 35 },
        3  => Spell { name: "Light Area", level: 5, mana: 4, fail: 35 },
        4  => Spell { name: "Find Hidden Doors/Traps", level: 5, mana: 5, fail: 45 },
        5  => Spell { name: "Magical Jig", level: 7, mana: 8, fail: 40 },
        6  => Spell { name: "Detect Magic", level: 7, mana: 9, fail: 70 },

        8  => Spell { name: "Battle Dance", level: 9, mana: 10, fail: 45 },
        9  => Spell { name: "Charm Monsters", level: 9, mana: 11, fail: 50 },
        10 => Spell { name: "Detect Curse", level: 11, mana: 12, fail: 40 },
        11 => Spell { name: "Detect Invisible", level: 11, mana: 12, fail: 40 },
        12 => Spell { name: "Cure Poison", level: 13, mana: 14, fail: 45 },
        13 => Spell { name: "Invisibility", level: 15, mana: 16, fail: 50 },
        14 => Spell { name: "Shadow Gate", level: 17, mana: 18, fail: 55 },


        17 => Spell { name: "Recharge Item", level: 19, mana: 18, fail: 55 },
        18 => Spell { name: "Remove Curse", level: 21, mana: 20, fail: 90 },
        19 => Spell { name: "Legend Lore", level: 23, mana: 22, fail: 95 },

        21 => Spell { name: "Detect Treasure", level: 25, mana: 25, fail: 50 },
        22 => Spell { name: "Detect Object", level: 25, mana: 25, fail: 55 },



        26 => Spell { name: "Word of Recall", level: 27, mana: 27, fail: 60 },

        28 => Spell { name: "See Invisible", level: 29, mana: 29, fail: 65 },
        29 => Spell { name: "Magic Mapping", level: 31, mana: 30, fail: 70 },

        31 => Spell { name: "Battle Frenzy", level: 33, mana: 31, fail: 70 },

        33 => Spell { name: "Resist Charm", level: 35, mana: 32, fail: 70 },
        34 => Spell { name: "Item Lore", level: 37, mana: 33, fail: 95 },
        _  => Spell { name: "", level: 99, mana: 99, fail: 0 },
    }
}
