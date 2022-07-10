use crate::model::Race;

pub struct History {
    // Sentence, or part thereof, explaining part of a player's history
    pub history: &'static str,

    // Chance of this being picked, as in if 1d100 >= chance
    pub chance: u8,

    // Default social class
    pub social_class: i16,
}

pub fn for_race(race: Race) -> Vec<Vec<History>> {
    match race {
        Race::Human => human(),
        Race::HalfElf => halfelf(),
        Race::Elf => elf(),
        Race::Halfling => halfling(),
        Race::Gnome => gnome(),
        Race::Dwarf => dwarf(),
        Race::HalfOrc => halforc(),
        Race::HalfTroll => halftroll(),
        Race::Phraint => phraint(),
        Race::Dryad => dryad(),
    }
}

pub fn human() -> Vec<Vec<History>> {
    vec![
        vec![
            History {
                history: "You are the illegitimate and unacknowledged child ",
                chance: 10,
                social_class: -25,
            },
            History {
                history: "You are the illegitimate but acknowledged child ",
                chance: 20,
                social_class: -15,
            },
            History {
                history: "You are one of several children ",
                chance: 95,
                social_class: -5,
            },
            History {
                history: "You are the first child ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "of a serf.  ",
                chance: 40,
                social_class: 15,
            },
            History {
                history: "of a yeoman.  ",
                chance: 65,
                social_class: 30,
            },
            History {
                history: "of a townsman.  ",
                chance: 80,
                social_class: 40,
            },
            History {
                history: "of a guildsman.  ",
                chance: 90,
                social_class: 55,
            },
            History {
                history: "of a landed knight.  ",
                chance: 96,
                social_class: 70,
            },
            History {
                history: "of a titled noble.  ",
                chance: 99,
                social_class: 80,
            },
            History {
                history: "of a royal blood line.  ",
                chance: 100,
                social_class: 90,
            },
        ],
        vec![
            History {
                history: "You are the black sheep of the family.  ",
                chance: 20,
                social_class: -30,
            },
            History {
                history: "You are a credit to the family.  ",
                chance: 80,
                social_class: 5,
            },
            History {
                history: "You are a well liked child.  ",
                chance: 100,
                social_class: 10,
            },
        ],
        vec![
            History {
                history: "You have dark brown eyes, ",
                chance: 20,
                social_class: 0,
            },
            History {
                history: "You have brown eyes, ",
                chance: 60,
                social_class: 0,
            },
            History {
                history: "You have hazel eyes, ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "You have green eyes, ",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "You have blue eyes, ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "You have blue-gray eyes, ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "straight ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "wavey ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "curly ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "black hair, ",
                chance: 30,
                social_class: 0,
            },
            History {
                history: "brown hair, ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "auburn hair, ",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "red hair, ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "blonde hair, ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "and a very dark complexion.",
                chance: 10,
                social_class: 0,
            },
            History {
                history: "and a dark complexion.",
                chance: 30,
                social_class: 0,
            },
            History {
                history: "and an average complexion.",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "and a fair complexion.",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "and a very fair complexion.",
                chance: 100,
                social_class: 0,
            },
        ],
    ]
}

pub fn halfelf() -> Vec<Vec<History>> {
    let mut history = vec![vec![
        History {
            history: "Your mother was a Green-Elf.  ",
            chance: 40,
            social_class: 0,
        },
        History {
            history: "Your father was a Green-Elf.  ",
            chance: 75,
            social_class: 5,
        },
        History {
            history: "Your mother was a Grey-Elf.  ",
            chance: 90,
            social_class: 5,
        },
        History {
            history: "Your father was a Grey-Elf.  ",
            chance: 95,
            social_class: 10,
        },
        History {
            history: "Your mother was a High-Elf.  ",
            chance: 98,
            social_class: 15,
        },
        History {
            history: "Your father was a High-Elf.  ",
            chance: 100,
            social_class: 20,
        },
    ]];
    history.append(&mut human());
    return history;
}

