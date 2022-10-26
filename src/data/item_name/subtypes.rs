use std::borrow::Cow;
use std::convert::TryInto;

use crate::conversion::item_subtype::light_source::from_usize;
use crate::model::item_subtype::LightSourceSubType;
use crate::model::Item;

use super::helpers::number_of;

pub(crate) fn light_source(item: &Item) -> String {
    let plural_s = if item.p1 == 0 { "" } else { "s" };
    let subval: _ = item
        .subval
        .try_into()
        .unwrap_or_else(|_| panic!("Failed to parse {} to usize", item.subval));

    vec![
        number_of(item),
        match from_usize(subval) {
            Some(subtype) => match subtype {
                LightSourceSubType::WoodenTorch => Cow::from("wooden torch"),
                LightSourceSubType::BrassLantern => Cow::from("brass lantern"),
                LightSourceSubType::MagicTorch => Cow::from("magic torch"),
                LightSourceSubType::MagicLantern => Cow::from("magic lantern"),
            },
            None => Cow::from("alien lightsource"),
        },
        Cow::from(format!(" with {} turn{} of light", item.p1, plural_s)),
    ]
    .join("")
}
