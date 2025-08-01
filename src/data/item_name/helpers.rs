use std::borrow::Cow;

use crate::{misc, model::Item};

pub(crate) fn maybe_armor_bonus<'a>(item: &Item) -> Option<Cow<'a, str>> {
    if item.ac == 0 && (!item.is_identified() || item.toac == 0) {
        return None
    }

    if !item.is_identified() {
        return Some(format!(" [{}]", item.ac).into());
    }

    let toac_sign = if item.toac > 0 { "+" } else { "" };
    Some(format!(" [{},{}{}]", item.ac, toac_sign, item.toac).into())
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
    let raw_string = item.damage.iter().map(|&i| i as u8).collect::<Vec<u8>>();
    let damage_string = misc::c_array_to_rust_string(raw_string);
    Cow::from(format!(" ({})", damage_string))
}

pub(crate) fn attack_bonus<'a>(item: &Item) -> Cow<'a, str> {
    let tohit_sign = if item.tohit > 0 { "+" } else { "" };
    let todam_sign = if item.todam > 0 { "+" } else { "" };
    Cow::from(format!(
        " ({}{},{}{})",
        tohit_sign, item.tohit, todam_sign, item.todam
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

pub(crate) fn p1_bonus<'a>(item: &Item) -> Cow<'a, str> {
    let p1_sign = if item.p1 > 0 { "+" } else { "" };
    Cow::from(format!(" ({}{})", p1_sign, item.p1))
}