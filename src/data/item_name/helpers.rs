use std::borrow::Cow;
use crate::helper;
use crate::misc::item_name2rs;
use crate::model::Item;

pub(crate) fn maybe_armor_bonus<'a>(item: &Item) -> Option<Cow<'a, str>> {
    if item.ac == 0 && (!item.is_identified() || item.toac == 0) {
        return None
    }

    if !item.is_identified() {
        return Some(format!(" [{}]", item.ac).into());
    }

    // For armor, we show both AC and its (possibly signed) to-ac modifier.
    Some(format!(" [{},{}]", item.ac, helper::format_signed(item.toac)).into())
}

/**
 * Returns the number of the given item. 1 returns an empty string
 */
pub(crate) fn maybe_number_of<'a>(item: &Item) -> Option<Cow<'a, str>> {
    match item.number {
        0 => Some("no more ".into()),
        1 => None,
        _ => Some((item.number.to_string() + " ").into()),
    }
}

/**
 * Returns the number of the given item, including 1 if number is 1
 */
pub(crate) fn full_number_of<'a>(item: &Item) -> Cow<'a, str> {
    match item.number {
        0 => Cow::from("no more "),
        _ => Cow::from(item.number.to_string() + " "),
    }
}

/**
 * Returns if there are no items left, or empty string
 */
pub(crate) fn no_more<'a>(item: &Item) -> Cow<'a, str> {
    match item.number {
        0 => Cow::from("no more "),
        _ => Cow::from(""),
    }
}

pub(crate) fn p1_plural_s<'a>(item: &Item) -> Cow<'a, str> {
    if item.p1 == 1 {
        Cow::Borrowed("")
    } else {
        Cow::Borrowed("s")
    }
}

pub(crate) fn plural_s<'a>(item: &Item) -> Cow<'a, str> {
    if item.number == 1 {
        Cow::Borrowed("")
    } else {
        Cow::Borrowed("s")
    }
}

pub(crate) fn plural_es<'a>(item: &Item) -> Cow<'a, str> {
    if item.number == 1 {
        Cow::Borrowed("")
    } else {
        Cow::Borrowed("es")
    }
}

pub(crate) fn damage<'a>(item: &Item) -> Cow<'a, str> {
    Cow::from(format!(" ({})", item.damage_string()))
}

pub(crate) fn attack_bonus<'a>(item: &Item) -> Cow<'a, str> {
    Cow::from(format!(
        " ({},{})",
        helper::format_signed(item.tohit),
        helper::format_signed(item.todam),
    ))
}

pub fn to_hit_if_exists<'a>(item: &Item) -> Option<Cow<'a, str>> {
    match item.tohit {
        x if x > 0 => Some(format!(" (+{})", item.tohit).into()),
        x if x < 0 => Some(format!(" ({})", item.tohit).into()),
        _ => None,
    }
}

pub fn maybe_p1_bonus<'a>(item: &Item) -> Option<Cow<'a, str>> {
    if item.is_identified() && item.p1 != 0 {
        Some(p1_bonus(item))
    } else {
        None
    }
}

pub fn maybe_special_attribute(item: &'_ Item) -> Option<Cow<'_, str>> {
    if item.is_identified() {
        let item_name = item_name2rs(&item.name);
        let suffixes = [
            "R", "RA", "RF", "RC", "RL", "FT", "FB", "WB", "BB", "SM", "SD", "SU", "SR", "HA",
            "DF", "DB", "SS", "V"
        ];
        for suffix in suffixes.iter() {
            let formatted_suffix = format!("({})", suffix);
            if item_name.contains(&formatted_suffix) {
                return Some(format!(" {}", formatted_suffix).into());
            }
        }
    }
    None
}


pub(crate) fn p1_bonus<'a>(item: &Item) -> Cow<'a, str> {
    Cow::from(format!(" ({})", helper::format_signed(item.p1)))
}

pub(crate) fn toac_bonus<'a>(item: &Item) -> Cow<'a, str> {
    Cow::from(format!(" [{}]", helper::format_signed(item.toac)))
}