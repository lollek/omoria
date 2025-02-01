use crate::data::item_name::generate;
use crate::generate_item;
use crate::generate_item::template::WandTemplate;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wand_unidentified() {
        let mut item = generate_item::generate(Box::new(WandTemplate::WandOfCloneMonster), 0);

        item.set_identified(false);
        assert_eq!(generate(&item), "Unknown wand");
    }

    #[test]
    fn test_wand_identified() {
        let mut item = generate_item::generate(Box::new(WandTemplate::WandOfCloneMonster), 0);

        item.set_identified(true);
        item.p1 = 0;
        assert_eq!(generate(&item), "Wand of Clone Monster (0 charges)");
    }
}
