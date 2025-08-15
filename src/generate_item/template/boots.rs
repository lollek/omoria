use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{BootsSubType, ItemSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BootsTemplate {
    SoftLeatherShoes,
    SoftLeatherBoots,
    HardLeatherBoots,
    Sandals,
    ChainBoots,
    LightPlatedBoots,
    SharkskinBoots,
    DemonhideBoots,
    WyrmhideBoot,
}

impl BootsTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(BootsTemplate::SoftLeatherShoes),
            Box::new(BootsTemplate::SoftLeatherBoots),
            Box::new(BootsTemplate::HardLeatherBoots),
            Box::new(BootsTemplate::Sandals),
            Box::new(BootsTemplate::ChainBoots),
            Box::new(BootsTemplate::LightPlatedBoots),
            Box::new(BootsTemplate::SharkskinBoots),
            Box::new(BootsTemplate::DemonhideBoots),
            Box::new(BootsTemplate::WyrmhideBoot),
        ]
    }
    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        BootsTemplate::vec().into_iter()
    }
}

impl ItemTemplate for BootsTemplate {
    /* TODO add fn create

static void mt__boots(treasure_type *treasure_ptr, const long level,
                      const bool is_magic, const bool is_special,
                      const bool is_cursed, const bool forceit) {
  if (is_magic) {
    treasure_ptr->toac = mt__m_bonus(1, 20, level, forceit);
    if (is_special) {
      switch (randint(16)) {
      case 1:
        strcat(treasure_ptr->name, " of Speed");
        treasure_ptr->flags |= Speed_worn_bit;
        treasure_ptr->p1 = 1;
        treasure_ptr->cost += 500000;
        break;
      case 2:
      case 3:
      case 4:
      case 5:
        strcat(treasure_ptr->name, " of Stealth");
        treasure_ptr->flags |= Stealth_worn_bit;
        treasure_ptr->cost += 50000;
        break;
      default:
        if (treasure_ptr->subval == 4 && randint(6) == 1) {
          /* Pair of Sandals */
          strcat(treasure_ptr->name, " of Dryadkind");
          treasure_ptr->flags |= Charisma_worn_bit | Feather_Fall_worn_bit |
                                 See_Invisible_worn_bit | Free_Action_worn_bit;
          treasure_ptr->flags2 |= Magic_proof_worn_bit;
          treasure_ptr->p1 = 3;
          treasure_ptr->cost = 1; /*{see magi item}*/
          break;
        }
      }
    } else {
      strcat(treasure_ptr->name, " of Slow descent");
      treasure_ptr->flags |= Feather_Fall_worn_bit;
      treasure_ptr->cost += 25000;
    }
  } else if (is_cursed) {
    treasure_ptr->cost = 0;
    treasure_ptr->ac = -mt__m_bonus(2, 45, level, forceit);

    switch (randint(3)) {
    case 1:
      strcat(treasure_ptr->name, " of Slowness");
      treasure_ptr->flags |= Cursed_worn_bit | Speed_worn_bit;
      treasure_ptr->p1 = -1;
      break;
    case 2:
      strcat(treasure_ptr->name, " of Noise");
      treasure_ptr->flags |= Cursed_worn_bit | Aggravation_worn_bit;
      break;
    case 3:
      strcat(treasure_ptr->name, " of Great Mass");
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->weight *= 5;
      break;
    }
  }
}
     */
    fn name(&self) -> &str {
        match self {
            BootsTemplate::SoftLeatherShoes => "Soft Leather Shoes^ [%P6,%P4]",
            BootsTemplate::SoftLeatherBoots => "Soft Leather Boots^ [%P6,%P4]",
            BootsTemplate::HardLeatherBoots => "Hard Leather Boots^ [%P6,%P4]",
            BootsTemplate::Sandals => "Sandals^ [%P6,%P4]",
            BootsTemplate::ChainBoots => "Chain Boots^ [%P6,%P4]",
            BootsTemplate::LightPlatedBoots => "Light Plated Boots^ [%P6,%P4]",
            BootsTemplate::SharkskinBoots => "Sharkskin Boots^ [%P6,%P4]",
            BootsTemplate::DemonhideBoots => "Demonhide Boots^ [%P6,%P4]",
            BootsTemplate::WyrmhideBoot => "Wyrmhide Boots^ [%P6,%P4]",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Boots
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
            BootsTemplate::SoftLeatherShoes => 4,
            BootsTemplate::SoftLeatherBoots => 7,
            BootsTemplate::HardLeatherBoots => 12,
            BootsTemplate::Sandals => 1,
            BootsTemplate::ChainBoots => 100,
            BootsTemplate::LightPlatedBoots => 150,
            BootsTemplate::SharkskinBoots => 400,
            BootsTemplate::DemonhideBoots => 500,
            BootsTemplate::WyrmhideBoot => 1000,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            BootsTemplate::SoftLeatherShoes => ItemSubType::Boots(BootsSubType::SoftLeatherShoes),
            BootsTemplate::SoftLeatherBoots => ItemSubType::Boots(BootsSubType::SoftLeatherBoots),
            BootsTemplate::HardLeatherBoots => ItemSubType::Boots(BootsSubType::HardLeatherBoots),
            BootsTemplate::Sandals => ItemSubType::Boots(BootsSubType::Sandals),
            BootsTemplate::ChainBoots => ItemSubType::Boots(BootsSubType::ChainBoots),
            BootsTemplate::LightPlatedBoots => ItemSubType::Boots(BootsSubType::LightPlatedBoots),
            BootsTemplate::SharkskinBoots => ItemSubType::Boots(BootsSubType::SharkskinBoots),
            BootsTemplate::DemonhideBoots => ItemSubType::Boots(BootsSubType::DemonhideBoots),
            BootsTemplate::WyrmhideBoot => ItemSubType::Boots(BootsSubType::WyrmhideBoot),
        }
    }

    fn weight(&self) -> u16 {
        match self {
            BootsTemplate::SoftLeatherShoes => 5,
            BootsTemplate::SoftLeatherBoots => 20,
            BootsTemplate::HardLeatherBoots => 40,
            BootsTemplate::Sandals => 1,
            BootsTemplate::ChainBoots => 60,
            BootsTemplate::LightPlatedBoots => 80,
            BootsTemplate::SharkskinBoots => 50,
            BootsTemplate::DemonhideBoots => 50,
            BootsTemplate::WyrmhideBoot => 50,
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
            BootsTemplate::SoftLeatherShoes => 1,
            BootsTemplate::SoftLeatherBoots => 2,
            BootsTemplate::HardLeatherBoots => 3,
            BootsTemplate::Sandals => 0,
            BootsTemplate::ChainBoots => 4,
            BootsTemplate::LightPlatedBoots => 5,
            BootsTemplate::SharkskinBoots => 6,
            BootsTemplate::DemonhideBoots => 7,
            BootsTemplate::WyrmhideBoot => 8,
        }
    }

    fn modifier_to_ac(&self) -> i16 {
        0
    }
    fn damage(&self) -> &str {
        "0d0"
    }

    fn item_level(&self) -> u8 {
        match self {
            BootsTemplate::SoftLeatherShoes => 1,
            BootsTemplate::SoftLeatherBoots => 4,
            BootsTemplate::HardLeatherBoots => 6,
            BootsTemplate::Sandals => 1,
            BootsTemplate::ChainBoots => 10,
            BootsTemplate::LightPlatedBoots => 16,
            BootsTemplate::SharkskinBoots => 30,
            BootsTemplate::DemonhideBoots => 40,
            BootsTemplate::WyrmhideBoot => 50,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
