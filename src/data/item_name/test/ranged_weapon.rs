use crate::{
    data::item_name::generate,
    generate_item::{
        self,
        template::{BowTemplate, CrossbowTemplate, SlingTemplate},
    },
};

#[test]
fn test_shortbow() {
    let mut item = generate_item::generate(Box::new(BowTemplate::Shortbow), 0);
    item.tohit = 12;
    item.todam = 24;

    item.number = 0;
    assert_eq!(generate(&item), "no more shortbow (+2)");

    item.number = 1;
    assert_eq!(generate(&item), "shortbow (+2)");

    item.set_identified(true);
    assert_eq!(generate(&item), "shortbow (+2) (+12,+24)");

    item.tohit = 0;
    item.todam = 0;
    assert_eq!(generate(&item), "shortbow (+2) (0,0)");

    item.tohit = -1;
    item.todam = -2;
    assert_eq!(generate(&item), "shortbow (+2) (-1,-2)");
}

#[test]
fn test_hunters_bow() {
    let mut item = generate_item::generate(Box::new(BowTemplate::HuntersBow), 0);
    item.tohit = 12;
    item.todam = 24;

    item.number = 0;
    assert_eq!(generate(&item), "no more hunters bow (+3)");

    item.number = 1;
    assert_eq!(generate(&item), "hunters bow (+3)");

    item.set_identified(true);
    assert_eq!(generate(&item), "hunters bow (+3) (+12,+24)");

    item.tohit = 0;
    item.todam = 0;
    assert_eq!(generate(&item), "hunters bow (+3) (0,0)");

    item.tohit = -1;
    item.todam = -2;
    assert_eq!(generate(&item), "hunters bow (+3) (-1,-2)");
}

#[test]
fn test_composite_bow() {
    let mut item = generate_item::generate(Box::new(BowTemplate::CompositeBow), 0);
    item.tohit = 12;
    item.todam = 24;

    item.number = 0;
    assert_eq!(generate(&item), "no more composite bow (+4)");

    item.number = 1;
    assert_eq!(generate(&item), "composite bow (+4)");

    item.set_identified(true);
    assert_eq!(generate(&item), "composite bow (+4) (+12,+24)");

    item.tohit = 0;
    item.todam = 0;
    assert_eq!(generate(&item), "composite bow (+4) (0,0)");

    item.tohit = -1;
    item.todam = -2;
    assert_eq!(generate(&item), "composite bow (+4) (-1,-2)");
}

#[test]
fn test_war_bow() {
    let mut item = generate_item::generate(Box::new(BowTemplate::WarBow), 0);
    item.tohit = 12;
    item.todam = 24;

    item.number = 0;
    assert_eq!(generate(&item), "no more war bow (+5)");

    item.number = 1;
    assert_eq!(generate(&item), "war bow (+5)");

    item.set_identified(true);
    assert_eq!(generate(&item), "war bow (+5) (+12,+24)");

    item.tohit = 0;
    item.todam = 0;
    assert_eq!(generate(&item), "war bow (+5) (0,0)");

    item.tohit = -1;
    item.todam = -2;
    assert_eq!(generate(&item), "war bow (+5) (-1,-2)");
}

#[test]
fn test_double_bow() {
    let mut item = generate_item::generate(Box::new(BowTemplate::DoubleBow), 0);
    item.tohit = 12;
    item.todam = 24;

    item.number = 0;
    assert_eq!(generate(&item), "no more double bow (+6)");

    item.number = 1;
    assert_eq!(generate(&item), "double bow (+6)");

    item.set_identified(true);
    assert_eq!(generate(&item), "double bow (+6) (+12,+24)");

    item.tohit = 0;
    item.todam = 0;
    assert_eq!(generate(&item), "double bow (+6) (0,0)");

    item.tohit = -1;
    item.todam = -2;
    assert_eq!(generate(&item), "double bow (+6) (-1,-2)");
}

#[test]
fn test_siege_bow() {
    let mut item = generate_item::generate(Box::new(BowTemplate::SiegeBow), 0);
    item.tohit = 12;
    item.todam = 24;

    item.number = 0;
    assert_eq!(generate(&item), "no more siege bow (+7)");

    item.number = 1;
    assert_eq!(generate(&item), "siege bow (+7)");

    item.set_identified(true);
    assert_eq!(generate(&item), "siege bow (+7) (+12,+24)");

    item.tohit = 0;
    item.todam = 0;
    assert_eq!(generate(&item), "siege bow (+7) (0,0)");

    item.tohit = -1;
    item.todam = -2;
    assert_eq!(generate(&item), "siege bow (+7) (-1,-2)");
}

