use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{HelmSubType, ItemSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum HelmTemplate {
    ClothHat,
    SoftLeatherCap,
    HardLeatherCap,
    MetalCap,
    FullHelm,
    GreatHelm,
    WingedHelm,
    SilverCrown,
    SilverMask,
    GoldenCrown,
    GoldenMask,
    JewelEncrustedCrown,
}

impl HelmTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(HelmTemplate::ClothHat),
            Box::new(HelmTemplate::SoftLeatherCap),
            Box::new(HelmTemplate::HardLeatherCap),
            Box::new(HelmTemplate::MetalCap),
            Box::new(HelmTemplate::FullHelm),
            Box::new(HelmTemplate::GreatHelm),
            Box::new(HelmTemplate::WingedHelm),
            Box::new(HelmTemplate::SilverCrown),
            Box::new(HelmTemplate::SilverMask),
            Box::new(HelmTemplate::GoldenCrown),
            Box::new(HelmTemplate::GoldenMask),
            Box::new(HelmTemplate::JewelEncrustedCrown),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        HelmTemplate::vec().into_iter()
    }
}

impl ItemTemplate for HelmTemplate {
    /* TODO add fn create
static void mt__helms(treasure_type *treasure_ptr, const long level,
                      const bool is_magic, const bool is_special,
                      const bool is_cursed, const bool forceit) {
  if (is_magic) {
    treasure_ptr->toac = mt__m_bonus(1, 20, level, forceit);
    if (is_special)
      switch (treasure_ptr->subval) {
      case 1:  case 2:  case 3:  case 4:  case 5:
      case 12: case 13: case 14: case 15:
      case 16: case 17: case 18:
        switch (randint(3)) {
        case 1:
          strcat(treasure_ptr->name, " of Intelligence");
          treasure_ptr->flags |= Intelligence_worn_bit;
          treasure_ptr->p1 = randint(2);
          treasure_ptr->cost += treasure_ptr->p1 * 50000;
          break;
        case 2:
          strcat(treasure_ptr->name, " of Wisdom");
          treasure_ptr->flags |= Wisdom_worn_bit;
          treasure_ptr->p1 = randint(2);
          treasure_ptr->cost += treasure_ptr->p1 * 50000;
          break;
        case 3:
          strcat(treasure_ptr->name, " of Infra-Vision");
          treasure_ptr->flags |= Infra_Vision_worn_bit;
          treasure_ptr->p1 = 1 + randint(4);
          treasure_ptr->cost += treasure_ptr->p1 * 25000;
          break;
        }
        break;

      case 6:  case 7:  case 8:  case 9:  case 10:
      case 19: case 20: case 21: case 22: case 23:
        switch (randint(6)) {
        case 1:
          strcat(treasure_ptr->name, " of Might");
          treasure_ptr->flags |= Free_Action_worn_bit | Constitution_worn_bit |
                                 Strength_worn_bit | Dexterity_worn_bit;
          treasure_ptr->p1 = randint(3);
          treasure_ptr->cost += 100000 + treasure_ptr->p1 * 50000;
          break;
        case 2:
          strcat(treasure_ptr->name, " of Lordliness");
          treasure_ptr->flags |= Wisdom_worn_bit | Charisma_worn_bit;
          treasure_ptr->p1 = randint(3);
          treasure_ptr->cost += 100000 + treasure_ptr->p1 * 50000;
          break;
        case 3:
          strcat(treasure_ptr->name, " of the Magi");
          treasure_ptr->flags |= Free_Action_worn_bit | Strength_worn_bit |
                                 Constitution_worn_bit | Dexterity_worn_bit;
          treasure_ptr->p1 = randint(3);
          treasure_ptr->cost += 300000 + treasure_ptr->p1 * 50000;
          break;
        case 4:
          strcat(treasure_ptr->name, " of Beauty");
          treasure_ptr->flags |= Charisma_worn_bit;
          treasure_ptr->p1 = randint(3);
          treasure_ptr->cost += 75000;
          break;
        case 5:
          strcat(treasure_ptr->name, " of Seeing");
          treasure_ptr->flags |= See_Invisible_worn_bit | Searching_worn_bit;
          treasure_ptr->p1 = 1 + randint(4);
          treasure_ptr->cost += 100000 + treasure_ptr->p1 * 10000;
          break;
        case 6:
          strcat(treasure_ptr->name, " of Regeneration");
          treasure_ptr->flags |= Regeneration_worn_bit;
          treasure_ptr->cost += 150000;
          break;
        }
        break;
      case 11:
        strcat(treasure_ptr->name, " of Hobbitkind");
        treasure_ptr->flags |= Infra_Vision_worn_bit | See_Invisible_worn_bit |
                               Free_Action_worn_bit | Searching_worn_bit;
        treasure_ptr->p1 = 5;
        treasure_ptr->cost += 170000;
        break;
      }
  } else if (is_cursed) {
    treasure_ptr->flags |= Cursed_worn_bit;
    treasure_ptr->toac = -mt__m_bonus(1, 45, level, forceit);
    treasure_ptr->cost = 0;
    if (is_special) {
      switch (randint(15)) {
      case 1:
      case 2:
        strcat(treasure_ptr->name, " of Stupidity");
        treasure_ptr->flags |= Intelligence_worn_bit;
        treasure_ptr->p1 = -1;
        break;
      case 3:
      case 4:
        strcat(treasure_ptr->name, " of Dullness");
        treasure_ptr->flags |= Wisdom_worn_bit;
        treasure_ptr->p1 = -1;
        break;
      case 5:
      case 6:
        strcat(treasure_ptr->name, " of Blindness");
        treasure_ptr->flags |= Blindness_worn_bit;
        break;
      case 7:
      case 8:
        strcat(treasure_ptr->name, " of Timidness");
        treasure_ptr->flags |= Timidness_worn_bit;
        break;
      case 9:
      case 10:
        strcat(treasure_ptr->name, " of Weakness");
        treasure_ptr->flags |= Strength_worn_bit;
        treasure_ptr->p1 = -1;
        break;
      case 11:
      case 12:
        strcat(treasure_ptr->name, " of Teleportation");
        treasure_ptr->flags |= Teleportation_worn_bit;
        break;
      case 13:
      case 14:
        strcat(treasure_ptr->name, " of Ugliness");
        treasure_ptr->flags |= Charisma_worn_bit;
        treasure_ptr->p1 = -1;
        break;
      case 15:
        strcat(treasure_ptr->name, " of **TOTAL DOOM**");
        treasure_ptr->flags |=
            Cursed_worn_bit | Strength_worn_bit | Dexterity_worn_bit |
            Constitution_worn_bit | Intelligence_worn_bit | Wisdom_worn_bit |
            Charisma_worn_bit | Stealth_worn_bit | Aggravation_worn_bit |
            Teleportation_worn_bit | Blindness_worn_bit | Timidness_worn_bit;
        treasure_ptr->flags2 |= Hunger_worn_bit | Known_cursed_bit;
        treasure_ptr->p1 = -5;
        break;
      }
      treasure_ptr->p1 *= randint(5);
    }
  }
}
     */
    fn name(&self) -> &str {
        match self {
            HelmTemplate::ClothHat => "Cloth Hat^ [%P6,%P4]",
            HelmTemplate::SoftLeatherCap => "Soft Leather Cap^ [%P6,%P4]",
            HelmTemplate::HardLeatherCap => "Hard Leather Cap^ [%P6,%P4]",
            HelmTemplate::MetalCap => "Metal Cap^ [%P6,%P4]",
            HelmTemplate::FullHelm => "Full Helm^ [%P6,%P4]",
            HelmTemplate::GreatHelm => "Great Helm^ [%P6,%P4]",
            HelmTemplate::WingedHelm => "Winged Helm^ [%P6,%P4]",
            HelmTemplate::SilverCrown => "Silver Crown^ [%P6,%P4] (%P1)",
            HelmTemplate::SilverMask => "Silver Mask^ [%P6,%P4] (%P1)",
            HelmTemplate::GoldenCrown => "Golden Crown^ [%P6,%P4] (%P1)",
            HelmTemplate::GoldenMask => "Golden Mask^ [%P6,%P4] (%P1)",
            HelmTemplate::JewelEncrustedCrown => "Jewel Encrusted Crown^ [%P6,%P4] (%P1)",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Helm
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
            HelmTemplate::ClothHat => 5,
            HelmTemplate::SoftLeatherCap => 10,
            HelmTemplate::HardLeatherCap => 15,
            HelmTemplate::MetalCap => 30,
            HelmTemplate::FullHelm => 75,
            HelmTemplate::GreatHelm => 200,
            HelmTemplate::WingedHelm => 300,
            HelmTemplate::SilverCrown => 250,
            HelmTemplate::SilverMask => 350,
            HelmTemplate::GoldenCrown => 500,
            HelmTemplate::GoldenMask => 1000,
            HelmTemplate::JewelEncrustedCrown => 1000,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            HelmTemplate::ClothHat => ItemSubType::Helm(HelmSubType::ClothHat),
            HelmTemplate::SoftLeatherCap => ItemSubType::Helm(HelmSubType::SoftLeatherCap),
            HelmTemplate::HardLeatherCap => ItemSubType::Helm(HelmSubType::HardLeatherCap),
            HelmTemplate::MetalCap => ItemSubType::Helm(HelmSubType::MetalCap),
            HelmTemplate::FullHelm => ItemSubType::Helm(HelmSubType::FullHelm),
            HelmTemplate::GreatHelm => ItemSubType::Helm(HelmSubType::GreatHelm),
            HelmTemplate::WingedHelm => ItemSubType::Helm(HelmSubType::WingedHelm),
            HelmTemplate::SilverCrown => ItemSubType::Helm(HelmSubType::SilverCrown),
            HelmTemplate::SilverMask => ItemSubType::Helm(HelmSubType::SilverMask),
            HelmTemplate::GoldenCrown => ItemSubType::Helm(HelmSubType::GoldenCrown),
            HelmTemplate::GoldenMask => ItemSubType::Helm(HelmSubType::GoldenMask),
            HelmTemplate::JewelEncrustedCrown => {
                ItemSubType::Helm(HelmSubType::JewelEncrustedCrown)
            }
        }
    }

