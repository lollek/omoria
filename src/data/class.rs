use crate::generate_item;
use crate::generate_item::ItemQuality;
use crate::generate_item::template::*;
use crate::model;

pub fn name(class: &model::Class) -> &'static str {
    match class {
        model::Class::Fighter => "Fighter",
        model::Class::Wizard => "Wizard",
        model::Class::Cleric => "Cleric",
        model::Class::Rogue => "Rogue",
        model::Class::Ranger => "Ranger",
        model::Class::Paladin => "Paladin",
        model::Class::Druid => "Druid",
        model::Class::Bard => "Bard",
        model::Class::Adventurer => "Adventurer",
        model::Class::Monk => "Monk",
        model::Class::Barbarian => "Barbarian",
    }
}

pub fn info(class: &model::Class) -> &'static str {
    match class {
        model::Class::Fighter => "Some take up arms for glory, wealth, or revenge. Others do battle to prove themselves, to protect others, or because they know nothing else. Still others learn the ways of weaponcraft to hone their bodies in battle and prove their mettle in the forge of war. Lords of the battlefield, fighters are a disparate lot, training with many weapons or just one, perfecting the uses of armor, learning the fighting techniques of exotic masters, and studying the art of combat, all to shape themselves into living weapons. Far more than mere thugs, these skilled warriors reveal the true deadliness of their weapons, turning hunks of metal into arms capable of taming kingdoms, slaughtering monsters, and rousing the hearts of armies.  Soldiers, knights, hunters, and artists of war, fighters are unparalleled champions, and woe to those who dare stand against them.",
        model::Class::Wizard => "Beyond the veil of the mundane hide the secrets of absolute power. The works of beings beyond mortals, the legends of realms where gods and spirits tread, the lore of creations both wondrous and terrible-such mysteries call to those with the ambition and the intellect to rise above the common folk to grasp true might. Such is the path of the wizard. These shrewd magic-users seek, collect, and covet esoteric knowledge, drawing on cultic arts to work wonders beyond the abilities of mere mortals. While some might choose a particular field of magical study and become masters of such powers, others embrace versatility, reveling in the unbounded wonders of all magic. In either case, wizards prove a cunning and potent lot, capable of smiting their foes, empowering their allies, and shaping the world to their every desire.",
        model::Class::Cleric => "In faith and the miracles of the divine, many find a greater purpose. Called to serve powers beyond most mortal understanding, all priests preach wonders and provide for the spiritual needs of their people. Clerics are more than mere priests, though; these emissaries of the divine work are the will of their deities through strength of arms and the magic of their gods. Devoted to the tenets of the religions and philosophies that inspire them, these ecclesiastics quest to spread the knowledge and inf luence of their faith. Yet while they might share similar abilities, clerics prove as different from one another as the divinities they serve, with some offering healing and redemption, others judging law and truth, and still others spreading conflict and corruption. The ways of the cleric are varied, yet all who tread these paths walk with the mightiest of allies and bear the arms of the gods themselves.",
        model::Class::Rogue => "Life is an endless adventure for those who live by their wits. Ever just one step ahead of danger, rogues bank on their cunning, skill, and charm to bend fate to their favor. Never knowing what to expect, they prepare for everything, becoming masters of a wide variety of skills, training themselves to be adept manipulators, agile acrobats, shadowy stalkers, or masters of any of dozens of other professions or talents. Thieves and gamblers, fast talkers and diplomats, bandits and bounty hunters, and explorers and investigators all might be considered rogues, as well as countless other professions that rely upon wits, prowess, or luck. Although many rogues favor cities and the innumerable opportunities of civilization, some embrace lives on the road, journeying far, meeting exotic people, and facing fantastic danger in pursuit of equally fantastic riches. In the end, any who desire to shape their fates and live life on their own terms might come to be called rogues.",
        model::Class::Ranger => "For those who relish the thrill of the hunt, there are only predators and prey. Be they scouts, trackers, or bounty hunters, rangers share much in common: unique mastery of specialized weapons, skill at stalking even the most elusive game, and the expertise to defeat a wide range of quarries. Knowledgeable, patient, and skilled hunters, these rangers hound man, beast, and monster alike, gaining insight into the way of the predator, skill in varied environments, and ever more lethal martial prowess. While some track man-eating creatures to protect the frontier, others pursue more cunning game- even fugitives among their own people.",
        model::Class::Paladin => "Through a select, worthy few shines the power of the divine.  Called paladins, these noble souls dedicate their swords and lives to the battle against evil. Knights, crusaders, and law-bringers, paladins seek not just to spread divine justice but to embody the teachings of the virtuous deities they serve.  In pursuit of their lofty goals, they adhere to ironclad laws of morality and discipline. As reward for their righteousness, these holy champions are blessed with boons to aid them in their quests: powers to banish evil, heal the innocent, and inspire the faithful. Although their convictions might lead them into conflict with the very souls they would save, paladins weather endless challenges of faith and dark temptations, risking their lives to do right and fighting to bring about a brighter future.",
        model::Class::Druid => "Within the purity of the elements and the order of the wilds lingers a power beyond the marvels of civilization.  Furtive yet undeniable, these primal magics are guarded over by servants of philosophical balance known as druids.  Allies to beasts and manipulators of nature, these often misunderstood protectors of the wild strive to shield their lands from all who would threaten them and prove the might of the wilds to those who lock themselves behind city walls. Rewarded for their devotion with incredible powers, druids gain unparalleled shape-shifting abilities, the companionship of mighty beasts, and the power to call upon nature's wrath. The mightiest temper powers akin to storms, earthquakes, and volcanoes with primeval wisdom long abandoned and forgotten by civilization.",
        model::Class::Bard => "Untold wonders and secrets exist for those skillful enough to discover them. Through cleverness, talent, and magic, these cunning few unravel the wiles of the world, becoming adept in the arts of persuasion, manipulation, and inspiration. Typically masters of one or many forms of artistry, bards possess an uncanny ability to know more than they should and use what they learn to keep themselves and their allies ever one step ahead of danger.  Bards are quick-witted and captivating, and their skills might lead them down many paths, be they gamblers or jacks-of-all-trades, scholars or performers, leaders or scoundrels, or even all of the above. For bards, every day brings its own opportunities, adventures, and challenges, and only by bucking the odds, knowing the most, and being the best might they claim the treasures of each.",
        model::Class::Adventurer => "",
        model::Class::Monk => "For the truly exemplary, martial skill transcends the battlefield-it is a lifestyle, a doctrine, a state of mind.  These warrior-artists search out methods of battle beyond swords and shields, finding weapons within themselves just as capable of crippling or killing as any blade. These monks (so called since they adhere to ancient philosophies and strict martial disciplines) elevate their bodies to become weapons of war, from battle-minded ascetics to self-taught brawlers. Monks tread the path of discipline, and those with the will to endure that path discover within themselves not what they are, but what they are meant to be.",
        model::Class::Barbarian => "Barbarians excel in combat, possessing the martial prowess and fortitude to take on foes seemingly far superior to themselves. With rage granting them boldness and daring beyond that of most other warriors, barbarians charge furiously into battle and ruin all who would stand in their way.",
    }
}

