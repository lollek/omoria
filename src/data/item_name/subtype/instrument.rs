use crate::conversion::item_subtype;
use crate::data::item_name::helpers::maybe_number_of;
use crate::model::item_subtype::{InstrumentSubType, ItemSubType};
use crate::model::Item;

pub fn instrument(item: &Item) -> String {
    let Some(subtype) = item_subtype::from_i64(item.item_type(), item.subval) else {
        return "alien instrument".to_string();
    };

    let ItemSubType::Instrument(t) = subtype else {
        return "alien instrument".to_string();
    };

    let mut parts = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }
    parts.push(subtype_name(t).into());
    parts.join("")
}

fn subtype_name(t: InstrumentSubType) -> &'static str {
    match t {
        InstrumentSubType::PipesOfPeace => "pipes of peace",
        InstrumentSubType::LyreOfNature => "lyre of nature",
        InstrumentSubType::LuteOfTheWoods => "lute of the woods",
        InstrumentSubType::HarpOfTheDruids => "harp of the druids",
    }
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::template::InstrumentTemplate;
    use crate::generate_item::{ItemQuality, ItemTemplate};
    use crate::identification;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_instrument_single() {
        let template = InstrumentTemplate::PipesOfPeace;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        item.number = 1;
        assert_eq!(generate(&item), "pipes of peace");
    }

    #[test]
    #[serial]
    fn test_instrument_multiple_prefix() {
        let template = InstrumentTemplate::PipesOfPeace;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        item.number = 2;
        // No plural marker exists in the template name, so quantity should not add an "s".
        assert_eq!(generate(&item), "2 pipes of peace");
    }

    #[test]
    #[serial]
    fn test_instrument_none_prefix() {
        let template = InstrumentTemplate::PipesOfPeace;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        item.number = 0;
        assert_eq!(generate(&item), "no more pipes of peace");
    }

    #[test]
    #[serial]
    fn test_instrument_known_subtype_has_no_effect_without_item_identification() {
        let template = InstrumentTemplate::PipesOfPeace;
        let subtype = template.subtype();
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        item.set_identified(false);
        item.number = 1;

        identification::set_identified(subtype, false);
        assert_eq!(generate(&item), "pipes of peace");

        identification::set_identified(subtype, true);
        assert_eq!(generate(&item), "pipes of peace");

        // Avoid leaking global state.
        identification::set_identified(subtype, false);
    }
}
