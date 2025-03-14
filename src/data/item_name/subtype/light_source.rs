use crate::conversion::item_subtype::from_i64;
use crate::data::item_name::helpers::{maybe_number_of, p1_plural_s};
use crate::model::item_subtype::{ItemSubType, LightSourceSubType};
use crate::model::{Item, ItemType};
use std::borrow::Cow;

pub fn light_source(item: &Item) -> String {
    let mut parts = vec![];
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }
    parts.push(match from_i64(ItemType::LightSource, item.subval) {
        Some(subtype) => match subtype {
            ItemSubType::LightSource(LightSourceSubType::WoodenTorch) => Cow::from("wooden torch"),
            ItemSubType::LightSource(LightSourceSubType::BrassLantern) => {
                Cow::from("brass lantern")
            }
            ItemSubType::LightSource(LightSourceSubType::MagicTorch) => Cow::from("magic torch"),
            ItemSubType::LightSource(LightSourceSubType::MagicLantern) => {
                Cow::from("magic lantern")
            }
            t => panic!("Expected light source, got {:?}", t),
        },
        None => Cow::from("alien lightsource"),
    });
    parts.push(Cow::from(format!(
        " with {} turn{} of light",
        item.p1,
        p1_plural_s(item)
    )));
    parts.join("")
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::template::LightSourceTemplate;

    #[test]
    fn test_light_source_wooden_torch() {
        let mut item = generate_item::generate(Box::new(LightSourceTemplate::WoodenTorch), 0);
        assert_eq!(generate(&item), "wooden torch with 4000 turns of light");

        item.p1 = 1;
        assert_eq!(generate(&item), "wooden torch with 1 turn of light");
    }

    #[test]
    fn test_light_source_brass_lantern() {
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(LightSourceTemplate::BrassLantern),
                0
            )),
            "brass lantern with 7500 turns of light"
        );
    }

    #[test]
    fn test_light_source_magic_torch() {
        let mut magic_torch = generate_item::generate(Box::new(LightSourceTemplate::MagicTorch), 0);
        magic_torch.set_identified(true);
        assert_eq!(
            generate(&magic_torch),
            "magic torch with 9000 turns of light"
        );

        magic_torch.set_identified(false);
        assert_eq!(
            generate(&magic_torch),
            "magic torch with 9000 turns of light"
        );
    }

    #[test]
    fn test_light_source_magic_lantern() {
        let mut magic_lantern =
            generate_item::generate(Box::new(LightSourceTemplate::MagicLantern), 0);
        magic_lantern.set_identified(true);
        assert_eq!(
            generate(&magic_lantern),
            "magic lantern with 20000 turns of light"
        );

        magic_lantern.set_identified(false);
        assert_eq!(
            generate(&magic_lantern),
            "magic lantern with 20000 turns of light"
        );
    }
}
