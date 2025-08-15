use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{BeltSubType, ItemSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BeltTemplate {
    Sash,
    LightBelt,
    Belt,
    HeavyBelt,
    LightPlatedBelt,
    SharkskinBelt,
    DemonhideBelt,
    WyrmhideBelt,
}

impl BeltTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(BeltTemplate::Sash),
            Box::new(BeltTemplate::LightBelt),
            Box::new(BeltTemplate::Belt),
            Box::new(BeltTemplate::HeavyBelt),
            Box::new(BeltTemplate::LightPlatedBelt),
            Box::new(BeltTemplate::SharkskinBelt),
            Box::new(BeltTemplate::DemonhideBelt),
            Box::new(BeltTemplate::WyrmhideBelt),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        BeltTemplate::vec().into_iter()
    }
}

impl ItemTemplate for BeltTemplate {
    /* TODO add fn create

static void mt__belt(treasure_type *treasure_ptr, const long level,
                     const bool is_magic, const bool is_special,
                     const bool is_cursed, const bool forceit) {
  if (is_magic) {
    treasure_ptr->toac = mt__m_bonus(1, 20, level, forceit);
    if (is_special) {
      switch (treasure_ptr->subval) {

      case 1: /* Girdle */
        treasure_ptr->flags2 |= Increase_carry_worn_bit;

        switch (randint(16)) {
        case 1:
          if (randint(3) == 1) {
            strcat(treasure_ptr->name, " of Titan Strength");
            treasure_ptr->flags |= Resist_Lightning_worn_bit |
                                   Resist_Fire_worn_bit | Resist_Cold_worn_bit |
                                   Resist_Acid_worn_bit |
                                   Regeneration_worn_bit | Free_Action_worn_bit;
            treasure_ptr->flags2 |= Magic_proof_worn_bit;
            treasure_ptr->p1 = 7;
            treasure_ptr->cost += 7500000;
          } else {
            strcat(treasure_ptr->name, " of Storm Giant Strength");
            treasure_ptr->flags |=
                Resist_Lightning_worn_bit | Resist_Acid_worn_bit;
            treasure_ptr->flags2 |= Magic_proof_worn_bit;
            treasure_ptr->p1 = 6;
            treasure_ptr->cost += 3500000;
          }
          break;
        case 2:
          strcat(treasure_ptr->name, " of Cloud Giant Strength");
          treasure_ptr->flags |=
              Resist_Lightning_worn_bit | Resist_Acid_worn_bit;
          treasure_ptr->p1 = 5;
          treasure_ptr->cost += 2000000;
          break;
        case 3:
        case 4:
          strcat(treasure_ptr->name, " of Fire Giant Strength");
          treasure_ptr->flags |= Resist_Fire_worn_bit;
          treasure_ptr->p1 = 4;
          treasure_ptr->cost += 1750000;
          break;
        case 5:
        case 6:
        case 7:
          strcat(treasure_ptr->name, " of Frost Giant Strength");
          treasure_ptr->flags |= Resist_Cold_worn_bit;
          treasure_ptr->p1 = 3;
          treasure_ptr->cost += 1250000;
          break;
        case 8:
        case 9:
        case 10:
        case 11:
          strcat(treasure_ptr->name, " of Stone Giant Strength");
          treasure_ptr->p1 = 2;
          treasure_ptr->cost += 800000;
          break;
        case 12:
        case 13:
        case 14:
        case 15:
        case 16:
          strcat(treasure_ptr->name, " of Hill Giant Strength");
          treasure_ptr->p1 = 1;
          treasure_ptr->cost += 600000;
          break;
        }
        treasure_ptr->tohit = treasure_ptr->p1;
        treasure_ptr->todam = treasure_ptr->p1;
        break;

      case 10: /* Silver Belt Buckle, Gold Belt Buckle */
      case 11:
        switch (randint(2)) {
        case 1:
          strcat(treasure_ptr->name, " of Deflection");
          treasure_ptr->flags2 |= Magic_proof_worn_bit;
          treasure_ptr->toac += randint(5);
          treasure_ptr->cost += treasure_ptr->toac * 20000;
          break;
        case 2:
          strcat(treasure_ptr->name, " of Improved Digestion");
          treasure_ptr->flags |=
              Sustain_Stat_worn_bit | Slow_Digestion_worn_bit;
          treasure_ptr->p1 = 2;
          treasure_ptr->cost += 100000;
          break;
        }
        break;