pub fn elf() -> Vec<Vec<History>> {
    vec![
        vec![
            History {
                history: "You are one of several children ",
                chance: 60,
                social_class: 0,
            },
            History {
                history: "You are the only child ",
                chance: 100,
                social_class: 5,
            },
        ],
        vec![
            History {
                history: "of a Green-Elf ",
                chance: 75,
                social_class: 0,
            },
            History {
                history: "of a Grey-Elf ",
                chance: 95,
                social_class: 5,
            },
            History {
                history: "of a High-Elf ",
                chance: 100,
                social_class: 10,
            },
        ],
        vec![
            History {
                history: "ranger.  ",
                chance: 40,
                social_class: 30,
            },
            History {
                history: "archer.  ",
                chance: 70,
                social_class: 40,
            },
            History {
                history: "warrior.  ",
                chance: 87,
                social_class: 60,
            },
            History {
                history: "mage.  ",
                chance: 95,
                social_class: 75,
            },
            History {
                history: "prince.  ",
                chance: 99,
                social_class: 90,
            },
            History {
                history: "king.  ",
                chance: 100,
                social_class: 95,
            },
        ],
        vec![
            History {
                history: "You have light grey eyes, ",
                chance: 85,
                social_class: 0,
            },
            History {
                history: "You have light violet eyes, ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "You have light blue eyes, ",
                chance: 95,
                social_class: 0,
            },
            History {
                history: "You have light green eyes, ",
                chance: 98,
                social_class: 2,
            },
            History {
                history: "You have light golden colored eyes, ",
                chance: 100,
                social_class: 5,
            },
        ],
        vec![
            History {
                history: "straight ",
                chance: 75,
                social_class: 0,
            },
            History {
                history: "wavey ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "jet black hair, and a fair complexion.",
                chance: 75,
                social_class: 0,
            },
            History {
                history: "light brown hair, and a fair complexion.",
                chance: 85,
                social_class: 0,
            },
            History {
                history: "blonde hair, and a fair complexion.",
                chance: 95,
                social_class: 0,
            },
            History {
                history: "silver hair, and a fair complexion.",
                chance: 98,
                social_class: 1,
            },
            History {
                history: "hair the color of spun gold and pale skin.",
                chance: 100,
                social_class: 5,
            },
        ],
    ]
}

pub fn halfling() -> Vec<Vec<History>> {
    vec![
        vec![
            History {
                history: "You are one of several children of a Halfling ",
                chance: 85,
                social_class: -5,
            },
            History {
                history: "You are the only child of a Halfling ",
                chance: 100,
                social_class: 5,
            },
        ],
        vec![
            History {
                history: "bum.  ",
                chance: 20,
                social_class: 5,
            },
            History {
                history: "tavern owner.  ",
                chance: 30,
                social_class: 30,
            },
            History {
                history: "miller.  ",
                chance: 40,
                social_class: 40,
            },
            History {
                history: "home owner.  ",
                chance: 50,
                social_class: 50,
            },
            History {
                history: "burglar.  ",
                chance: 80,
                social_class: 60,
            },
            History {
                history: "monk.  ",
                chance: 95,
                social_class: 65,
            },
            History {
                history: "clan elder.  ",
                chance: 100,
                social_class: 90,
            },
        ],
        vec![
            History {
                history: "You are the black sheep of the family.  ",
                chance: 20,
                social_class: -30,
            },
            History {
                history: "You are a credit to the family.  ",
                chance: 80,
                social_class: 5,
            },
            History {
                history: "You are a well liked child.  ",
                chance: 100,
                social_class: 10,
            },
        ],
        vec![
            History {
                history: "You have dark brown eyes, ",
                chance: 20,
                social_class: 0,
            },
            History {
                history: "You have brown eyes, ",
                chance: 60,
                social_class: 0,
            },
            History {
                history: "You have hazel eyes, ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "You have green eyes, ",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "You have blue eyes, ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "You have blue-gray eyes, ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "straight ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "wavey ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "curly ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "black hair, ",
                chance: 30,
                social_class: 0,
            },
            History {
                history: "brown hair, ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "auburn hair, ",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "red hair, ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "blonde hair, ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "and a very dark complexion.",
                chance: 10,
                social_class: 0,
            },
            History {
                history: "and a dark complexion.",
                chance: 30,
                social_class: 0,
            },
            History {
                history: "and an average complexion.",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "and a fair complexion.",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "and a very fair complexion.",
                chance: 100,
                social_class: 0,
            },
        ],
    ]
}

