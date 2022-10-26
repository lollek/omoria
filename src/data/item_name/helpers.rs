use std::borrow::Cow;

use crate::model::Item;

pub(crate) fn number_of<'a>(item: &Item) -> Cow<'a, str> {
    match item.number {
        0 => Cow::from("no more "),
        1 => Cow::from(""),
        _ => Cow::from(item.number.to_string() + " "),
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