      case 13: /* Leather Belt */
        strcat(treasure_ptr->name, " of Dwarvenkind");
        treasure_ptr->flags |=
            Infra_Vision_worn_bit | Tunneling_worn_bit | Sustain_Stat_worn_bit;
        treasure_ptr->flags2 |= Magic_proof_worn_bit;
        treasure_ptr->p1 = 2;
        treasure_ptr->cost += 70000;
        break;
      }
    }
  } else if (is_cursed) {
    treasure_ptr->flags |= Cursed_worn_bit;
    treasure_ptr->toac = -mt__m_bonus(1, 45, level, forceit);
    treasure_ptr->cost = 0;
    if (is_special) {
      switch (treasure_ptr->subval) {
      case 1: /* Girdle */
        switch (randint(2)) {
        case 1:
          strcat(treasure_ptr->name, " of Sex Change");
          treasure_ptr->flags |= Charisma_worn_bit;
          treasure_ptr->p1 = -2;
          break;
        case 2:
          strcat(treasure_ptr->name, " of Weakness");
          treasure_ptr->flags |= Strength_worn_bit;
          treasure_ptr->p1 = -1;
          break;
        }
        break;

      case 10: /* Silver Belt Buckle, Gold Belt Buckle */
      case 11:
        strcat(treasure_ptr->name, " of Fear");
        treasure_ptr->flags |= Cursed_worn_bit | Timidness_worn_bit;
        treasure_ptr->p1 = -1;
        break;
      case 13: /* Leather Belt */
        strcat(treasure_ptr->name, " of Hunger");
        treasure_ptr->flags |= Cursed_worn_bit;
        treasure_ptr->flags2 |= Hunger_worn_bit;
        treasure_ptr->p1 = -1;
        break;
      }
      treasure_ptr->p1 *= randint(5);
    }
  }
}
     */
    fn name(&self) -> &str {
        match self {
            BeltTemplate::Sash => "Sash^ [%P6,%P4]",
            BeltTemplate::LightBelt => "Light Belt^ [%P6,%P4]",
            BeltTemplate::Belt => "Belt^ [%P6,%P4]",
            BeltTemplate::HeavyBelt => "Heavy Belt^ [%P6,%P4]",
            BeltTemplate::LightPlatedBelt => "Light Plated Belt^ [%P6,%P4]",
            BeltTemplate::SharkskinBelt => "Sharkskin Belt^ [%P6,%P4]",
            BeltTemplate::DemonhideBelt => "Demonhide Belt^ [%P6,%P4]",
            BeltTemplate::WyrmhideBelt => "Wyrmhide Belt^ [%P6,%P4]",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Belt
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
            BeltTemplate::Sash => 1,
            BeltTemplate::LightBelt => 5,
            BeltTemplate::Belt => 7,
            BeltTemplate::HeavyBelt => 20,
            BeltTemplate::LightPlatedBelt => 50,
            BeltTemplate::SharkskinBelt => 200,
            BeltTemplate::DemonhideBelt => 300,
            BeltTemplate::WyrmhideBelt => 500,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            BeltTemplate::Sash => ItemSubType::Belt(BeltSubType::Sash),
            BeltTemplate::LightBelt => ItemSubType::Belt(BeltSubType::LightBelt),
            BeltTemplate::Belt => ItemSubType::Belt(BeltSubType::Belt),
            BeltTemplate::HeavyBelt => ItemSubType::Belt(BeltSubType::HeavyBelt),
            BeltTemplate::LightPlatedBelt => ItemSubType::Belt(BeltSubType::LightPlatedBelt),
            BeltTemplate::SharkskinBelt => ItemSubType::Belt(BeltSubType::SharkskinBelt),
            BeltTemplate::DemonhideBelt => ItemSubType::Belt(BeltSubType::DemonhideBelt),
            BeltTemplate::WyrmhideBelt => ItemSubType::Belt(BeltSubType::WyrmhideBelt),
        }
    }

    fn weight(&self) -> u16 {
        match self {
            BeltTemplate::Sash => 1,
            BeltTemplate::LightBelt => 5,
            BeltTemplate::Belt => 10,
            BeltTemplate::HeavyBelt => 15,
            BeltTemplate::LightPlatedBelt => 30,
            BeltTemplate::SharkskinBelt => 10,
            BeltTemplate::DemonhideBelt => 10,
            BeltTemplate::WyrmhideBelt => 10,
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
            BeltTemplate::Sash => 0,
            BeltTemplate::LightBelt => 0,
            BeltTemplate::Belt => 0,
            BeltTemplate::HeavyBelt => 1,
            BeltTemplate::LightPlatedBelt => 2,
            BeltTemplate::SharkskinBelt => 3,
            BeltTemplate::DemonhideBelt => 4,
            BeltTemplate::WyrmhideBelt => 5,
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
            BeltTemplate::Sash => 0,
            BeltTemplate::LightBelt => 0,
            BeltTemplate::Belt => 0,
            BeltTemplate::HeavyBelt => 6,
            BeltTemplate::LightPlatedBelt => 15,
            BeltTemplate::SharkskinBelt => 30,
            BeltTemplate::DemonhideBelt => 40,
            BeltTemplate::WyrmhideBelt => 50,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