pub fn gnome() -> Vec<Vec<History>> {
    vec![
        vec![
            History {
                history: "You are one of several children of a Gnome ",
                chance: 85,
                social_class: -5,
            },
            History {
                history: "You are the only child of a Gnome ",
                chance: 100,
                social_class: 5,
            },
        ],
        vec![
            History {
                history: "beggar.  ",
                chance: 20,
                social_class: 5,
            },
            History {
                history: "bragart.  ",
                chance: 50,
                social_class: 20,
            },
            History {
                history: "prankster.  ",
                chance: 75,
                social_class: 35,
            },
            History {
                history: "druid.  ",
                chance: 95,
                social_class: 50,
            },
            History {
                history: "mage.  ",
                chance: 100,
                social_class: 75,
            },
        ],
        vec![
            History {
                history: "You are the black sheep of the family.  ",
                chance: 20,
                social_class: -30,
            },
            History {
                history: "You are a credit to the family.  ",
                chance: 80,
                social_class: 5,
            },
            History {
                history: "You are a well liked child.  ",
                chance: 100,
                social_class: 10,
            },
        ],
        vec![
            History {
                history: "You have dark brown eyes, ",
                chance: 20,
                social_class: 0,
            },
            History {
                history: "You have brown eyes, ",
                chance: 60,
                social_class: 0,
            },
            History {
                history: "You have hazel eyes, ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "You have green eyes, ",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "You have blue eyes, ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "You have blue-gray eyes, ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "straight ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "wavey ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "curly ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "black hair, ",
                chance: 30,
                social_class: 0,
            },
            History {
                history: "brown hair, ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "auburn hair, ",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "red hair, ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "blonde hair, ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "and a very dark complexion.",
                chance: 10,
                social_class: 0,
            },
            History {
                history: "and a dark complexion.",
                chance: 30,
                social_class: 0,
            },
            History {
                history: "and an average complexion.",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "and a fair complexion.",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "and a very fair complexion.",
                chance: 100,
                social_class: 0,
            },
        ],
    ]
}

pub fn dwarf() -> Vec<Vec<History>> {
    vec![
        vec![
            History {
                history: "You are one of two children of a Dwarven ",
                chance: 25,
                social_class: -10,
            },
            History {
                history: "You are the only child of a Dwarven ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "thief.  ",
                chance: 10,
                social_class: 10,
            },
            History {
                history: "prison guard.  ",
                chance: 25,
                social_class: 25,
            },
            History {
                history: "miner.  ",
                chance: 75,
                social_class: 40,
            },
            History {
                history: "warrior.  ",
                chance: 90,
                social_class: 60,
            },
            History {
                history: "priest.  ",
                chance: 99,
                social_class: 80,
            },
            History {
                history: "king.  ",
                chance: 100,
                social_class: 100,
            },
        ],
        vec![
            History {
                history: "You are the black sheep of the family.  ",
                chance: 15,
                social_class: -40,
            },
            History {
                history: "You are a credit to the family.  ",
                chance: 85,
                social_class: 0,
            },
            History {
                history: "You are a well liked child.  ",
                chance: 100,
                social_class: 5,
            },
        ],
        vec![
            History {
                history: "You have dark brown eyes, ",
                chance: 99,
                social_class: 0,
            },
            History {
                history: "You have glowing red eyes, ",
                chance: 100,
                social_class: 10,
            },
        ],
        vec![
            History {
                history: "straight ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "wavey ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "black hair, ",
                chance: 75,
                social_class: 0,
            },
            History {
                history: "brown hair, ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "a one foot beard, ",
                chance: 25,
                social_class: 0,
            },
            History {
                history: "a two foot beard, ",
                chance: 60,
                social_class: 1,
            },
            History {
                history: "a three foot beard, ",
                chance: 90,
                social_class: 3,
            },
            History {
                history: "a four foot beard, ",
                chance: 100,
                social_class: 5,
            },
        ],
        vec![History {
            history: "and a dark complexion.",
            chance: 100,
            social_class: 0,
        }],
    ]
}

