use std::borrow::Cow;

use item_template;
use misc;
use model;

pub fn armor<'a>(item: &model::Item) -> Cow<'a, str> {
    if item.ac == 0 && (!item.is_identified() || item.toac == 0) {
        return Cow::from("");
    }

    if !item.is_identified() {
        return Cow::from(format!(" [{}]", item.ac));
    }

    let toac_sign = if item.toac > 0 { "+" } else {""};
    return Cow::from(format!(" [{},{}{}]", item.ac, toac_sign, item.toac))
}

pub fn attack_enchantment<'a>(item: &model::Item) -> Cow<'a, str> {
    if !item.is_identified() {
        return Cow::from("");
    }

    let tohit_sign = if item.tohit > 0 { "+" } else {""};
    let todam_sign = if item.todam > 0 { "+" } else {""};
    Cow::from(format!(" ({}{},{}{})", tohit_sign, item.tohit, todam_sign, item.todam))
}

pub fn damage<'a>(item: &model::Item) -> Cow<'a, str> {
    let raw_string = item.damage.iter().map(|&i| i as u8).collect::<Vec<u8>>();
    let damage_string = misc::c_array_to_rust_string(raw_string);
    Cow::from(format!(" ({})", damage_string))
}

pub fn number_of<'a>(item: &model::Item) -> Cow<'a, str> {
    match item.number {
        0 => Cow::from("no more "),
        1 => Cow::from(""),
        _ => Cow::from(item.number.to_string() + " "),
    }
}

pub fn charges<'a>(item: &model::Item) -> Cow<'a, str> {
    if !item.is_identified() {
        return Cow::from("");
    }
    return Cow::from(format!(" ({} charges)", item.p1))
}

pub fn p1<'a>(item: &model::Item) -> Cow<'a, str> {
    if !item.is_identified() {
        return Cow::from("");
    }
    return Cow::from(format!(" ({})", item.p1))
}

pub fn to_hit<'a>(item: &model::Item) -> Cow<'a, str> {
    if !item.is_identified() {
        return Cow::from("");
    }
    return Cow::from(format!(" ({})", item.tohit))
}

pub fn to_damage<'a>(item: &model::Item) -> Cow<'a, str> {
    if !item.is_identified() {
        return Cow::from("");
    }
    return Cow::from(format!(" ({})", item.todam))
}

pub fn to_ac<'a>(item: &model::Item) -> Cow<'a, str> {
    if !item.is_identified() {
        return Cow::from("");
    }
    return Cow::from(format!(" ({})", item.toac))
}

pub fn generate_book_name<'a>(item: &model::Item, name: Cow<'a, str>) -> String {
    let mut parts = Vec::new();
    parts.push(number_of(item));
    parts.push(name);
    parts.join("")
}

pub fn generate_weapon_name<'a>(item: &model::Item, name: Cow<'a, str>) -> String {
    let mut parts = Vec::new();
    parts.push(name);
    parts.push(damage(&item));
    parts.push(attack_enchantment(&item));
    parts.push(armor(&item));
    if item.flags & 0x01598001 {
        parts.push(Cow::from(" (HA)"));
    } else if item.flags & 0x07B80900 {
        parts.push(Cow::from(" (DF)"));
    } else if item.flags & 0x00080000 && item.flags2 & 0x00000001 {
        parts.push(Cow::from(" (DB)"));
    } else if item.flags & 0x01000838 && item.flags2 & 0x00200002 {
        parts.push(Cow::from(" (SS)"));
    } else if item.flags & 0x00400000 && item.flags2 & 0x00000040 {
        parts.push(Cow::from(" (V)"));
    } else if item.flags & 0x00300960 {
        parts.push(Cow::from(" of Trollkind"));
    } else if item.flags & 0x01004000 {
        parts.push(Cow::from(" (SM)"));
    } else if item.flags & 0x00002000 {
        parts.push(Cow::from(" (SD)"));
    } else if item.flags & 0x00004000 {
        parts.push(Cow::from(" of Slay Monster"));
    } else if item.flags & 0x00008000 {
        parts.push(Cow::from(" of Slay Evil"));
    } else if item.flags & 0x00010000 {
        parts.push(Cow::from(" (SU)"));
    } else if item.flags & 0x00020000 {
        parts.push(Cow::from(" (FB)"));
    } else if item.flags & 0x00040000 {
        parts.push(Cow::from(" (FT)"));
    } else if item.flags2 & 0x00000010 {
        if item.item_type() == ItemType::Dagger {
            parts.push(Cow::from(" (WB)"));
        } else if item.item_type() == ItemType::Mace {
            parts.push(Cow::from(" (BB)"));
        } else {
            parts.push(Cow::from(" (Magic)"));
        }
    } else if item.flags2 & 0x00000004 {
        parts.push(Cow::from(" (SR)"));
    } else if item.flags2 & 0x00000040 {
        parts.push(Cow::from(" of Criticals"));
    } else if item.tohit > 4 && item.todam > 4 {
        parts.push(Cow::from(" of Slaying"));
    }
    parts.join("")
}