    fn weight(&self) -> u16 {
        match self {
            HelmTemplate::ClothHat => 5,
            HelmTemplate::SoftLeatherCap => 10,
            HelmTemplate::HardLeatherCap => 20,
            HelmTemplate::MetalCap => 30,
            HelmTemplate::FullHelm => 75,
            HelmTemplate::GreatHelm => 80,
            HelmTemplate::WingedHelm => 80,
            HelmTemplate::SilverCrown => 30,
            HelmTemplate::SilverMask => 40,
            HelmTemplate::GoldenCrown => 30,
            HelmTemplate::GoldenMask => 50,
            HelmTemplate::JewelEncrustedCrown => 50,
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
            HelmTemplate::ClothHat => 1,
            HelmTemplate::SoftLeatherCap => 2,
            HelmTemplate::HardLeatherCap => 3,
            HelmTemplate::MetalCap => 4,
            HelmTemplate::FullHelm => 6,
            HelmTemplate::GreatHelm => 7,
            HelmTemplate::WingedHelm => 8,
            HelmTemplate::SilverCrown => 0,
            HelmTemplate::SilverMask => 1,
            HelmTemplate::GoldenCrown => 0,
            HelmTemplate::GoldenMask => 2,
            HelmTemplate::JewelEncrustedCrown => 0,
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
            HelmTemplate::ClothHat => 1,
            HelmTemplate::SoftLeatherCap => 2,
            HelmTemplate::HardLeatherCap => 5,
            HelmTemplate::MetalCap => 10,
            HelmTemplate::FullHelm => 20,
            HelmTemplate::GreatHelm => 30,
            HelmTemplate::WingedHelm => 50,
            HelmTemplate::SilverCrown => 20,
            HelmTemplate::SilverMask => 30,
            HelmTemplate::GoldenCrown => 40,
            HelmTemplate::GoldenMask => 50,
            HelmTemplate::JewelEncrustedCrown => 75,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