pub fn halforc() -> Vec<Vec<History>> {
    vec![
        vec![
            History {
                history: "Your mother was an Orc, but it is unacknowledged.  ",
                chance: 25,
                social_class: -25,
            },
            History {
                history: "Your father was an Orc, but it is unacknowledged.  ",
                chance: 100,
                social_class: -25,
            },
        ],
        vec![History {
            history: "You are the adopted child ",
            chance: 100,
            social_class: 0,
        }],
        vec![
            History {
                history: "of a serf.  ",
                chance: 40,
                social_class: 15,
            },
            History {
                history: "of a yeoman.  ",
                chance: 65,
                social_class: 30,
            },
            History {
                history: "of a townsman.  ",
                chance: 80,
                social_class: 40,
            },
            History {
                history: "of a guildsman.  ",
                chance: 90,
                social_class: 55,
            },
            History {
                history: "of a landed knight.  ",
                chance: 96,
                social_class: 70,
            },
            History {
                history: "of a titled noble.  ",
                chance: 99,
                social_class: 80,
            },
            History {
                history: "of a royal blood line.  ",
                chance: 100,
                social_class: 90,
            },
        ],
        vec![
            History {
                history: "You are the black sheep of the family.  ",
                chance: 20,
                social_class: -30,
            },
            History {
                history: "You are a credit to the family.  ",
                chance: 80,
                social_class: 5,
            },
            History {
                history: "You are a well liked child.  ",
                chance: 100,
                social_class: 10,
            },
        ],
        vec![
            History {
                history: "You have dark brown eyes, ",
                chance: 20,
                social_class: 0,
            },
            History {
                history: "You have brown eyes, ",
                chance: 60,
                social_class: 0,
            },
            History {
                history: "You have hazel eyes, ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "You have green eyes, ",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "You have blue eyes, ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "You have blue-gray eyes, ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "straight ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "wavey ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "curly ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "black hair, ",
                chance: 30,
                social_class: 0,
            },
            History {
                history: "brown hair, ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "auburn hair, ",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "red hair, ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "blonde hair, ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "and a very dark complexion.",
                chance: 10,
                social_class: 0,
            },
            History {
                history: "and a dark complexion.",
                chance: 30,
                social_class: 0,
            },
            History {
                history: "and an average complexion.",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "and a fair complexion.",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "and a very fair complexion.",
                chance: 100,
                social_class: 0,
            },
        ],
    ]
}