pub fn generate_armor_name<'a>(item: &model::Item, name: Cow<'a, str>) -> String {
    let mut parts = Vec::new();
    parts.push(name);
    if item.tohit != 0 || item.todam != 0 {
        parts.push(attack_enchantment(&item));
    }
    parts.push(armor(&item));
    if item.flags & 0x02380000 {
        parts.push(Cow::from(" (R)"));
    } else if item.flags & 0x00800007 {
        parts.push(Cow::from(" of Might"));
    } else if item.flags & 0x00800007 {
        parts.push(Cow::from(" of the Magi"));
    } else if item.flags & 0x00000030 {
        parts.push(Cow::from(" of Lordliness"));
    } else if item.flags & 0x41800040 {
        parts.push(Cow::from(" of Hobbitkind"));
    } else if item.flags & 0x01400120 {
        parts.push(Cow::from(" of Elvenkin"));
    } else if item.flags & 0x60400000 & item.flags2 & 0x00000010 {
        parts.push(Cow::from(" of Dwarvenkind"));
    } else if item.flags & 0x05800020 & item.flags2 & 0x00000010 {
        parts.push(Cow::from(" of Dryadkind"));
    } else if item.flags & 0x03B80800 & item.flags2 & 0x00000010 {
        parts.push(Cow::from(" of Titan Strength"));
    } else if item.flags & 0x03100000 & item.flags2 & 0x00000010 {
        parts.push(Cow::from(" of Storm Giant Strength"));
    } else if item.flags & 0x03100000  {
        parts.push(Cow::from(" of Cloud Giant Strength"));
    } else if item.flags & 0x01080000  {
        parts.push(Cow::from(" of Fire Giant Strength"));
    } else if item.flags & 0x01200000  {
        parts.push(Cow::from(" of Frost Giant Strength"));
    } else if item.flags & 0x01000000 && item.p1 > 1  {
        parts.push(Cow::from(" of Stone Giant Strength"));
    } else if item.flags & 0x01000000 && item.p1 == 1  {
        parts.push(Cow::from(" of Hill Giant Strength"));
    } else if item.flags & 0x00000081 {
        parts.push(Cow::from(" of Ogre Power"));
    } else if item.flags & 0x01000040 {
        parts.push(Cow::from(" of Seeing"));
    } else if item.flags & 0x9800073F && item.flags2 & 0x80400000 {
        parts.push(Cow::from(" of **TOTAL DOOM**"));
    } else if item.flags & 0x80000001 {
        parts.push(Cow::from(" of Weakness"));
    } else if item.flags & 0x80000002 {
        parts.push(Cow::from(" of Clumsiness"));
    } else if item.flags & 0x80000008 {
        parts.push(Cow::from(" of Stupidity"));
    } else if item.flags & 0x80000010 {
        parts.push(Cow::from(" of Dullness"));
    } else if item.flags & 0x80000020 {
        parts.push(Cow::from(" of Ugliness"));
    } else if item.flags & 0x80000200 {
        parts.push(Cow::from(" of Noise"));
    } else if item.flags & 0x80000400 {
        parts.push(Cow::from(" of Teleportation"));
    } else if item.flags & 0x80001000 {
        parts.push(Cow::from(" of Slowness"));
    } else if item.flags & 0x80400000 {
        parts.push(Cow::from(" of Hunger"));
    } else if item.flags & 0x88000000 {
        parts.push(Cow::from(" of Blindness"));
    } else if item.flags & 0x90000000 {
        parts.push(Cow::from(" of Fear"));
    } else if item.flags & 0x80000000 {
        parts.push(Cow::from(" of Great Mass"));
    } else if item.flags & 0x05000000 && item.flags2 & 0x00000020 {
        parts.push(Cow::from(" of Thievery"));
    } else if item.flags2 & 0x00000010 && item.toac > 0 {
        parts.push(Cow::from(" of Deflection"));
    } else if item.flags & 0x00400080 {
        parts.push(Cow::from(" of Improved Digestion"));
    } else if item.flags & 0x00100000 {
        parts.push(Cow::from(" (RA)"));
    } else if item.flags & 0x00080000 {
        parts.push(Cow::from(" (RF)"));
    } else if item.flags & 0x00200000 {
        parts.push(Cow::from(" (RC)"));
    } else if item.flags & 0x02000000 {
        parts.push(Cow::from(" (RL)"));
    } else if item.flags & 0x00000002 {
        parts.push(Cow::from(" of the Hive"));
    } else if item.flags & 0x00001000 {
        parts.push(Cow::from(" of Speed"));
    } else if item.flags & 0x00000100 {
        parts.push(Cow::from(" of Stealth"));
    } else if item.flags & 0x00800000 {
        parts.push(Cow::from(" of Free Action"));
    } else if item.flags & 0x04000000 {
        parts.push(Cow::from(" of Slow Descent"));
    } else if item.tohit != 0 || item.todam != 0 {
        parts.push(Cow::from(" of Slaying"));
    } else if item.flags & 0x00000008 {
        parts.push(Cow::from(" of Intelligence"));
    } else if item.flags & 0x00000010 {
        parts.push(Cow::from(" of Wisdom"));
    } else if item.flags & 0x00000020 {
        parts.push(Cow::from(" of Beauty"));
    } else if item.flags & 0x40000000 {
        parts.push(Cow::from(" of Infravision"));
    } else if item.flags & 0x00000800 {
        parts.push(Cow::from(" of Regeneration"));
    }
    parts.join("")
}

pub fn generate_name(item: &model::Item) -> String {
    <dyn item_template::ItemTemplate>::from(item.item_type(), item.subval).name(&item),
}
