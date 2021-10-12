use types::{ Magic, Item };
use model::Ability;
use template;

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Class {
    Fighter = 0,
    Wizard = 1,
    Cleric = 2,
    Rogue = 3,
    Ranger = 4,
    Paladin = 5,
    Druid = 6,
    Bard = 7,
    Adventurer = 8,
    Monk = 9,
    Barbarian = 10,
}

impl Class {
    pub fn name(&self) -> &'static str {
        match self {
            Class::Fighter => "Fighter",
            Class::Wizard => "Wizard",
            Class::Cleric => "Cleric",
            Class::Rogue => "Rogue",
            Class::Ranger => "Ranger",
            Class::Paladin => "Paladin",
            Class::Druid => "Druid",
            Class::Bard => "Bard",
            Class::Adventurer => "Adventurer",
            Class::Monk => "Monk",
            Class::Barbarian => "Barbarian",
        }
    }

    pub fn info(&self) -> &'static str {
        match self {
            Class::Fighter => "Some take up arms for glory, wealth, or revenge. Others do battle to prove themselves, to protect others, or because they know nothing else. Still others learn the ways of weaponcraft to hone their bodies in battle and prove their mettle in the forge of war. Lords of the battlefield, fighters are a disparate lot, training with many weapons or just one, perfecting the uses of armor, learning the fighting techniques of exotic masters, and studying the art of combat, all to shape themselves into living weapons. Far more than mere thugs, these skilled warriors reveal the true deadliness of their weapons, turning hunks of metal into arms capable of taming kingdoms, slaughtering monsters, and rousing the hearts of armies.  Soldiers, knights, hunters, and artists of war, fighters are unparalleled champions, and woe to those who dare stand against them.",
            Class::Wizard => "Beyond the veil of the mundane hide the secrets of absolute power. The works of beings beyond mortals, the legends of realms where gods and spirits tread, the lore of creations both wondrous and terrible-such mysteries call to those with the ambition and the intellect to rise above the common folk to grasp true might. Such is the path of the wizard. These shrewd magic-users seek, collect, and covet esoteric knowledge, drawing on cultic arts to work wonders beyond the abilities of mere mortals. While some might choose a particular field of magical study and become masters of such powers, others embrace versatility, reveling in the unbounded wonders of all magic. In either case, wizards prove a cunning and potent lot, capable of smiting their foes, empowering their allies, and shaping the world to their every desire.",
            Class::Cleric => "In faith and the miracles of the divine, many find a greater purpose. Called to serve powers beyond most mortal understanding, all priests preach wonders and provide for the spiritual needs of their people. Clerics are more than mere priests, though; these emissaries of the divine work are the will of their deities through strength of arms and the magic of their gods. Devoted to the tenets of the religions and philosophies that inspire them, these ecclesiastics quest to spread the knowledge and inf luence of their faith. Yet while they might share similar abilities, clerics prove as different from one another as the divinities they serve, with some offering healing and redemption, others judging law and truth, and still others spreading conflict and corruption. The ways of the cleric are varied, yet all who tread these paths walk with the mightiest of allies and bear the arms of the gods themselves.",
            Class::Rogue => "Life is an endless adventure for those who live by their wits. Ever just one step ahead of danger, rogues bank on their cunning, skill, and charm to bend fate to their favor. Never knowing what to expect, they prepare for everything, becoming masters of a wide variety of skills, training themselves to be adept manipulators, agile acrobats, shadowy stalkers, or masters of any of dozens of other professions or talents. Thieves and gamblers, fast talkers and diplomats, bandits and bounty hunters, and explorers and investigators all might be considered rogues, as well as countless other professions that rely upon wits, prowess, or luck. Although many rogues favor cities and the innumerable opportunities of civilization, some embrace lives on the road, journeying far, meeting exotic people, and facing fantastic danger in pursuit of equally fantastic riches. In the end, any who desire to shape their fates and live life on their own terms might come to be called rogues.",
            Class::Ranger => "For those who relish the thrill of the hunt, there are only predators and prey. Be they scouts, trackers, or bounty hunters, rangers share much in common: unique mastery of specialized weapons, skill at stalking even the most elusive game, and the expertise to defeat a wide range of quarries. Knowledgeable, patient, and skilled hunters, these rangers hound man, beast, and monster alike, gaining insight into the way of the predator, skill in varied environments, and ever more lethal martial prowess. While some track man-eating creatures to protect the frontier, others pursue more cunning game- even fugitives among their own people.",
            Class::Paladin => "Through a select, worthy few shines the power of the divine.  Called paladins, these noble souls dedicate their swords and lives to the battle against evil. Knights, crusaders, and law-bringers, paladins seek not just to spread divine justice but to embody the teachings of the virtuous deities they serve.  In pursuit of their lofty goals, they adhere to ironclad laws of morality and discipline. As reward for their righteousness, these holy champions are blessed with boons to aid them in their quests: powers to banish evil, heal the innocent, and inspire the faithful. Although their convictions might lead them into conflict with the very souls they would save, paladins weather endless challenges of faith and dark temptations, risking their lives to do right and fighting to bring about a brighter future.",
            Class::Druid => "Within the purity of the elements and the order of the wilds lingers a power beyond the marvels of civilization.  Furtive yet undeniable, these primal magics are guarded over by servants of philosophical balance known as druids.  Allies to beasts and manipulators of nature, these often misunderstood protectors of the wild strive to shield their lands from all who would threaten them and prove the might of the wilds to those who lock themselves behind city walls. Rewarded for their devotion with incredible powers, druids gain unparalleled shape-shifting abilities, the companionship of mighty beasts, and the power to call upon nature's wrath. The mightiest temper powers akin to storms, earthquakes, and volcanoes with primeval wisdom long abandoned and forgotten by civilization.",
            Class::Bard => "Untold wonders and secrets exist for those skillful enough to discover them. Through cleverness, talent, and magic, these cunning few unravel the wiles of the world, becoming adept in the arts of persuasion, manipulation, and inspiration. Typically masters of one or many forms of artistry, bards possess an uncanny ability to know more than they should and use what they learn to keep themselves and their allies ever one step ahead of danger.  Bards are quick-witted and captivating, and their skills might lead them down many paths, be they gamblers or jacks-of-all-trades, scholars or performers, leaders or scoundrels, or even all of the above. For bards, every day brings its own opportunities, adventures, and challenges, and only by bucking the odds, knowing the most, and being the best might they claim the treasures of each.",
            Class::Adventurer => "",
            Class::Monk => "For the truly exemplary, martial skill transcends the battlefield-it is a lifestyle, a doctrine, a state of mind.  These warrior-artists search out methods of battle beyond swords and shields, finding weapons within themselves just as capable of crippling or killing as any blade. These monks (so called since they adhere to ancient philosophies and strict martial disciplines) elevate their bodies to become weapons of war, from battle-minded ascetics to self-taught brawlers. Monks tread the path of discipline, and those with the will to endure that path discover within themselves not what they are, but what they are meant to be.",
            Class::Barbarian => "Barbarians excel in combat, possessing the martial prowess and fortitude to take on foes seemingly far superior to themselves. With rage granting them boldness and daring beyond that of most other warriors, barbarians charge furiously into battle and ruin all who would stand in their way.",
        }
    }

    pub fn restriction_info(&self) -> &'static str {
        match self {
            Class::Druid => "Can only use the following weapons: Club, Dagger, Dart, Quarterstaff, Scimitar, Scythe, Sickle, Shortspear, Sling, Spear. For armor, shields and Misc. items: Cannot wear anything consisting of a lot of metal. Also can not use any large shields",
            Class::Barbarian => "Cannot use heavy armor",
            _ => "No restrictions!",
        }
    }

    pub fn health_bonus(&self) -> u8 {
        match self {
            Class::Fighter => 10,
            Class::Wizard => 6,
            Class::Cleric => 8,
            Class::Rogue => 8,
            Class::Ranger => 10,
            Class::Paladin => 10,
            Class::Druid => 8,
            Class::Bard => 8,
            Class::Adventurer => 10,
            Class::Monk => 8,
            Class::Barbarian => 12,
        }
    }

    pub fn melee_bonus(&self) -> i8 {
        match self {
            Class::Fighter => 10,
            Class::Wizard => 4,
            Class::Cleric => 6,
            Class::Rogue => 6,
            Class::Ranger => 6,
            Class::Paladin => 8,
            Class::Druid => 4,
            Class::Bard => 5,
            Class::Adventurer => 6,
            Class::Monk => 8,
            Class::Barbarian => 10,
        }
    }

    pub fn ranged_bonus(&self) -> i8 {
        match self {
            Class::Fighter => 10,
            Class::Wizard => 4,
            Class::Cleric => 5,
            Class::Rogue => 10,
            Class::Ranger => 10,
            Class::Paladin => 6,
            Class::Druid => 7,
            Class::Bard => 6,
            Class::Adventurer => 6,
            Class::Monk => 6,
            Class::Barbarian => 10,
        }
    }

    pub fn search_mod(&self) -> i8 {
        match self {
            Class::Fighter => 14,
            Class::Wizard => 16,
            Class::Cleric => 16,
            Class::Rogue => 32,
            Class::Ranger => 24,
            Class::Paladin => 12,
            Class::Druid => 16,
            Class::Bard => 22,
            Class::Adventurer => 24,
            Class::Monk => 24,
            Class::Barbarian => 14,
        }
    }

    pub fn disarm_mod(&self) -> i8 {
        match self {
            Class::Fighter => 25,
            Class::Wizard => 30,
            Class::Cleric => 25,
            Class::Rogue => 45,
            Class::Ranger => 30,
            Class::Paladin => 20,
            Class::Druid => 25,
            Class::Bard => 30,
            Class::Adventurer => 30,
            Class::Monk => 45,
            Class::Barbarian => 25,
        }
    }

    pub fn stealth_mod(&self) -> i8 {
        match self {
            Class::Fighter => 1,
            Class::Wizard => 2,
            Class::Cleric => 2,
            Class::Rogue => 4,
            Class::Ranger => 3,
            Class::Paladin => 1,
            Class::Druid => 1,
            Class::Bard => 2,
            Class::Adventurer => 3,
            Class::Monk => 3,
            Class::Barbarian => 1,
        }
    }

    pub fn search_freq(&self) -> i8 {
        match self {
            Class::Fighter => 38,
            Class::Wizard => 36,
            Class::Cleric => 32,
            Class::Rogue => 16,
            Class::Ranger => 24,
            Class::Paladin => 38,
            Class::Druid => 32,
            Class::Bard => 28,
            Class::Adventurer => 24,
            Class::Monk => 24,
            Class::Barbarian => 38,
        }
    }

    pub fn save_mod(&self) -> i8 {
        match self {
            Class::Fighter => 10,
            Class::Wizard => 25,
            Class::Cleric => 20,
            Class::Rogue => 15,
            Class::Ranger => 20,
            Class::Paladin => 15,
            Class::Druid => 20,
            Class::Bard => 20,
            Class::Adventurer => 20,
            Class::Monk => 25,
            Class::Barbarian => 10,
        }
    }

    pub fn magic_resist(&self) -> i8 {
        match self {
            Class::Fighter => -10,
            Class::Wizard => 0,
            Class::Cleric => 0,
            Class::Rogue => -5,
            Class::Ranger => -5,
            Class::Paladin => -5,
            Class::Druid => -5,
            Class::Bard => -5,
            Class::Adventurer => -5,
            Class::Monk => -5,
            Class::Barbarian => -10,
        }
    }

    pub fn expfactor(&self) -> f32 {
        match self {
            Class::Fighter => 0.0,
            Class::Wizard => 0.3,
            Class::Cleric => 0.2,
            Class::Rogue => 0.1,
            Class::Ranger => 0.3,
            Class::Paladin => 0.35,
            Class::Druid => 0.2,
            Class::Bard => 0.3,
            Class::Adventurer => 0.4,
            Class::Monk => 0.1,
            Class::Barbarian => 0.0,
        }
    }

    pub fn abilities(&self) -> Vec<Ability> {
        match self {
            Class::Barbarian => [Ability::Rage].to_vec(),
            _ => Vec::new(),
        }
    }


    pub fn magic_type(&self) -> Option<Magic> {
        match self {
            Class::Wizard | Class::Adventurer => Some(Magic::Arcane),
            Class::Cleric | Class::Paladin => Some(Magic::Divine),
            Class::Druid | Class::Ranger => Some(Magic::Nature),
            Class::Bard | Class::Rogue => Some(Magic::Song),
            Class::Monk => Some(Magic::Chakra),
            Class::Fighter => None,
            Class::Barbarian => None,
        }
    }


    pub fn starting_items(&self) -> Vec<Item> {
        match self {
            Class::Fighter => vec![
                template::weapon::stiletto(),
            ],
            Class::Wizard => vec![
                template::weapon::stiletto(),
                template::magic_token::magic_book(),
                ],
            Class::Cleric => vec![
                template::weapon::quarterstaff(),
                template::magic_token::prayer_book(),
            ],
            Class::Rogue => vec![
                template::weapon::stiletto(),
                template::magic_token::song_book(),
            ],
            Class::Ranger => vec![
                template::weapon::stiletto(),
                template::magic_token::instrument(),
            ],
            Class::Paladin => vec![
                template::weapon::stiletto(),
                template::magic_token::prayer_book(),
            ],
            Class::Druid => vec![
                template::weapon::quarterstaff(),
                template::magic_token::instrument(),
            ],
            Class::Bard => vec![
                template::weapon::stiletto(),
                template::magic_token::song_book(),
            ],
            Class::Adventurer => vec![
                template::weapon::stiletto(),
                template::magic_token::magic_book(),
            ],
            Class::Monk => vec![
            ],
            Class::Barbarian => vec![
                template::weapon::stiletto(),
            ],
        }
    }
}


impl From<usize> for Class {
    fn from(pos: usize) -> Self {
        match pos {
            0 => Class::Fighter,
            1 => Class::Wizard,
            2 => Class::Cleric,
            3 => Class::Rogue,
            4 => Class::Ranger,
            5 => Class::Paladin,
            6 => Class::Druid,
            7 => Class::Bard,
            8 => Class::Adventurer,
            9 => Class::Monk,
            10 => Class::Barbarian,
            _ => panic!(),
        }
    }
}


