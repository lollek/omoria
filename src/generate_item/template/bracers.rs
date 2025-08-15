use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{BracersSubType, ItemSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BracersTemplate {
    BracersOfProtection,
    BracersOfDefense,
    BracersOfShielding,
    MithrilBracers,
    AdamantiteBracers,
    BracersOfWeaponAttraction,
    SilverBraceletOfWarding,
    SilverBracelet,
    GoldBracelet,
    PlatinumBracelet,
    LeatherBracers,
    StuddedLeatherBracers,
    LightPlatedBracers,
    SharkskinBracers,
    DemonhideBracers,
    WyrmhideBracers,
    ChainmailBracers,
    LamellarBracers,
}

impl BracersTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(BracersTemplate::BracersOfProtection),
            Box::new(BracersTemplate::BracersOfDefense),
            Box::new(BracersTemplate::BracersOfShielding),
            Box::new(BracersTemplate::MithrilBracers),
            Box::new(BracersTemplate::AdamantiteBracers),
            Box::new(BracersTemplate::BracersOfWeaponAttraction),
            Box::new(BracersTemplate::SilverBraceletOfWarding),
            Box::new(BracersTemplate::SilverBracelet),
            Box::new(BracersTemplate::GoldBracelet),
            Box::new(BracersTemplate::PlatinumBracelet),
            Box::new(BracersTemplate::LeatherBracers),
            Box::new(BracersTemplate::StuddedLeatherBracers),
            Box::new(BracersTemplate::LightPlatedBracers),
            Box::new(BracersTemplate::SharkskinBracers),
            Box::new(BracersTemplate::DemonhideBracers),
            Box::new(BracersTemplate::WyrmhideBracers),
            Box::new(BracersTemplate::ChainmailBracers),
            Box::new(BracersTemplate::LamellarBracers),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        BracersTemplate::vec().into_iter()
    }
}