pub fn halftroll() -> Vec<Vec<History>> {
    vec![
        vec![
            History {
                history: "Your mother was a Cave-Troll ",
                chance: 30,
                social_class: -30,
            },
            History {
                history: "Your father was a Cave-Troll ",
                chance: 60,
                social_class: -25,
            },
            History {
                history: "Your mother was a Hill-Troll ",
                chance: 75,
                social_class: -20,
            },
            History {
                history: "Your father was a Hill-Troll ",
                chance: 90,
                social_class: -15,
            },
            History {
                history: "Your mother was a Water-Troll ",
                chance: 95,
                social_class: -10,
            },
            History {
                history: "Your father was a Water-Troll ",
                chance: 100,
                social_class: -5,
            },
        ],
        vec![
            History {
                history: "cook.  ",
                chance: 5,
                social_class: 10,
            },
            History {
                history: "warrior.  ",
                chance: 95,
                social_class: 5,
            },
            History {
                history: "shaman.  ",
                chance: 99,
                social_class: 15,
            },
            History {
                history: "clan chief.  ",
                chance: 100,
                social_class: 30,
            },
        ],
        vec![
            History {
                history: "You have slime green eyes, ",
                chance: 60,
                social_class: 0,
            },
            History {
                history: "You have puke yellow eyes, ",
                chance: 85,
                social_class: 0,
            },
            History {
                history: "You have blue-bloodshot eyes, ",
                chance: 99,
                social_class: 0,
            },
            History {
                history: "You have glowing red eyes, ",
                chance: 100,
                social_class: 5,
            },
        ],
        vec![
            History {
                history: "dirty ",
                chance: 33,
                social_class: 0,
            },
            History {
                history: "mangy ",
                chance: 66,
                social_class: 0,
            },
            History {
                history: "oily ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "sea-weed green hair, ",
                chance: 33,
                social_class: 0,
            },
            History {
                history: "bright red hair, ",
                chance: 66,
                social_class: 0,
            },
            History {
                history: "dark purple hair, ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "and green ",
                chance: 25,
                social_class: 0,
            },
            History {
                history: "and blue ",
                chance: 50,
                social_class: 0,
            },
            History {
                history: "and white ",
                chance: 75,
                social_class: 0,
            },
            History {
                history: "and black ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "ulcerous skin.",
                chance: 33,
                social_class: 0,
            },
            History {
                history: "scabby skin.",
                chance: 66,
                social_class: 0,
            },
            History {
                history: "leprous skin.",
                chance: 100,
                social_class: 0,
            },
        ],
    ]
}

pub fn phraint() -> Vec<Vec<History>> {
    vec![
        vec![
            History {
                history: "You are one of many illegitimate children ",
                chance: 5,
                social_class: -30,
            },
            History {
                history: "You are one of several illegitimate children ",
                chance: 10,
                social_class: -25,
            },
            History {
                history: "You are one of many children ",
                chance: 50,
                social_class: -10,
            },
            History {
                history: "You are one of several children ",
                chance: 75,
                social_class: -5,
            },
            History {
                history: "You are the 2nd child ",
                chance: 95,
                social_class: 0,
            },
            History {
                history: "You are the only child ",
                chance: 100,
                social_class: 5,
            },
        ],
        vec![
            History {
                history: "of a worker.  ",
                chance: 50,
                social_class: 15,
            },
            History {
                history: "of a warrior.  ",
                chance: 75,
                social_class: 30,
            },
            History {
                history: "of an elite warrior.  ",
                chance: 90,
                social_class: 50,
            },
            History {
                history: "of the hive mother.  ",
                chance: 100,
                social_class: 100,
            },
        ],
        vec![
            History {
                history: "You are the outcast of the hive.  ",
                chance: 5,
                social_class: -50,
            },
            History {
                history: "You are the black sheep of the hive.  ",
                chance: 20,
                social_class: -30,
            },
            History {
                history: "You are a credit to the hive.  ",
                chance: 80,
                social_class: 5,
            },
            History {
                history: "You are a well liked child.  ",
                chance: 100,
                social_class: 10,
            },
        ],
        vec![
            History {
                history: "You have small ",
                chance: 40,
                social_class: 0,
            },
            History {
                history: "You have large ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "You have very large ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "buggy green eyes, ",
                chance: 10,
                social_class: 0,
            },
            History {
                history: "buggy silver eyes, ",
                chance: 30,
                social_class: 0,
            },
            History {
                history: "iridescent eyes, ",
                chance: 70,
                social_class: 0,
            },
            History {
                history: "glowing eyes, ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "straight feelers, ",
                chance: 10,
                social_class: 0,
            },
            History {
                history: "curved feelers, ",
                chance: 30,
                social_class: 0,
            },
            History {
                history: "bent feelers, ",
                chance: 80,
                social_class: 0,
            },
            History {
                history: "very long feelers, ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "and dull brown chiton. ",
                chance: 10,
                social_class: 0,
            },
            History {
                history: "and shiny brown chiton. ",
                chance: 60,
                social_class: 0,
            },
            History {
                history: "and shiny black chiton. ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "and polished silver chiton. ",
                chance: 100,
                social_class: 0,
            },
        ],
    ]
}

