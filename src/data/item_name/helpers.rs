use std::borrow::Cow;

use crate::{misc, model::Item};

/**
 * Returns the number of the given item. 1 returns an empty string
 */
pub(crate) fn number_of<'a>(item: &Item) -> Cow<'a, str> {
    match item.number {
        0 => Cow::from("no more "),
        1 => Cow::from(""),
        _ => Cow::from(item.number.to_string() + " "),
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

#[cfg(test)]
mod test {

    use crate::generate_item::{self, template::FoodTemplate};

    use super::*;

    #[test]
    fn test_number_of() {
        let mut item = generate_item::generate(Box::new(FoodTemplate::RationOfFood), 0);

        item.number = 0;
        assert_eq!(number_of(&item).as_ref(), "no more ");

        item.number = 1;
        assert_eq!(number_of(&item).as_ref(), "");

        item.number = 2;
        assert_eq!(number_of(&item).as_ref(), "2 ");
    }
}