#[test]
fn test_warded_bow() {
    let mut item = generate_item::generate(Box::new(BowTemplate::WardedBow), 0);
    item.tohit = 12;
    item.todam = 24;

    item.number = 0;
    assert_eq!(generate(&item), "no more warded bow (+8)");

    item.number = 1;
    assert_eq!(generate(&item), "warded bow (+8)");

    item.set_identified(true);
    assert_eq!(generate(&item), "warded bow (+8) (+12,+24)");

    item.tohit = 0;
    item.todam = 0;
    assert_eq!(generate(&item), "warded bow (+8) (0,0)");

    item.tohit = -1;
    item.todam = -2;
    assert_eq!(generate(&item), "warded bow (+8) (-1,-2)");
}

#[test]
fn test_sling() {
    let mut item = generate_item::generate(Box::new(SlingTemplate::Sling), 0);
    item.tohit = 12;
    item.todam = 24;

    item.number = 0;
    assert_eq!(generate(&item), "no more sling (+2)");

    item.number = 1;
    assert_eq!(generate(&item), "sling (+2)");

    item.set_identified(true);
    assert_eq!(generate(&item), "sling (+2) (+12,+24)");

    item.tohit = 0;
    item.todam = 0;
    assert_eq!(generate(&item), "sling (+2) (0,0)");

    item.tohit = -1;
    item.todam = -2;
    assert_eq!(generate(&item), "sling (+2) (-1,-2)");
}

#[test]
fn test_light_crossbow() {
    let mut item = generate_item::generate(Box::new(CrossbowTemplate::LightCrossbow), 0);
    item.tohit = 12;
    item.todam = 24;

    item.number = 0;
    assert_eq!(generate(&item), "no more light crossbow (+2)");

    item.number = 1;
    assert_eq!(generate(&item), "light crossbow (+2)");

    item.set_identified(true);
    assert_eq!(generate(&item), "light crossbow (+2) (+12,+24)");

    item.tohit = 0;
    item.todam = 0;
    assert_eq!(generate(&item), "light crossbow (+2) (0,0)");

    item.tohit = -1;
    item.todam = -2;
    assert_eq!(generate(&item), "light crossbow (+2) (-1,-2)");
}

#[test]
fn test_heavy_crossbow() {
    let mut item = generate_item::generate(Box::new(CrossbowTemplate::HeavyCrossbow), 0);
    item.tohit = 12;
    item.todam = 24;

    item.number = 0;
    assert_eq!(generate(&item), "no more heavy crossbow (+4)");

    item.number = 1;
    assert_eq!(generate(&item), "heavy crossbow (+4)");

    item.set_identified(true);
    assert_eq!(generate(&item), "heavy crossbow (+4) (+12,+24)");

    item.tohit = 0;
    item.todam = 0;
    assert_eq!(generate(&item), "heavy crossbow (+4) (0,0)");

    item.tohit = -1;
    item.todam = -2;
    assert_eq!(generate(&item), "heavy crossbow (+4) (-1,-2)");
}

#[test]
fn test_siege_crossbow() {
    let mut item = generate_item::generate(Box::new(CrossbowTemplate::SiegeCrossbow), 0);
    item.tohit = 12;
    item.todam = 24;

    item.number = 0;
    assert_eq!(generate(&item), "no more siege crossbow (+6)");

    item.number = 1;
    assert_eq!(generate(&item), "siege crossbow (+6)");

    item.set_identified(true);
    assert_eq!(generate(&item), "siege crossbow (+6) (+12,+24)");

    item.tohit = 0;
    item.todam = 0;
    assert_eq!(generate(&item), "siege crossbow (+6) (0,0)");

    item.tohit = -1;
    item.todam = -2;
    assert_eq!(generate(&item), "siege crossbow (+6) (-1,-2)");
}

#[test]
fn test_ballista() {
    let mut item = generate_item::generate(Box::new(CrossbowTemplate::Ballista), 0);
    item.tohit = 12;
    item.todam = 24;

    item.number = 0;
    assert_eq!(generate(&item), "no more ballista (+8)");

    item.number = 1;
    assert_eq!(generate(&item), "ballista (+8)");

    item.set_identified(true);
    assert_eq!(generate(&item), "ballista (+8) (+12,+24)");

    item.tohit = 0;
    item.todam = 0;
    assert_eq!(generate(&item), "ballista (+8) (0,0)");

    item.tohit = -1;
    item.todam = -2;
    assert_eq!(generate(&item), "ballista (+8) (-1,-2)");
}
