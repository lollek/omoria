use crate::conversion::item_subtype;
use crate::data;
use crate::generate_item::template::MaceTemplate;
use crate::misc;
use crate::misc::rs2item_name;
use crate::model::item_subtype::{ItemSubType, MaulSubType};
use crate::model::ItemType;
use crate::model::{Currency, WornFlag1};
use crate::model::{Item, WornFlag2};
use crate::rng::randint;

#[derive(Copy, Clone, PartialEq)]
pub enum ItemQuality {
    LowQuality,
    HighQuality,
    Special,
    Magic,
    Normal,
    Cursed,
}

pub trait ItemTemplate {
    fn create(&self, item_quality: ItemQuality, _item_level: u8) -> Item {
        default_create(self, item_quality)
    }

    fn name(&self) -> &str;
    fn item_type(&self) -> ItemType;
    fn flags1(&self) -> u64;
    fn flags2(&self) -> u64;
    fn p1(&self) -> i64;
    fn cost(&self) -> i64;
    fn subtype(&self) -> ItemSubType;
    fn weight(&self) -> u16;
    fn number(&self) -> u16;
    fn modifier_to_hit(&self) -> i16;
    fn modifier_to_damage(&self) -> i16;
    fn base_ac(&self) -> i16;
    fn modifier_to_ac(&self) -> i16;
    fn damage(&self) -> &str;
    fn item_level(&self) -> u8;
    fn is_identified(&self) -> bool;

    fn apply_random_tier3_weapon(&self, item: &mut Item) {
        match randint(5) {
            1 => self.apply_weapon_holy_avenger(item),
            2 => self.apply_weapon_defender(item),
            3 => self.apply_weapon_demon_bane(item),
            4 => self.apply_weapon_soul_sword(item),
            _ => self.apply_weapon_vorpal_sword(item),
        }
    }

    fn apply_random_tier2_weapon(&self, item: &mut Item) {
        match randint(4) {
            1 => self.apply_weapon_slay_monster(item),
            2 => self.apply_weapon_slay_dragon(item),
            3 => self.apply_weapon_slay_undead(item),
            _ => self.apply_weapon_slay_regenerative(item),
        }
    }

    fn apply_random_tier1_weapon(&self, item: &mut Item) {
        match randint(4) {
            1 => self.apply_weapon_flame_tongue(item),
            2 => self.apply_weapon_frost_brand(item),
            3 => self.apply_weapon_wizard_blade(item),
            _ => self.apply_weapon_blessed_blade(item),
        }
    }

