use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChimeTemplate {
    ChimeOfLight,
    ChimeOfDetectDoorsStairs,
    ChimeOfDetectTraps,
    ChimeOfTeleportation,
    ChimeOfThunderblast,
    ChimeOfSummonMonster,
    ChimeOfDisarming,
    ChimeOfAggravation,
    ChimeOfSlowMonster,
    ChimeOfSootheMonster,
    ChimeOfCureLightWound,
    ChimeOfChanging,
    ChimeOfRemoveCurse,
    ChimeOfCuring,
    ChimeOfDispelEvil,
    ChimeOfDarkness,
}

pub fn generate_chime(item_level: u8, template: ChimeTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::Chime as u8,
        flags: 0,
        flags2: template.flags2(),
        p1: 0,
        cost: template.cost(),
        subval: template.subval(),
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: item_level as i8,
        identified: 0,
    }
}

impl ChimeTemplate {
    fn name(&self) -> &str {
        match self {
            ChimeTemplate::ChimeOfLight => "& %M Chime| of Light^ (%P1 charges)",
            ChimeTemplate::ChimeOfDetectDoorsStairs => "& %M Chime| of Detect Doors/Stairs^ (%P1 charges)",
            ChimeTemplate::ChimeOfDetectTraps => "& %M Chime| of Detect Traps^ (%P1 charges)",
            ChimeTemplate::ChimeOfTeleportation => "& %M Chime| of Teleportation^ (%P1 charges)",
            ChimeTemplate::ChimeOfThunderblast => "& %M Chime| of Thunderblasts^ (%P1 charges)",
            ChimeTemplate::ChimeOfSummonMonster => "& %M Chime| of Summon Monster^ (%P1 charges)",
            ChimeTemplate::ChimeOfDisarming => "& %M Chime| of Disarming^ (%P1 charges)",
            ChimeTemplate::ChimeOfAggravation => "& %M Chime| of Aggravation^ (%P1 charges)",
            ChimeTemplate::ChimeOfSlowMonster => "& %M Chime| of Slow Monster^ (%P1 charges)",
            ChimeTemplate::ChimeOfSootheMonster => "& %M Chime| of Soothe Monster^ (%P1 charges)",
            ChimeTemplate::ChimeOfCureLightWound => "& %M Chime| of Cure Light Wound^ (%P1 charges)",
            ChimeTemplate::ChimeOfChanging => "& %M Chime| of Changing^ (%P1 charges)",
            ChimeTemplate::ChimeOfRemoveCurse => "& %M Chime| of Remove Curse^ (%P1 charges)",
            ChimeTemplate::ChimeOfCuring => "& %M Chime| of Curing^ (%P1 charges)",
            ChimeTemplate::ChimeOfDispelEvil => "& %M Chime| of Dispel Evil^ (%P1 charges)",
            ChimeTemplate::ChimeOfDarkness => "& %M Chime| of Darkness^ (%P1 charges)",
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            ChimeTemplate::ChimeOfLight => 0x00000001,
            ChimeTemplate::ChimeOfDetectDoorsStairs => 0x00000002,
            ChimeTemplate::ChimeOfDetectTraps => 0x00000004,
            ChimeTemplate::ChimeOfTeleportation => 0x00000008,
            ChimeTemplate::ChimeOfThunderblast => 0x00000010,
            ChimeTemplate::ChimeOfSummonMonster => 0x00000020,
            ChimeTemplate::ChimeOfDisarming => 0x00000040,
            ChimeTemplate::ChimeOfAggravation => 0x00000080,
            ChimeTemplate::ChimeOfSlowMonster => 0x00000100,
            ChimeTemplate::ChimeOfSootheMonster => 0x00000200,
            ChimeTemplate::ChimeOfCureLightWound => 0x00000400,
            ChimeTemplate::ChimeOfChanging => 0x00000800,
            ChimeTemplate::ChimeOfRemoveCurse => 0x00001000,
            ChimeTemplate::ChimeOfCuring => 0x00002000,
            ChimeTemplate::ChimeOfDispelEvil => 0x00004000,
            ChimeTemplate::ChimeOfDarkness => 0x00008000,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            ChimeTemplate::ChimeOfLight => 275,
            ChimeTemplate::ChimeOfDetectDoorsStairs => 375,
            ChimeTemplate::ChimeOfDetectTraps => 375,
            ChimeTemplate::ChimeOfTeleportation => 450,
            ChimeTemplate::ChimeOfThunderblast => 400,
            ChimeTemplate::ChimeOfSummonMonster => 0,
            ChimeTemplate::ChimeOfDisarming => 400,
            ChimeTemplate::ChimeOfAggravation => 0,
            ChimeTemplate::ChimeOfSlowMonster => 850,
            ChimeTemplate::ChimeOfSootheMonster => 800,
            ChimeTemplate::ChimeOfCureLightWound => 400,
            ChimeTemplate::ChimeOfChanging => 800,
            ChimeTemplate::ChimeOfRemoveCurse => 675,
            ChimeTemplate::ChimeOfCuring => 1100,
            ChimeTemplate::ChimeOfDispelEvil => 1300,
            ChimeTemplate::ChimeOfDarkness => 0,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            ChimeTemplate::ChimeOfLight => 1,
            ChimeTemplate::ChimeOfDetectDoorsStairs => 2,
            ChimeTemplate::ChimeOfDetectTraps => 3,
            ChimeTemplate::ChimeOfTeleportation => 4,
            ChimeTemplate::ChimeOfThunderblast => 5,
            ChimeTemplate::ChimeOfSummonMonster => 6,
            ChimeTemplate::ChimeOfDisarming => 7,
            ChimeTemplate::ChimeOfAggravation => 8,
            ChimeTemplate::ChimeOfSlowMonster => 9,
            ChimeTemplate::ChimeOfSootheMonster => 10,
            ChimeTemplate::ChimeOfCureLightWound => 11,
            ChimeTemplate::ChimeOfChanging => 12,
            ChimeTemplate::ChimeOfRemoveCurse => 13,
            ChimeTemplate::ChimeOfCuring => 14,
            ChimeTemplate::ChimeOfDispelEvil => 15,
            ChimeTemplate::ChimeOfDarkness => 16,
        }
    }
}