pub fn restriction_info(class: &model::Class) -> &'static str {
    match class {
        model::Class::Druid => "Can only use the following weapons: Club, Dagger, Dart, Quarterstaff, Scimitar, Scythe, Sickle, Shortspear, Sling, Spear. For armor, shields and Misc. items: Cannot wear anything consisting of a lot of metal. Also can not use any large shields",
        model::Class::Barbarian => "Cannot use heavy armor",
        _ => "No restrictions",
    }
}

pub fn health_bonus(class: &model::Class) -> u8 {
    match class {
        model::Class::Fighter => 10,
        model::Class::Wizard => 6,
        model::Class::Cleric => 8,
        model::Class::Rogue => 8,
        model::Class::Ranger => 10,
        model::Class::Paladin => 10,
        model::Class::Druid => 8,
        model::Class::Bard => 8,
        model::Class::Adventurer => 10,
        model::Class::Monk => 8,
        model::Class::Barbarian => 12,
    }
}

pub fn melee_bonus(class: &model::Class) -> i8 {
    match class {
        model::Class::Fighter => 10,
        model::Class::Wizard => 4,
        model::Class::Cleric => 6,
        model::Class::Rogue => 6,
        model::Class::Ranger => 6,
        model::Class::Paladin => 8,
        model::Class::Druid => 4,
        model::Class::Bard => 5,
        model::Class::Adventurer => 6,
        model::Class::Monk => 8,
        model::Class::Barbarian => 10,
    }
}