    fn apply_weapon_of_criticals(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} of Criticals", self.name()));
        item.apply_wornflag2(WornFlag2::Sharp);
        item.tohit += 5;
        item.cost += 300_000;
    }

    fn apply_weapon_flame_tongue(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (FT)", self.name()));
        item.apply_wornflag1(WornFlag1::SlayFire);
        item.tohit += 1;
        item.todam += 3;
        item.cost += 200_000;
    }

    fn apply_weapon_frost_brand(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (FB)", self.name()));
        item.apply_wornflag1(WornFlag1::SlayCold);
        item.tohit += 1;
        item.todam += 1;
        item.cost += 120_000;
    }

    fn apply_weapon_wizard_blade(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (WB)", self.name()));
        item.apply_wornflag2(WornFlag2::MagicProof);
        item.weight = (item.weight * 4) / 5;
        //TODO item.tval = ItemSubType::Dagger() Let mages use it
        item.tohit += 3;
        item.todam += 1;
        item.cost += 80_000;
    }

    fn apply_weapon_blessed_blade(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (BB)", self.name()));
        item.apply_wornflag2(WornFlag2::MagicProof);
        //TODO item.tval = ItemSubType::Maul() Let priests use it
        item.tohit += 2;
        item.todam += 4;
        item.cost += 80_000;
    }

    fn apply_weapon_slay_monster(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (SM)", self.name()));
        item.apply_wornflag1(WornFlag1::SeeInvisible);
        item.apply_wornflag1(WornFlag1::SlayMonster);
        item.tohit += 3;
        item.todam += 3;
        item.cost += 500_000;
    }

    fn apply_weapon_slay_dragon(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (SD)", self.name()));
        item.apply_wornflag1(WornFlag1::SlayDragon);
        item.tohit += 3;
        item.todam += 3;
        item.cost += 400_000;
    }

    fn apply_weapon_slay_undead(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (SU)", self.name()));
        item.apply_wornflag1(WornFlag1::SlayUndead);
        item.tohit += 2;
        item.todam += 2;
        item.cost += 300_000;
    }

    fn apply_weapon_slay_regenerative(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (SR)", self.name()));
        item.apply_wornflag2(WornFlag2::SlayRegenerators);
        item.tohit += 2;
        item.todam += 2;
        item.cost += 150_000;
    }

    fn apply_weapon_holy_avenger(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (HA)", self.name()));
        item.apply_wornflag1(WornFlag1::SeeInvisible);
        item.apply_wornflag1(WornFlag1::ResistStatDrain);
        item.apply_wornflag1(WornFlag1::ResistAcid);
        item.apply_wornflag1(WornFlag1::ResistFire);
        item.apply_wornflag1(WornFlag1::GivesStrength);
        item.apply_wornflag1(WornFlag1::SlayUndead);
        item.apply_wornflag1(WornFlag1::SlayEvil);
        item.tohit += 5;
        item.todam += 5;
        item.toac = randint(4) as i16;
        item.p1 = randint(4) -1;
        item.cost += item.p1 * 50_000;
        item.cost += 1_000_000;
    }

    fn apply_weapon_defender(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} [%P4] (DF)", self.name()));
        item.apply_wornflag1(WornFlag1::FeatherFall);
        item.apply_wornflag1(WornFlag1::Regeneration);
        item.apply_wornflag1(WornFlag1::ResistAcid);
        item.apply_wornflag1(WornFlag1::ResistCold);
        item.apply_wornflag1(WornFlag1::ResistFire);
        item.apply_wornflag1(WornFlag1::ResistLightning);
        item.apply_wornflag1(WornFlag1::ResistParalysis);
        item.apply_wornflag1(WornFlag1::SeeInvisible);
        item.apply_wornflag1(WornFlag1::Stealth);
        item.tohit += 3;
        item.todam += 3;
        item.toac = 5 + randint(5) as i16;
        item.p1 = randint(3);
        item.cost += item.p1 * 50_000;
        item.cost += 750_000;
    }

    fn apply_weapon_demon_bane(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (DB)", self.name()));
        item.apply_wornflag1(WornFlag1::ResistFire);
        item.apply_wornflag2(WornFlag2::SlayDemon);
        item.tohit += 3;
        item.todam += 3;
        item.cost += 500_000;
    }

    fn apply_weapon_soul_sword(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (SS)", self.name()));
        item.apply_wornflag1(WornFlag1::GivesCharisma);
        item.apply_wornflag1(WornFlag1::GivesIntelligence);
        item.apply_wornflag1(WornFlag1::GivesWisdom);
        item.apply_wornflag1(WornFlag1::Regeneration);
        item.apply_wornflag1(WornFlag1::SeeInvisible);
        item.apply_wornflag2(WornFlag2::BadReputation);
        item.apply_wornflag2(WornFlag2::SoulSword);
        item.tohit += 5;
        item.todam += 10;
        item.p1 = -randint(3) -2;
        item.cost += 800_000 + item.p1 * 40_000;
    }

    fn apply_weapon_vorpal_sword(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (V)", self.name()));
        item.apply_wornflag1(WornFlag1::ResistStatDrain);
        item.apply_wornflag2(WornFlag2::Sharp);
        item.tohit += 5;
        item.todam += 5;
        item.p1 = 1;
        item.cost += 750_000;
    }

    fn apply_armor_resist(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (R)", self.name()));
        item.apply_wornflag1(WornFlag1::ResistLightning);
        item.apply_wornflag1(WornFlag1::ResistCold);
        item.apply_wornflag1(WornFlag1::ResistAcid);
        item.apply_wornflag1(WornFlag1::ResistFire);
        item.toac += 5;
        item.cost += 250_000;
    }

    fn apply_armor_resist_acid(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (RA)", self.name()));
        item.apply_wornflag1(WornFlag1::ResistAcid);
        item.cost += 100_000;
    }

    fn apply_armor_resist_fire(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (RF)", self.name()));
        item.apply_wornflag1(WornFlag1::ResistFire);
        item.cost += 60_000;
    }

    fn apply_armor_resist_cold(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (RC)", self.name()));
        item.apply_wornflag1(WornFlag1::ResistCold);
        item.cost += 60_000;
    }

    fn apply_armor_resist_lightning(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} (RL)", self.name()));
        item.apply_wornflag1(WornFlag1::ResistLightning);
        item.cost += 50_000;
    }
}

