use crate::conversion::item_subtype::from_i64;
use crate::data::item_name::helpers::no_more;
use crate::model::item_subtype::{ItemSubType, MiscUsableSubType};
use crate::model::MiscUsableFlag1::{
    ContainingDemons, ContainingDjinni, DemonDispelling, LifeGiving, MajorSummonDemon,
    MajorSummonUndead, SummonDemon, SummonUndead, Turning,
};
use crate::model::{Item, ItemType};
use std::borrow::Cow;

pub fn misc_usable(item: &Item) -> String {
    let mut parts = Vec::new();
    parts.push(no_more(item));
    if item.is_identified() {
        if item.has_misc_usable_flag(Turning) {
            parts.push(Cow::Borrowed("cross of turning"));
        } else if item.has_misc_usable_flag(DemonDispelling) {
            parts.push(Cow::Borrowed("cross of demon dispelling"));
        } else if item.has_misc_usable_flag(SummonUndead) {
            parts.push(Cow::Borrowed("cross of undead summoning"));
        } else if item.has_misc_usable_flag(SummonDemon) {
            parts.push(Cow::Borrowed("cross of demon summoning"));
        } else if item.has_misc_usable_flag(ContainingDjinni) {
            parts.push(Cow::Borrowed("corked bottle containing a djinni"));
        } else if item.has_misc_usable_flag(ContainingDemons) {
            parts.push(Cow::Borrowed("corked bottle containing some demons"));
        } else if item.has_misc_usable_flag(MajorSummonUndead) {
            parts.push(Cow::Borrowed("statue of undead summoning"));
        } else if item.has_misc_usable_flag(MajorSummonDemon) {
            parts.push(Cow::Borrowed("statue of demon summoning"));
        } else if item.has_misc_usable_flag(LifeGiving) {
            parts.push(Cow::Borrowed("statue of life"));
        } else {
            parts.push(subtype_name(item));
        }
    } else {
        parts.push(subtype_name(item));
    }
    parts.join("")
}