pub fn ranged_bonus(class: &model::Class) -> i8 {
    match class {
        model::Class::Fighter => 10,
        model::Class::Wizard => 4,
        model::Class::Cleric => 5,
        model::Class::Rogue => 10,
        model::Class::Ranger => 10,
        model::Class::Paladin => 6,
        model::Class::Druid => 7,
        model::Class::Bard => 6,
        model::Class::Adventurer => 6,
        model::Class::Monk => 6,
        model::Class::Barbarian => 10,
    }
}

pub fn search_mod(class: &model::Class) -> i8 {
    match class {
        model::Class::Fighter => 14,
        model::Class::Wizard => 16,
        model::Class::Cleric => 16,
        model::Class::Rogue => 32,
        model::Class::Ranger => 24,
        model::Class::Paladin => 12,
        model::Class::Druid => 16,
        model::Class::Bard => 22,
        model::Class::Adventurer => 24,
        model::Class::Monk => 24,
        model::Class::Barbarian => 14,
    }
}

pub fn disarm_mod(class: &model::Class) -> i8 {
    match class {
        model::Class::Fighter => 25,
        model::Class::Wizard => 30,
        model::Class::Cleric => 25,
        model::Class::Rogue => 45,
        model::Class::Ranger => 30,
        model::Class::Paladin => 20,
        model::Class::Druid => 25,
        model::Class::Bard => 30,
        model::Class::Adventurer => 30,
        model::Class::Monk => 45,
        model::Class::Barbarian => 25,
    }
}

pub fn stealth_mod(class: &model::Class) -> i8 {
    match class {
        model::Class::Fighter => 1,
        model::Class::Wizard => 2,
        model::Class::Cleric => 2,
        model::Class::Rogue => 4,
        model::Class::Ranger => 3,
        model::Class::Paladin => 1,
        model::Class::Druid => 1,
        model::Class::Bard => 2,
        model::Class::Adventurer => 3,
        model::Class::Monk => 3,
        model::Class::Barbarian => 1,
    }
}

pub fn search_freq(class: &model::Class) -> i8 {
    match class {
        model::Class::Fighter => 38,
        model::Class::Wizard => 36,
        model::Class::Cleric => 32,
        model::Class::Rogue => 16,
        model::Class::Ranger => 24,
        model::Class::Paladin => 38,
        model::Class::Druid => 32,
        model::Class::Bard => 28,
        model::Class::Adventurer => 24,
        model::Class::Monk => 24,
        model::Class::Barbarian => 38,
    }
}

pub fn save_mod(class: &model::Class) -> i8 {
    match class {
        model::Class::Fighter => 10,
        model::Class::Wizard => 25,
        model::Class::Cleric => 20,
        model::Class::Rogue => 15,
        model::Class::Ranger => 20,
        model::Class::Paladin => 15,
        model::Class::Druid => 20,
        model::Class::Bard => 20,
        model::Class::Adventurer => 20,
        model::Class::Monk => 25,
        model::Class::Barbarian => 10,
    }
}

pub fn magic_resist(class: &model::Class) -> i8 {
    match class {
        model::Class::Fighter => -10,
        model::Class::Wizard => 0,
        model::Class::Cleric => 0,
        model::Class::Rogue => -5,
        model::Class::Ranger => -5,
        model::Class::Paladin => -5,
        model::Class::Druid => -5,
        model::Class::Bard => -5,
        model::Class::Adventurer => -5,
        model::Class::Monk => -5,
        model::Class::Barbarian => -10,
    }
}

pub fn expfactor(class: &model::Class) -> f32 {
    match class {
        model::Class::Fighter => 0.0,
        model::Class::Wizard => 0.3,
        model::Class::Cleric => 0.2,
        model::Class::Rogue => 0.1,
        model::Class::Ranger => 0.3,
        model::Class::Paladin => 0.35,
        model::Class::Druid => 0.2,
        model::Class::Bard => 0.3,
        model::Class::Adventurer => 0.4,
        model::Class::Monk => 0.1,
        model::Class::Barbarian => 0.0,
    }
}

pub fn abilities(class: &model::Class) -> Vec<model::Ability> {
    match class {
        model::Class::Barbarian => [model::Ability::Rage].to_vec(),
        _ => Vec::new(),
    }
}