impl ItemTemplate for BracersTemplate {
    /* TODO add fn create

static void mt__gloves_and_gauntlets(treasure_type *treasure_ptr,
                                     const long level, const bool is_magic,
                                     const bool is_special,
                                     const bool is_cursed,
                                     const bool forceit) {
  if (is_magic) {
    treasure_ptr->toac = mt__m_bonus(1, 20, level, forceit);
    if (is_special) {
      /* gloves_and_gauntlets 5 is "Set of Cloth Gloves" */
      if (treasure_ptr->subval == 5 && randint(10) == 1) {
        strcat(treasure_ptr->name, " of the Hive");
        treasure_ptr->flags |= Dexterity_worn_bit;
        treasure_ptr->p1 = 2;
        treasure_ptr->cost += 50000;
      } else {
        switch (randint(5)) {
        case 1:
          strcat(treasure_ptr->name, " of Free Action");
          treasure_ptr->flags |= Free_Action_worn_bit;
          treasure_ptr->cost += 100000;
          break;
        case 2:
          strcat(treasure_ptr->name, " of Slaying");
          treasure_ptr->tohit = 1 + randint(3);
          treasure_ptr->todam = 1 + randint(3);
          treasure_ptr->cost +=
              (treasure_ptr->tohit + treasure_ptr->todam) * 25000;
          break;
        case 3:
          strcat(treasure_ptr->name, " of Thievery (%P1)");
          treasure_ptr->flags2 |= Disarm_worn_bit;
          treasure_ptr->flags |= Feather_Fall_worn_bit | See_Invisible_worn_bit;
          treasure_ptr->p1 = mt__m_bonus(5, 50, level, forceit);
          treasure_ptr->cost += 20000 + treasure_ptr->p1 * 5;
          break;
        case 4:
        case 5:
          strcat(treasure_ptr->name, " of Ogre Power");
          treasure_ptr->flags |= Slow_Digestion_worn_bit | Strength_worn_bit;
          treasure_ptr->p1 = randint(4);
          treasure_ptr->cost += 150000;
          break;
        }
      }
    }
  } else if (is_cursed) {
    if (is_special) {
      switch (randint(3)) {
      case 1:
        strcat(treasure_ptr->name, " of Clumsiness");
        treasure_ptr->flags |= Cursed_worn_bit | Dexterity_worn_bit;
        treasure_ptr->p1 = 1;
        break;
      case 2:
        strcat(treasure_ptr->name, " of Weakness");
        treasure_ptr->flags |= Cursed_worn_bit | Strength_worn_bit;
        treasure_ptr->p1 = 1;
        break;
      case 3:
        strcat(treasure_ptr->name, " of Ogre Intelligence");
        treasure_ptr->flags |= Cursed_worn_bit | Intelligence_worn_bit;
        treasure_ptr->p1 = 1;
        break;
      }
    }
    treasure_ptr->flags |= Cursed_worn_bit;
    treasure_ptr->toac = -mt__m_bonus(1, 40, level, forceit);
    treasure_ptr->p1 = -mt__m_bonus(1, 10, level, forceit);
    treasure_ptr->cost = 0;
  }
}
     */
    fn name(&self) -> &str {
        match self {
            BracersTemplate::BracersOfProtection => "Bracers^ of Protection [%P6,%P4]",
            BracersTemplate::BracersOfDefense => "Bracers^ of Defense [%P6,%P4]",
            BracersTemplate::BracersOfShielding => "Bracers^ of Shielding [%P6,%P4]",
            BracersTemplate::MithrilBracers => "Mithril Bracers^ [%P6,%P4]",
            BracersTemplate::AdamantiteBracers => "Adamantite Bracers^ [%P6,%P4]",
            BracersTemplate::BracersOfWeaponAttraction => "Bracers^ of Weapon Attraction [%P6,%P4]",
            BracersTemplate::SilverBraceletOfWarding => "Silver Bracelet^ of Warding [%P6,%P4] (R)",
            BracersTemplate::SilverBracelet => "Silver Bracelet^ [%P6,%P4]",
            BracersTemplate::GoldBracelet => "Gold Bracelet^ [%P6,%P4]",
            BracersTemplate::PlatinumBracelet => "Platinum Bracelet^ [%P6,%P4]",
            BracersTemplate::LeatherBracers => "Leather Bracers^ [%P6,%P4]",
            BracersTemplate::StuddedLeatherBracers => "Studded Leather Bracers^ [%P6,%P4]",
            BracersTemplate::LightPlatedBracers => "Light Plated Bracers^ [%P6,%P4]",
            BracersTemplate::SharkskinBracers => "Sharkskin Bracers^ [%P6,%P4]",
            BracersTemplate::DemonhideBracers => "Demonhide Bracers^ [%P6,%P4]",
            BracersTemplate::WyrmhideBracers => "Wyrmhide Bracers^ [%P6,%P4]",
            BracersTemplate::ChainmailBracers => "Chainmail Bracers^ [%P6,%P4]",
            BracersTemplate::LamellarBracers => "Lamellar Bracers^ [%P6,%P4]",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Bracers
    }
    fn flags1(&self) -> u64 {
        0
    }
    fn flags2(&self) -> u64 {
        0
    }
    fn p1(&self) -> i64 {
        0
    }

    fn cost(&self) -> i64 {
        match self {
            BracersTemplate::BracersOfProtection => 1200,
            BracersTemplate::BracersOfDefense => 2400,
            BracersTemplate::BracersOfShielding => 3600,
            BracersTemplate::MithrilBracers => 4800,
            BracersTemplate::AdamantiteBracers => 6000,
            BracersTemplate::BracersOfWeaponAttraction => -1200,
            BracersTemplate::SilverBraceletOfWarding => 10000,
            BracersTemplate::SilverBracelet => 25,
            BracersTemplate::GoldBracelet => 50,
            BracersTemplate::PlatinumBracelet => 100,
            BracersTemplate::LeatherBracers => 4,
            BracersTemplate::StuddedLeatherBracers => 25,
            BracersTemplate::LightPlatedBracers => 100,
            BracersTemplate::SharkskinBracers => 200,
            BracersTemplate::DemonhideBracers => 250,
            BracersTemplate::WyrmhideBracers => 500,
            BracersTemplate::ChainmailBracers => 100,
            BracersTemplate::LamellarBracers => 200,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            BracersTemplate::BracersOfProtection => {
                ItemSubType::Bracers(BracersSubType::BracersOfProtection)
            }
            BracersTemplate::BracersOfDefense => {
                ItemSubType::Bracers(BracersSubType::BracersOfDefense)
            }
            BracersTemplate::BracersOfShielding => {
                ItemSubType::Bracers(BracersSubType::BracersOfShielding)
            }
            BracersTemplate::MithrilBracers => ItemSubType::Bracers(BracersSubType::MithrilBracers),
            BracersTemplate::AdamantiteBracers => {
                ItemSubType::Bracers(BracersSubType::AdamantiteBracers)
            }
            BracersTemplate::BracersOfWeaponAttraction => {
                ItemSubType::Bracers(BracersSubType::BracersOfWeaponAttraction)
            }
            BracersTemplate::SilverBraceletOfWarding => {
                ItemSubType::Bracers(BracersSubType::SilverBraceletOfWarding)
            }
            BracersTemplate::SilverBracelet => ItemSubType::Bracers(BracersSubType::SilverBracelet),
            BracersTemplate::GoldBracelet => ItemSubType::Bracers(BracersSubType::GoldBracelet),
            BracersTemplate::PlatinumBracelet => {
                ItemSubType::Bracers(BracersSubType::PlatinumBracelet)
            }
            BracersTemplate::LeatherBracers => ItemSubType::Bracers(BracersSubType::LeatherBracers),
            BracersTemplate::StuddedLeatherBracers => {
                ItemSubType::Bracers(BracersSubType::StuddedLeatherBracers)
            }
            BracersTemplate::LightPlatedBracers => {
                ItemSubType::Bracers(BracersSubType::LightPlatedBracers)
            }
            BracersTemplate::SharkskinBracers => {
                ItemSubType::Bracers(BracersSubType::SharkskinBracers)
            }
            BracersTemplate::DemonhideBracers => {
                ItemSubType::Bracers(BracersSubType::DemonhideBracers)
            }
            BracersTemplate::WyrmhideBracers => {
                ItemSubType::Bracers(BracersSubType::WyrmhideBracers)
            }
            BracersTemplate::ChainmailBracers => {
                ItemSubType::Bracers(BracersSubType::ChainmailBracers)
            }
            BracersTemplate::LamellarBracers => {
                ItemSubType::Bracers(BracersSubType::LamellarBracers)
            }
        }
    }

    fn weight(&self) -> u16 {
        match self {
            BracersTemplate::BracersOfProtection => 125,
            BracersTemplate::BracersOfDefense => 125,
            BracersTemplate::BracersOfShielding => 125,
            BracersTemplate::MithrilBracers => 125,
            BracersTemplate::AdamantiteBracers => 125,
            BracersTemplate::BracersOfWeaponAttraction => 125,
            BracersTemplate::SilverBraceletOfWarding => 5,
            BracersTemplate::SilverBracelet => 5,
            BracersTemplate::GoldBracelet => 5,
            BracersTemplate::PlatinumBracelet => 5,
            BracersTemplate::LeatherBracers => 5,
            BracersTemplate::StuddedLeatherBracers => 10,
            BracersTemplate::LightPlatedBracers => 30,
            BracersTemplate::SharkskinBracers => 10,
            BracersTemplate::DemonhideBracers => 10,
            BracersTemplate::WyrmhideBracers => 10,
            BracersTemplate::ChainmailBracers => 50,
            BracersTemplate::LamellarBracers => 40,
        }
    }

    fn number(&self) -> u16 {
        1
    }
    fn modifier_to_hit(&self) -> i16 {
        0
    }
    fn modifier_to_damage(&self) -> i16 {
        0
    }

    fn base_ac(&self) -> i16 {
        match self {
            BracersTemplate::BracersOfProtection => 6,
            BracersTemplate::BracersOfDefense => 7,
            BracersTemplate::BracersOfShielding => 8,
            BracersTemplate::MithrilBracers => 9,
            BracersTemplate::AdamantiteBracers => 10,
            BracersTemplate::BracersOfWeaponAttraction => -6,
            BracersTemplate::SilverBraceletOfWarding => 5,
            BracersTemplate::SilverBracelet => 0,
            BracersTemplate::GoldBracelet => 0,
            BracersTemplate::PlatinumBracelet => 0,
            BracersTemplate::LeatherBracers => 1,
            BracersTemplate::StuddedLeatherBracers => 2,
            BracersTemplate::LightPlatedBracers => 3,
            BracersTemplate::SharkskinBracers => 3,
            BracersTemplate::DemonhideBracers => 4,
            BracersTemplate::WyrmhideBracers => 5,
            BracersTemplate::ChainmailBracers => 5,
            BracersTemplate::LamellarBracers => 7,
        }
    }

    fn modifier_to_ac(&self) -> i16 {
        0
    }
    fn damage(&self) -> &str {
        "1d1"
    }

    fn item_level(&self) -> u8 {
        match self {
            BracersTemplate::BracersOfProtection => 35,
            BracersTemplate::BracersOfDefense => 40,
            BracersTemplate::BracersOfShielding => 45,
            BracersTemplate::MithrilBracers => 50,
            BracersTemplate::AdamantiteBracers => 55,
            BracersTemplate::BracersOfWeaponAttraction => 30,
            BracersTemplate::SilverBraceletOfWarding => 50,
            BracersTemplate::SilverBracelet => 2,
            BracersTemplate::GoldBracelet => 5,
            BracersTemplate::PlatinumBracelet => 8,
            BracersTemplate::LeatherBracers => 1,
            BracersTemplate::StuddedLeatherBracers => 5,
            BracersTemplate::LightPlatedBracers => 15,
            BracersTemplate::SharkskinBracers => 30,
            BracersTemplate::DemonhideBracers => 40,
            BracersTemplate::WyrmhideBracers => 50,
            BracersTemplate::ChainmailBracers => 25,
            BracersTemplate::LamellarBracers => 44,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