pub(crate) fn default_create(
    template: &(impl ItemTemplate + ?Sized),
    _item_quality: ItemQuality,
) -> Item {
    Item {
        name: rs2item_name(template.name()),
        tval: template.item_type().into(),
        flags: template.flags1(),
        flags2: template.flags2(),
        p1: template.p1(),
        cost: template.cost() * data::currency::value(&Currency::Gold),
        subval: item_subtype::to_usize(&template.subtype()) as i64,
        weight: template.weight(),
        number: template.number(),
        tohit: template.modifier_to_hit(),
        todam: template.modifier_to_damage(),
        ac: template.base_ac(),
        toac: template.modifier_to_ac(),
        damage: misc::rs2item_damage(template.damage()),
        level: template.item_level() as i8,
        identified: if template.is_identified() { 1 } else { 0 },
    }
}

pub(crate) fn create_melee_weapon(
    template: &(impl ItemTemplate + ?Sized),
    item_quality: ItemQuality,
) -> Item {
    let mut item = default_create(template, item_quality);
    if item_quality == ItemQuality::Cursed {
        item.set_cursed(true);
        item.cost = 0;
        item.tohit = -randint(5) as i16;
        item.todam = -randint(5) as i16;
    } else if item_quality == ItemQuality::Magic {
        item.tohit = randint(4) as i16;
        item.todam = randint(4) as i16;
    } else if item_quality == ItemQuality::Special {
        item.tohit = randint(4) as i16;
        item.todam = randint(4) as i16;

        if template.subtype() == ItemSubType::Maul(MaulSubType::WoodenClub) && randint(5) == 1 {
            MaceTemplate::apply_club_of_trollkind(&mut item);
        } else {
            match randint(100) {
                x if x < 61 => template.apply_random_tier1_weapon(&mut item),
                x if x < 81 => template.apply_random_tier2_weapon(&mut item),
                x if x < 96 => template.apply_random_tier3_weapon(&mut item),
                _ => {
                    template.apply_random_tier1_weapon(&mut item);
                    template.apply_random_tier3_weapon(&mut item);
                },
            }
        }
    }
    item
}

pub(crate) fn create_ranged_weapon(
    template: &(impl ItemTemplate + ?Sized),
    item_quality: ItemQuality,
) -> Item {
    let mut item = default_create(template, item_quality);
    if item_quality == ItemQuality::Cursed {
        item.set_cursed(true);
        item.cost = 0;
        item.tohit = -randint(5) as i16;
    } else if item_quality == ItemQuality::Magic {
        item.tohit = randint(3) as i16;
    } else if item_quality == ItemQuality::Special {
        item.tohit = randint(3) as i16;
        template.apply_weapon_of_criticals(&mut item);
    }
    item
}
