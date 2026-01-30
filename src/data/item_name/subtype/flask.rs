use crate::data::item_name::helpers::{maybe_number_of, plural_s};
use crate::model::Item;

pub fn flask(item: &Item) -> String {
    let mut parts = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }
    parts.push(format!("flask{} of oil", plural_s(item)).into());
    parts.join("")
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::template::MiscUsableTemplate;
    use crate::generate_item::ItemQuality;

    #[test]
    fn test_flask_of_oil_single() {
        let mut item =
            generate_item::generate(Box::new(MiscUsableTemplate::FlaskOfOil), 0, ItemQuality::Normal);
        item.number = 1;
        assert_eq!(generate(&item), "flask of oil");
    }

    #[test]
    fn test_flask_of_oil_multiple() {
        let mut item =
            generate_item::generate(Box::new(MiscUsableTemplate::FlaskOfOil), 0, ItemQuality::Normal);
        item.number = 2;
        assert_eq!(generate(&item), "2 flasks of oil");
    }

    #[test]
    fn test_flask_of_oil_none() {
        let mut item =
            generate_item::generate(Box::new(MiscUsableTemplate::FlaskOfOil), 0, ItemQuality::Normal);
        item.number = 0;
        assert_eq!(generate(&item), "no more flasks of oil");
    }
}