pub fn dryad() -> Vec<Vec<History>> {
    vec![
        vec![
            History {
                history: "You are the Dryad of a sickly ",
                chance: 15,
                social_class: -20,
            },
            History {
                history: "You are the Dryad of a large ",
                chance: 40,
                social_class: 0,
            },
            History {
                history: "You are the Dryad of a rich, green ",
                chance: 60,
                social_class: 0,
            },
            History {
                history: "You are the Dryad of a magnificent ",
                chance: 90,
                social_class: 10,
            },
        ],
        vec![
            History {
                history: "pine tree",
                chance: 30,
                social_class: 5,
            },
            History {
                history: "birch tree",
                chance: 40,
                social_class: 15,
            },
            History {
                history: "ash tree",
                chance: 50,
                social_class: 30,
            },
            History {
                history: "cedar tree",
                chance: 70,
                social_class: 50,
            },
            History {
                history: "willow tree",
                chance: 90,
                social_class: 70,
            },
            History {
                history: "oak tree",
                chance: 100,
                social_class: 90,
            },
        ],
        vec![
            History {
                history: ", but the elven community avoids your forest.  ",
                chance: 10,
                social_class: -30,
            },
            History {
                history: " in a small glade.  ",
                chance: 40,
                social_class: -5,
            },
            History {
                history: " and you are a fine upholder of the woodlands.  ",
                chance: 60,
                social_class: 5,
            },
            History {
                history: " and Humans and Half-Elves hold your tree sacred.  ",
                chance: 88,
                social_class: 20,
            },
            History {
                history: " where the Elves hold yearly ceremonies.  ",
                chance: 90,
                social_class: 25,
            },
            History {
                history: " that all races hold in reverence.  ",
                chance: 100,
                social_class: 30,
            },
        ],
        vec![
            History {
                history: "You have light grey eyes, ",
                chance: 85,
                social_class: 0,
            },
            History {
                history: "You have light violet eyes, ",
                chance: 90,
                social_class: 0,
            },
            History {
                history: "You have light blue eyes, ",
                chance: 95,
                social_class: 0,
            },
            History {
                history: "You have light green eyes, ",
                chance: 98,
                social_class: 2,
            },
            History {
                history: "You have light golden colored eyes, ",
                chance: 100,
                social_class: 5,
            },
        ],
        vec![
            History {
                history: "straight ",
                chance: 75,
                social_class: 0,
            },
            History {
                history: "wavey ",
                chance: 100,
                social_class: 0,
            },
        ],
        vec![
            History {
                history: "jet black hair, and a fair complexion.",
                chance: 75,
                social_class: 0,
            },
            History {
                history: "light brown hair, and a fair complexion.",
                chance: 85,
                social_class: 0,
            },
            History {
                history: "blonde hair, and a fair complexion.",
                chance: 95,
                social_class: 0,
            },
            History {
                history: "silver hair, and a fair complexion.",
                chance: 98,
                social_class: 1,
            },
            History {
                history: "hair the color of spun gold and pale skin.",
                chance: 100,
                social_class: 5,
            },
        ],
    ]
}
