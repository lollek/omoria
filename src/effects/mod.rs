use crate::equipment::{Slot, EQUIP_MAX};
use crate::model::{Item, WornFlag1};

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

#[derive(Debug, PartialEq)]
enum ArmorDamage {
    Resisted { equipment_slot: usize },
    Damaged { equipment_slot: usize },
    Ineffective { equipment_slot: usize },
    Absent,
}

#[cfg(test)]
fn minus_ac_with_rng(
    equipment: &mut [Item; EQUIP_MAX],
    resistance_flag: WornFlag1,
    rng: &mut impl rand::Rng,
) -> ArmorDamage {
    minus_ac_with_roll(equipment, resistance_flag, |count| {
        crate::rng::randint_with_rng(rng, count)
    })
}

fn minus_ac_with_roll(
    equipment: &mut [Item; EQUIP_MAX],
    resistance_flag: WornFlag1,
    mut roll: impl FnMut(i64) -> i64,
) -> ArmorDamage {
    let slots = [
        Slot::Armor,
        Slot::Shield,
        Slot::Cloak,
        Slot::Gloves,
        Slot::Helm,
        Slot::Boots,
        Slot::Belt,
        Slot::Bracers,
    ];
    let equipped_armor_slots: Vec<usize> = slots
        .iter()
        .map(|slot| *slot as usize)
        .filter(|slot| equipment[*slot].tval > 0)
        .collect();
    if equipped_armor_slots.is_empty() {
        return ArmorDamage::Absent;
    }
    let equipment_slot =
        equipped_armor_slots[(roll(equipped_armor_slots.len() as i64) - 1) as usize];
    let item = &mut equipment[equipment_slot];
    if item.has_wornflag1(resistance_flag) {
        ArmorDamage::Resisted { equipment_slot }
    } else if i32::from(item.ac) + i32::from(item.toac) > 0 {
        item.toac -= 1;
        ArmorDamage::Damaged { equipment_slot }
    } else {
        ArmorDamage::Ineffective { equipment_slot }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::randint_with_rng;
    use rand::{SeedableRng, StdRng};

    #[test]
    fn seeded_selection_matches_c_slot_order_and_lowers_bonus() {
        let slots = [
            Slot::Armor,
            Slot::Shield,
            Slot::Cloak,
            Slot::Gloves,
            Slot::Helm,
            Slot::Boots,
            Slot::Belt,
            Slot::Bracers,
        ];
        for seed in 1..=32 {
            let mut equipment = [Item::default(); EQUIP_MAX];
            for slot in &slots {
                equipment[*slot as usize].tval = 1;
                equipment[*slot as usize].ac = 5;
                equipment[*slot as usize].toac = 2;
            }
            let mut reference = StdRng::from_seed(&[seed][..]);
            let expected = slots[(randint_with_rng(&mut reference, 8) - 1) as usize] as usize;
            let mut rng = StdRng::from_seed(&[seed][..]);
            assert_eq!(
                minus_ac_with_rng(&mut equipment, WornFlag1::ResistAcid, &mut rng),
                ArmorDamage::Damaged {
                    equipment_slot: expected,
                }
            );
            for slot in &slots {
                assert_eq!(
                    equipment[*slot as usize].toac,
                    if *slot as usize == expected { 1 } else { 2 }
                );
            }
        }
    }

    #[test]
    fn resistant_armor_absorbs_damage_without_losing_bonus() {
        let mut equipment = [Item::default(); EQUIP_MAX];
        let slot = Slot::Armor as usize;
        equipment[slot].tval = 1;
        equipment[slot].flags = WornFlag1::ResistAcid as u64;
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        assert_eq!(
            minus_ac_with_rng(&mut equipment, WornFlag1::ResistAcid, &mut rng),
            ArmorDamage::Resisted {
                equipment_slot: slot,
            }
        );
        assert_eq!(equipment[slot].toac, 0);
    }

    #[test]
    fn nonpositive_armor_does_not_absorb_damage() {
        let mut equipment = [Item::default(); EQUIP_MAX];
        let slot = Slot::Boots as usize;
        equipment[slot].tval = 1;
        equipment[slot].ac = 2;
        equipment[slot].toac = -2;
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        assert_eq!(
            minus_ac_with_rng(&mut equipment, WornFlag1::ResistAcid, &mut rng),
            ArmorDamage::Ineffective {
                equipment_slot: slot,
            }
        );
        assert_eq!(equipment[slot].toac, -2);
    }

    #[test]
    fn absent_armor_does_not_consume_rng() {
        let mut equipment = [Item::default(); EQUIP_MAX];
        equipment[Slot::Primary as usize].tval = 1;
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut reference = StdRng::from_seed(&[1, 2, 3, 4][..]);
        assert_eq!(
            minus_ac_with_rng(&mut equipment, WornFlag1::ResistAcid, &mut rng),
            ArmorDamage::Absent
        );
        assert_eq!(
            randint_with_rng(&mut rng, 100),
            randint_with_rng(&mut reference, 100)
        );
    }

    #[test]
    fn armor_sum_uses_c_integer_promotion() {
        let mut equipment = [Item::default(); EQUIP_MAX];
        let slot = Slot::Armor as usize;
        equipment[slot].tval = 1;
        equipment[slot].ac = i16::MAX;
        equipment[slot].toac = i16::MAX;
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        assert_eq!(
            minus_ac_with_rng(&mut equipment, WornFlag1::ResistAcid, &mut rng),
            ArmorDamage::Damaged {
                equipment_slot: slot,
            }
        );
        assert_eq!(equipment[slot].toac, i16::MAX - 1);
    }
}
