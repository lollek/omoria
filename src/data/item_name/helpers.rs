use std::borrow::Cow;

use crate::model::Item;

pub(crate) fn number_of<'a>(item: &Item) -> Cow<'a, str> {
    match item.number {
        0 => Cow::from("no more "),
        1 => Cow::from(""),
        _ => Cow::from(item.number.to_string() + " "),
    }
}

pub(crate) fn no_more<'a>(item: &Item) -> Cow<'a, str> {
    match item.number {
        0 => Cow::from("no more "),
        _ => Cow::from(""),
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