pub fn magic_type(class: &model::Class) -> Option<model::Magic> {
    match class {
        model::Class::Wizard | model::Class::Adventurer => Some(model::Magic::Arcane),
        model::Class::Cleric | model::Class::Paladin => Some(model::Magic::Divine),
        model::Class::Druid | model::Class::Ranger => Some(model::Magic::Nature),
        model::Class::Bard | model::Class::Rogue => Some(model::Magic::Song),
        model::Class::Monk => Some(model::Magic::Chakra),
        model::Class::Fighter => None,
        model::Class::Barbarian => None,
    }
}

pub fn starting_items(class: &model::Class) -> Vec<model::Item> {
    let item_level = 10;
    let mut items = match class {
        model::Class::Fighter => vec![
            generate_item::generate_main_armor(item_level, ItemQuality::Normal),
            generate_item::generate_melee_weapon(item_level, ItemQuality::Normal)
        ],
        model::Class::Wizard => vec![
            generate_item::generate(Box::new(MagicBookTemplate::BeginnersMagic), item_level, ItemQuality::Normal),
            generate_item::generate(Box::new(ArmorTemplate::Robe), item_level, ItemQuality::Normal),
            generate_item::generate(Box::new(DaggerTemplate::Stiletto), item_level, ItemQuality::Normal),
        ],
        model::Class::Cleric => vec![
            generate_item::generate(Box::new(PrayerBookTemplate::BeginnersHandbook), item_level, ItemQuality::Normal),
            generate_item::generate(Box::new(ArmorTemplate::WovenCordArmor), item_level, ItemQuality::Normal),
            generate_item::generate(Box::new(MaceTemplate::IronShodQuarterstaff), item_level, ItemQuality::Normal),
        ],
        model::Class::Rogue => vec![
            generate_item::generate(Box::new(SongBookTemplate::BeginnersHandbook), item_level, ItemQuality::Normal),
            generate_item::generate_main_armor(item_level, ItemQuality::Normal),
            generate_item::generate(Box::new(DaggerTemplate::Stiletto), item_level, ItemQuality::Normal),
        ],
        model::Class::Ranger => vec![
            generate_item::generate(Box::new(InstrumentTemplate::PipesOfPeace), item_level, ItemQuality::Normal),
            generate_item::generate_main_armor(item_level, ItemQuality::Normal),
            generate_item::generate_melee_weapon(item_level, ItemQuality::Normal),
        ],
        model::Class::Paladin => vec![
            generate_item::generate(Box::new(PrayerBookTemplate::BeginnersHandbook), item_level, ItemQuality::Normal),
            generate_item::generate_main_armor(item_level, ItemQuality::Normal),
            generate_item::generate_melee_weapon(item_level, ItemQuality::Normal),
        ],
        model::Class::Druid => vec![
            generate_item::generate(Box::new(InstrumentTemplate::PipesOfPeace), item_level, ItemQuality::Normal),
            generate_item::generate_main_armor(item_level, ItemQuality::Normal),
            generate_item::generate(Box::new(MaceTemplate::IronShodQuarterstaff), item_level, ItemQuality::Normal),
        ],
        model::Class::Bard => vec![
            generate_item::generate(Box::new(SongBookTemplate::BeginnersHandbook), item_level, ItemQuality::Normal),
            generate_item::generate_main_armor(item_level, ItemQuality::Normal),
            generate_item::generate_boots(item_level, ItemQuality::Normal),
            generate_item::generate(Box::new(DaggerTemplate::Stiletto), item_level, ItemQuality::Normal),
        ],
        model::Class::Adventurer => vec![
            generate_item::generate(Box::new(MagicBookTemplate::BeginnersMagic), item_level, ItemQuality::Normal),
            generate_item::generate_main_armor(item_level, ItemQuality::Normal),
            generate_item::generate_melee_weapon(item_level, ItemQuality::Normal),
        ],
        model::Class::Monk => vec![
            generate_item::generate(Box::new(ArmorTemplate::Robe), item_level, ItemQuality::Normal),
        ],
        model::Class::Barbarian => vec![
            generate_item::generate_main_armor(item_level, ItemQuality::Normal),
            generate_item::generate_melee_weapon(item_level, ItemQuality::Normal),
        ],
    };
    items.iter_mut().for_each(|item| {item.set_identified(true)});
    items
}