fn subtype_name(item: &Item) -> Cow<str> {
    Cow::Borrowed(match from_i64(ItemType::MiscUsable, item.subval) {
        Some(subtype) => match subtype {
            ItemSubType::MiscUsable(MiscUsableSubType::Statue) => "statue",
            ItemSubType::MiscUsable(MiscUsableSubType::SilverCross) => "silver cross",
            ItemSubType::MiscUsable(MiscUsableSubType::GoldCross) => "gold cross",
            ItemSubType::MiscUsable(MiscUsableSubType::MithrilCross) => "mithril cross",
            ItemSubType::MiscUsable(MiscUsableSubType::Cross) => "cross",
            ItemSubType::MiscUsable(MiscUsableSubType::CorkedBottle) => "corked bottle",
            t => panic!("Expected misc usable, got {:?}", t),
        },
        None => "alien usable item",
    })
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item::template::MiscUsableTemplate;
    use crate::generate_item::{ItemQuality, ItemTemplate};
    use crate::{generate_item, identification};

    #[test]
    fn test_statue_of_summon_undead() {
        let template = MiscUsableTemplate::Statue;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        MiscUsableTemplate::apply_statue_of_summon_undead(&template, &mut item);

        item.set_identified(false);
        identification::set_identified(template.subtype(), false);
        assert_eq!(generate(&item), "statue");

        identification::set_identified(template.subtype(), true);
        assert_eq!(generate(&item), "statue");

        item.set_identified(true);
        assert_eq!(generate(&item), "statue of undead summoning");
    }

    #[test]
    fn test_statue_of_summon_demon() {
        let template = MiscUsableTemplate::Statue;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        MiscUsableTemplate::apply_statue_of_summon_demon(&template, &mut item);

        item.set_identified(false);
        identification::set_identified(template.subtype(), false);
        assert_eq!(generate(&item), "statue");

        identification::set_identified(template.subtype(), true);
        assert_eq!(generate(&item), "statue");

        item.set_identified(true);
        assert_eq!(generate(&item), "statue of demon summoning");
    }

    #[test]
    fn test_statue_of_give_life() {
        let template = MiscUsableTemplate::Statue;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        MiscUsableTemplate::apply_statue_of_give_life(&template, &mut item);

        item.set_identified(false);
        identification::set_identified(template.subtype(), false);
        assert_eq!(generate(&item), "statue");

        identification::set_identified(template.subtype(), true);
        assert_eq!(generate(&item), "statue");

        item.set_identified(true);
        assert_eq!(generate(&item), "statue of life");
    }

    #[test]
    fn test_cross_of_turning() {
        let template = MiscUsableTemplate::Cross;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        MiscUsableTemplate::apply_cross_of_turning(&template, &mut item);

        item.set_identified(false);
        identification::set_identified(template.subtype(), false);
        assert_eq!(generate(&item), "cross");

        identification::set_identified(template.subtype(), true);
        assert_eq!(generate(&item), "cross");

        item.set_identified(true);
        assert_eq!(generate(&item), "cross of turning");
    }

    #[test]
    fn test_cross_of_demon_dispelling() {
        let template = MiscUsableTemplate::Cross;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        MiscUsableTemplate::apply_cross_of_demon_dispelling(&template, &mut item);

        item.set_identified(false);
        identification::set_identified(template.subtype(), false);
        assert_eq!(generate(&item), "cross");

        identification::set_identified(template.subtype(), true);
        assert_eq!(generate(&item), "cross");

        item.set_identified(true);
        assert_eq!(generate(&item), "cross of demon dispelling");
    }

    #[test]
    fn test_cross_of_undead_summoning() {
        let template = MiscUsableTemplate::Cross;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        MiscUsableTemplate::apply_cross_of_summon_undead(&template, &mut item);

        item.set_identified(false);
        identification::set_identified(template.subtype(), false);
        assert_eq!(generate(&item), "cross");

        identification::set_identified(template.subtype(), true);
        assert_eq!(generate(&item), "cross");

        item.set_identified(true);
        assert_eq!(generate(&item), "cross of undead summoning");
    }

    #[test]
    fn test_cross_of_demon_summoning() {
        let template = MiscUsableTemplate::Cross;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        MiscUsableTemplate::apply_cross_of_summon_demon(&template, &mut item);

        item.set_identified(false);
        identification::set_identified(template.subtype(), false);
        assert_eq!(generate(&item), "cross");

        identification::set_identified(template.subtype(), true);
        assert_eq!(generate(&item), "cross");

        item.set_identified(true);
        assert_eq!(generate(&item), "cross of demon summoning");
    }

    #[test]
    fn test_bottle_of_djinni() {
        let template = MiscUsableTemplate::CorkedBottle;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        MiscUsableTemplate::apply_bottle_of_djinni(&template, &mut item);

        item.set_identified(false);
        identification::set_identified(template.subtype(), false);
        assert_eq!(generate(&item), "corked bottle");

        identification::set_identified(template.subtype(), true);
        assert_eq!(generate(&item), "corked bottle");

        item.set_identified(true);
        assert_eq!(generate(&item), "corked bottle containing a djinni");
    }

    #[test]
    fn test_bottle_of_demons() {
        let template = MiscUsableTemplate::CorkedBottle;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        MiscUsableTemplate::apply_bottle_of_demons(&template, &mut item);

        item.set_identified(false);
        identification::set_identified(template.subtype(), false);
        assert_eq!(generate(&item), "corked bottle");

        identification::set_identified(template.subtype(), true);
        assert_eq!(generate(&item), "corked bottle");

        item.set_identified(true);
        assert_eq!(generate(&item), "corked bottle containing some demons");
    }

    #[test]
    fn test_misc_usable_statue() {
        let mut item =
            generate_item::generate(Box::new(MiscUsableTemplate::Statue), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "statue");

        item.number = 0;
        assert_eq!(generate(&item), "no more statue");
    }

    #[test]
    fn test_misc_usable_silver_cross() {
        let template = MiscUsableTemplate::SilverCross;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "silver cross");

        item.number = 0;
        assert_eq!(generate(&item), "no more silver cross");
    }

    #[test]
    fn test_misc_usable_gold_cross() {
        let template = MiscUsableTemplate::GoldCross;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "gold cross");

        item.number = 0;
        assert_eq!(generate(&item), "no more gold cross");
    }

    #[test]
    fn test_misc_usable_mithril_cross() {
        let template = MiscUsableTemplate::MithrilCross;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "mithril cross");

        item.number = 0;
        assert_eq!(generate(&item), "no more mithril cross");
    }

    #[test]
    fn test_misc_usable_cross() {
        let template = MiscUsableTemplate::Cross;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "cross");

        item.number = 0;
        assert_eq!(generate(&item), "no more cross");
    }

    #[test]
    fn test_misc_usable_corked_bottle() {
        let template = MiscUsableTemplate::CorkedBottle;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "corked bottle");

        item.number = 0;
        assert_eq!(generate(&item), "no more corked bottle");
    }
}
