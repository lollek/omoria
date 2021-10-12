use model::{ Class, Sex, StatBlock };
use types::{Ability};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Race {
    Human = 0,
    HalfElf = 1,
    Elf = 2,
    Halfling = 3,
    Gnome = 4,
    Dwarf = 5,
    HalfOrc = 6,
    HalfTroll = 7,
    Phraint = 8,
    Dryad = 9
}

impl Race {
    pub fn name(&self) -> &'static str {
        match self {
            Race::Human => "Human",
            Race::HalfElf => "Half-Elf",
            Race::Elf => "Elf",
            Race::Halfling => "Halfling",
            Race::Gnome => "Gnome",
            Race::Dwarf => "Dwarf",
            Race::HalfOrc => "Half-Orc",
            Race::HalfTroll => "Half-Troll",
            Race::Phraint => "Phraint",
            Race::Dryad => "Dryad"
        }
    }

    pub fn info(&self) -> &'static str {
        match self {
            Race::Human => "Humans possess exceptional drive and a great capacity, to endure and expand, and as such are currently the, dominant race in the world. Their empires and nations are vast, sprawling things, and the citizens of these societies carve names for themselves with the strength of their sword arms and the power of their spells. Humanity is best characterized by its tumultuousness and diversity, and human cultures run the gamut from savage but honorable tribes to decadent, devil-worshiping noble families in the most cosmopolitan cities. Human curiosity and ambition often triumph over their predilection for a sedentary lifestyle, and many leave their homes to explore the innumerable forgotten corners of the world or lead mighty armies to conquer their neighbors, simply because they can.",
            Race::HalfElf => "Elves have long drawn the covetous gazes of other races. Their generous life spans, magical affinity, and inherent grace each contribute to the admiration or bitter envy of their neighbors. Of all their traits, however, none so entrance their human associates as their beauty. Since the two races first came into contact with each other, the humans have held up elves as models of physical perfection, seeing in the fair folk idealized versions of themselves. For their part, many elves find humans attractive despite their comparatively barbaric ways, drawn to the passion and impetuosity with which members of the younger race play out their brief lives. Sometimes this mutual infatuation leads to romantic relationships. Though usually short-lived, even by human standards, these trysts commonly lead to the birth of half-elves, a race descended of two cultures yet inheritor of neither. Half-elves can breed with one another, but even these `pureblood` half-elves tend to be viewed as bastards by humans and elves alike.",
            Race::Elf => "The long-lived elves are children of the natural world, similar in many superficial ways to fey creatures, yet different as well. Elves value their privacy and traditions, and while they are often slow to make friends, at both the personal and national levels, once an outsider is accepted as a comrade, such alliances can last for generations. Elves have a curious attachment to their surroundings, perhaps as a result of their incredibly long lifespans or some deeper, more mystical reason. Elves who dwell in a region for long find themselves physically adapting to match their surroundings, most noticeably taking on coloration reflecting the local environment. Those elves that spend their lives among the short-lived races, on the other hand, often develop a skewed perception of mortality and become morose, the result of watching wave after wave of companions age and die before their eyes.",
            Race::Halfling => "Optimistic and cheerful by nature, blessed with uncanny luck and driven by a powerful wanderlust, halflings make up for their short stature with an abundance of bravado and curiosity. At once excitable and easy-going, halflings like to keep an even temper and a steady eye on opportunity, and are not as prone as some of the more volatile races to violent or emotional outbursts. Even in the jaws of catastrophe, a halfling almost never loses his sense of humor. Halflings are inveterate opportunists. Unable to physically defend themselves from the rigors of the world, they know when to bend with the wind and when to hide away. Yet a halfling's curiosity often overwhelms his good sense, leading to poor decisions and narrow escapes. Though their curiosity drives them to travel and seek new places and experiences, halflings possess a strong sense of house and home, often spending above their means to enhance the comforts of home life.",
            Race::Gnome => "Gnomes trace their lineage back to the mysterious realm of the fey, a place where colors are brighter, the wildlands wilder, and emotions more primal. Unknown forces drove the ancient gnomes from that realm long ago, forcing them to seek refuge in this world; despite this, the gnomes have never completely abandoned their fey roots or adapted to mortal culture. As a result, gnomes are widely regarded by the other races as alien and strange.",
            Race::Dwarf => "Dwarves are a stoic but stern race, ensconced in cities carved from the hearts of mountains and fiercely determined to repel the depredations of savage races like orcs and goblins. More than any other race, the dwarves have acquired a reputation as dour and humorless craftsmen of the earth. It could be said that dwarven history shapes the dark disposition of many dwarves, for they reside in high mountains and dangerous realms below the earth, constantly at war with giants, goblins, and other such horrors.",
            Race::HalfOrc => "Half-orcs are monstrosities, their tragic births the result of perversion and violence - or at least, that's how other races see them. It's true that half-orcs are rarely the result of loving unions, and as such are usually forced to grow up hard and fast, constantly fighting for protection or to make names for themselves. Feared, distrusted, and spat upon, half-orcs still consistently manage to surprise their detractors with great deeds and unexpected wisdom - though sometimes it's easier just to crack a few skulls.",
            Race::HalfTroll => "Trolls possess incredibly sharp claws and amazing regenerative powers, allowing them to recover from nearly any wound. They are stooped, fantastically ugly, and astonishingly strong - combined with their claws, their strength allows them to literally tear apart flesh to feed their voracious appetites. Trolls stand about 14 feet tall, but their hunched postures often make them appear shorter. An adult troll weighs around 1,000 pounds. A  troll's appetite and its regenerative powers make it a fearless combatant, ever prepared to charge headlong at the nearest living creature and attack with all of its fury. Only fire seems to cause a troll to hesitate, but even this mortal threat is not enough to stop a troll's advance. Despite their cruelty in combat, trolls are surprisingly tender and kind to their own young. Female trolls work as a group, spending a great deal of time teaching young trolls to hunt and fend for themselves before sending them off to find their own territories. A male troll tends to live a solitary existence, partnering with a female for only a brief time to mate. All trolls spend most of their time hunting for food, as they must consume vast amounts each day or face starvation.",
            Race::Phraint => "Phraints are a cold, emotionless insectile race that claims an origin on other worlds among the stars, though the truth is lost to time. Except for a very rare few, they are incapable of either feeling or comprehending emotion. Other races may employ them as mercenaries, as they are known for their intense loyalty and adherence to the specifics of their contracts. They are considered alien and strange by other races, even by such as the fey.",
            Race::Dryad => "Dryads are tree-fey who prefer secluded woodlands far from humanoids in need of lumber. Dryads' main interests are their own survival and that of their beloved forests, and they have been known to magically coerce passersby into aiding them in tasks they cannot complete. They are more likely to be friendly to non-evil druids and rangers, as they recognize a mutual respect for or empathy with nature. Dryads are benign guardians of trees, and though they can do little in the way of direct violence, they can trap and disable threats to their homes or turn enemies into allies. Some keep one or more charmed humanoids in their territory to fend off or lead away attackers. Incapacitated foes are typically dragged to the edge of the forest by the dryad's allies and left there, but evil or overtly hostile ones are killed once combat is over.",
        }
    }

    pub fn stats_info(&self) -> Vec<String> {
        let stats = self.stat_block();
        vec![
            format!("Melee bonus:       {}", self.melee_bonus()),
            format!("Ranged bonus:      {}", self.ranged_bonus()),
            format!("Experience factor: {}", self.expfactor()),
            format!("Search frequence:  {}", self.search_freq()),
            format!("Search modifier:   {}", self.search_mod()),
            format!("Stealth modifier:  {}", self.stealth_mod()),
            format!("Save modifier:     {}", self.save_mod()),
            format!("Disarm modifier:   {}", self.disarm_mod()),
            format!("Infravision:       {}", self.infravision()),
            format!("Swim speed:        {}", self.swim_speed()),
            "ATTRIBUTES:".to_owned(),
            format!("Strength:          {}", stats.strength),
            format!("Dexterity:         {}", stats.dexterity),
            format!("Constituton:       {}", stats.constitution),
            format!("Intelligence:      {}", stats.intelligence),
            format!("Wisdom:            {}", stats.wisdom),
            format!("Charisma:          {}", stats.charisma),
        ]
    }

    pub fn search_mod(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 6,
            Race::Elf => 8,
            Race::Halfling => 12,
            Race::Gnome => 6,
            Race::Dwarf => 7,
            Race::HalfOrc => 0,
            Race::HalfTroll => -1,
            Race::Phraint => 10,
            Race::Dryad => 6,
        }
    }

    pub fn melee_bonus(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 0,
            Race::Elf => -5,
            Race::Halfling => -10,
            Race::Gnome => -8,
            Race::Dwarf => 15,
            Race::HalfOrc => 12,
            Race::HalfTroll => 20,
            Race::Phraint => 3,
            Race::Dryad => 0,
        }
    }

    pub fn ranged_bonus(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 5,
            Race::Elf => 15,
            Race::Halfling => 20,
            Race::Gnome => 12,
            Race::Dwarf => 0,
            Race::HalfOrc => -5,
            Race::HalfTroll => -10,
            Race::Phraint => 5,
            Race::Dryad => 5,
        }
    }

    pub fn search_freq(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => -1,
            Race::Elf => -1,
            Race::Halfling => -5,
            Race::Gnome => -3,
            Race::Dwarf => 0,
            Race::HalfOrc => 3,
            Race::HalfTroll => 5,
            Race::Phraint => 3,
            Race::Dryad => -1,
        }
    }

    pub fn stealth_mod(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 1,
            Race::Elf => 1,
            Race::Halfling => 4,
            Race::Gnome => 3,
            Race::Dwarf => 0,
            Race::HalfOrc => -1,
            Race::HalfTroll => -2,
            Race::Phraint => 5,
            Race::Dryad => 1,
        }
    }

    pub fn save_mod(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 3,
            Race::Elf => 6,
            Race::Halfling => 18,
            Race::Gnome => 12,
            Race::Dwarf => 9,
            Race::HalfOrc => -3,
            Race::HalfTroll => -9,
            Race::Phraint => -3,
            Race::Dryad => 3,
        }
    }

    pub fn disarm_mod(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 2,
            Race::Elf => 5,
            Race::Halfling => 15,
            Race::Gnome => 10,
            Race::Dwarf => 2,
            Race::HalfOrc => -3,
            Race::HalfTroll => -5,
            Race::Phraint => 15,
            Race::Dryad => 2,
        }
    }

    pub fn expfactor(&self) -> f32 {
        match self {
            Race::Human => 1.00,
            Race::HalfElf => 1.10,
            Race::Elf => 1.20,
            Race::Halfling => 1.10,
            Race::Gnome => 1.25,
            Race::Dwarf => 1.20,
            Race::HalfOrc => 1.10,
            Race::HalfTroll => 1.20,
            Race::Phraint => 1.20,
            Race::Dryad => 1.20,
        }
    }

    pub fn infravision(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 0,
            Race::Elf => 0,
            Race::Halfling => 4,
            Race::Gnome => 3,
            Race::Dwarf => 5,
            Race::HalfOrc => 3,
            Race::HalfTroll => 3,
            Race::Phraint => 5,
            Race::Dryad => 3,
        }
    }

    pub fn swim_speed(&self) -> i8 {
        match self {
            Race::Human => 0,
            Race::HalfElf => 1,
            Race::Elf => 2,
            Race::Halfling => -2,
            Race::Gnome => -1,
            Race::Dwarf => -2,
            Race::HalfOrc => 0,
            Race::HalfTroll => 2,
            Race::Phraint => -1,
            Race::Dryad => -1,
        }
    }

    pub fn stat_block(&self) -> StatBlock {
        match self {
            Race::Human => StatBlock::from([0, 0, 0, 0, 0, 0]),
            Race::HalfElf => StatBlock::from([-1, 1, 0, 1, -1, 1]),
            Race::Elf => StatBlock::from([-1, 2, 1, 1, -2, 1]),
            Race::Halfling => StatBlock::from([-2, 2, 1, 3, 1, 1]),
            Race::Gnome => StatBlock::from([-1, 2, 0, 2, 1, -2]),
            Race::Dwarf => StatBlock::from([2, -3, 1, -2, 2, -3]),
            Race::HalfOrc => StatBlock::from([2, -1, 0, 0, 1, -4]),
            Race::HalfTroll => StatBlock::from([ 4, -4, -3, -4,  4, -6]),
            Race::Phraint => StatBlock::from([ 0,  0, -4,  5,  0, -3]),
            Race::Dryad => StatBlock::from([-1,  0,  3,  0, -2,  3]),
        }
    }

    pub fn available_classes(&self) -> Vec<Class> {
        match self {
            Race::Human => vec![
                Class::Fighter, Class::Wizard, Class::Cleric, Class::Rogue,
                Class::Ranger, Class::Paladin, Class::Druid, Class::Bard,
                Class::Adventurer, Class::Monk, Class::Barbarian,
            ],
            Race::HalfElf => vec![
                Class::Fighter, Class::Wizard, Class::Cleric, Class::Rogue,
                Class::Ranger, Class::Paladin, Class::Druid, Class::Bard,
                Class::Adventurer, Class::Monk,
            ],
            Race::Elf => vec![
                Class::Fighter, Class::Wizard, Class::Cleric, Class::Rogue,
                Class::Ranger, Class::Druid, Class::Bard, Class::Adventurer,
            ],
            Race::Halfling => vec![
                Class::Rogue, Class::Druid, Class::Bard, Class::Adventurer,
                Class::Monk,
            ],
            Race::Gnome => vec![
                Class::Wizard, Class::Cleric, Class::Rogue, Class::Druid,
            ],
            Race::Dwarf => vec![
                Class::Fighter, Class::Cleric, Class::Druid, Class::Barbarian,
            ],
            Race::HalfOrc => vec![
                Class::Fighter, Class::Cleric, Class::Rogue, Class::Monk,
                Class::Barbarian,
            ],
            Race::HalfTroll => vec![
                Class::Fighter, Class::Cleric, Class::Barbarian,
            ],
            Race::Phraint => vec![
                Class::Fighter, Class::Wizard, Class::Rogue, Class::Ranger,
                Class::Bard, Class::Adventurer, Class::Monk,
            ],
            Race::Dryad => vec![
                Class::Cleric, Class::Ranger, Class::Druid, Class::Bard,
                Class::Monk,
            ],
        }
    }

    pub fn weight_base(&self, player_sex: Sex) -> u16 {
        match self {
            Race::Human =>
                match player_sex {
                    Sex::Male => 180,
                    Sex::Female => 120,
                }
            Race::HalfElf =>
                match player_sex {
                    Sex::Male => 130,
                    Sex::Female => 100,
                }
            Race::Elf =>
                match player_sex {
                    Sex::Male => 100,
                    Sex::Female => 80,
                }
            Race::Halfling =>
                match player_sex {
                    Sex::Male => 60,
                    Sex::Female => 50,
                }
            Race::Gnome =>
                match player_sex {
                    Sex::Male => 90,
                    Sex::Female => 75,
                }
            Race::Dwarf =>
                match player_sex {
                    Sex::Male => 150,
                    Sex::Female => 120,
                }
            Race::HalfOrc =>
                match player_sex {
                    Sex::Male => 150,
                    Sex::Female => 120,
                }
            Race::HalfTroll =>
                match player_sex {
                    Sex::Male => 300,
                    Sex::Female => 260,
                }
            Race::Phraint =>
                match player_sex {
                    Sex::Male => 100,
                    Sex::Female => 95,
                }
            Race::Dryad =>
                match player_sex {
                    Sex::Male => 85,
                    Sex::Female => 70,
                }
        }
    }

    pub fn weight_modifier(&self, player_sex: Sex) -> u16 {
        match self {
            Race::Human =>
                match player_sex {
                    Sex::Male => 25,
                    Sex::Female => 20,
                }
            Race::HalfElf =>
                match player_sex {
                    Sex::Male => 15,
                    Sex::Female => 10,
                }
            Race::Elf => 6,
            Race::Halfling => 3,
            Race::Gnome =>
                match player_sex {
                    Sex::Male => 6,
                    Sex::Female => 3,
                }
            Race::Dwarf => 10,
            Race::HalfOrc => 5,
            Race::HalfTroll =>
                match player_sex {
                    Sex::Male => 50,
                    Sex::Female => 40,
                }
            Race::Phraint =>
                match player_sex {
                    Sex::Male => 20,
                    Sex::Female => 16,
                }
            Race::Dryad => 6,
        }
    }

    pub fn abilities(&self) -> Vec<Ability> {
        match self {
            _ => Vec::new(),
        }
    }
}

impl From<usize> for Race {
    fn from(pos: usize) -> Self {
        match pos {
            0 => Race::Human,
            1 => Race::HalfElf,
            2 => Race::Elf,
            3 => Race::Halfling,
            4 => Race::Gnome,
            5 => Race::Dwarf,
            6 => Race::HalfOrc,
            7 => Race::HalfTroll,
            8 => Race::Phraint,
            9 => Race::Dryad,
            _ => panic!("pos out of range")
        }
    }
}

pub fn races_iter() -> impl Iterator<Item=usize> {
    (Race::Human as usize)..(Race::Dryad as usize + 1)
}

