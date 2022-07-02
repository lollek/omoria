use std::borrow::Cow;

use model;
use misc;

pub fn plural_s<'a>(item: &model::Item) -> Cow<'a, str> {
    Cow::from(if item.number == 1 {
        ""
    } else {
        "s"
    })
}

pub fn plural_es<'a>(item: &model::Item) -> Cow<'a, str> {
    Cow::from(if item.number == 1 {
        ""
    } else {
        "es"
    })
}

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
