use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum HornTemplate {
    HornOfBubbles,
    HornOfCalling,
    HornOfSoftSounds,
    HornOfBlasting,
    HornOfCold,
    HornOfHeat,
    HornOfGas,
    HornOfRecall,
    HornOfChaos,
    HornOfGlue,
    HornOfValhalla,
    HornOfTritons,
    HornOfFog,
}

impl HornTemplate {
    pub fn iter() -> impl Iterator<Item=HornTemplate> {
        [
            HornTemplate::HornOfBubbles,
            HornTemplate::HornOfCalling,
            HornTemplate::HornOfSoftSounds,
            HornTemplate::HornOfBlasting,
            HornTemplate::HornOfCold,
            HornTemplate::HornOfHeat,
            HornTemplate::HornOfGas,
            HornTemplate::HornOfRecall,
            HornTemplate::HornOfChaos,
            HornTemplate::HornOfGlue,
            HornTemplate::HornOfValhalla,
            HornTemplate::HornOfTritons,
            HornTemplate::HornOfFog,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::Horn as u8,
            flags: 0,
            flags2: self.flags2(),
            p1: 0,
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subval(),
            weight: 1,
            number: 1,
            tohit: 0,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage("0d0"),
            level: 0,
            identified: 0,
        }
    }

    fn name(&self) -> &str {
        match self {
            HornTemplate::HornOfBubbles => "& %H| of Bubbles^ (%P1 charges)",
            HornTemplate::HornOfCalling => "& %H| of Calling^ (%P1 charges)",
            HornTemplate::HornOfSoftSounds => "& %H| of Soft Sounds^ (%P1 charges)",
            HornTemplate::HornOfBlasting => "& %H| of *Blasting*^ (%P1 charges)",
            HornTemplate::HornOfCold => "& %H| of Cold^ (%P1 charges)",
            HornTemplate::HornOfHeat => "& %H| of Heat^ (%P1 charges)",
            HornTemplate::HornOfGas => "& %H| of Gas^ (%P1 charges)",
            HornTemplate::HornOfRecall => "& %H| of Recall^ (%P1 charges)",
            HornTemplate::HornOfChaos => "& %H| of *Chaos*^ (%P1 charges)",
            HornTemplate::HornOfGlue => "& %H| of Glue^ (%P1 charges)",
            HornTemplate::HornOfValhalla => "& %H| of Valhalla^ (%P1 charges)",
            HornTemplate::HornOfTritons => "& %H| of Tritons^ (%P1 charges)",
            HornTemplate::HornOfFog => "& %H| of Fog^ (%P1 charges)",
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            HornTemplate::HornOfBubbles => 0x00010000,
            HornTemplate::HornOfCalling => 0x00020000,
            HornTemplate::HornOfSoftSounds => 0x00040000,
            HornTemplate::HornOfBlasting => 0x00080000,
            HornTemplate::HornOfCold => 0x00100000,
            HornTemplate::HornOfHeat => 0x00200000,
            HornTemplate::HornOfGas => 0x00400000,
            HornTemplate::HornOfRecall => 0x00800000,
            HornTemplate::HornOfChaos => 0x01000000,
            HornTemplate::HornOfGlue => 0x02000000,
            HornTemplate::HornOfValhalla => 0x04000000,
            HornTemplate::HornOfTritons => 0x08000000,
            HornTemplate::HornOfFog => 0x10000000,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            HornTemplate::HornOfBubbles => 0,
            HornTemplate::HornOfCalling => 0,
            HornTemplate::HornOfSoftSounds => 600,
            HornTemplate::HornOfBlasting => 2600,
            HornTemplate::HornOfCold => 1000,
            HornTemplate::HornOfHeat => 1000,
            HornTemplate::HornOfGas => 900,
            HornTemplate::HornOfRecall => 1200,
            HornTemplate::HornOfChaos => 800,
            HornTemplate::HornOfGlue => 0,
            HornTemplate::HornOfValhalla => 2700,
            HornTemplate::HornOfTritons => 200,
            HornTemplate::HornOfFog => 500,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            HornTemplate::HornOfBubbles => 1,
            HornTemplate::HornOfCalling => 2,
            HornTemplate::HornOfSoftSounds => 3,
            HornTemplate::HornOfBlasting => 4,
            HornTemplate::HornOfCold => 5,
            HornTemplate::HornOfHeat => 6,
            HornTemplate::HornOfGas => 7,
            HornTemplate::HornOfRecall => 8,
            HornTemplate::HornOfChaos => 9,
            HornTemplate::HornOfGlue => 10,
            HornTemplate::HornOfValhalla => 11,
            HornTemplate::HornOfTritons => 12,
            HornTemplate::HornOfFog => 13,
        }
    }

    fn level(&self) -> u8 {
        match self {
            HornTemplate::HornOfBubbles => 15,
            HornTemplate::HornOfCalling => 10,
            HornTemplate::HornOfSoftSounds => 8,
            HornTemplate::HornOfBlasting => 49,
            HornTemplate::HornOfCold => 40,
            HornTemplate::HornOfHeat => 40,
            HornTemplate::HornOfGas => 35,
            HornTemplate::HornOfRecall => 30,
            HornTemplate::HornOfChaos => 43,
            HornTemplate::HornOfGlue => 20,
            HornTemplate::HornOfValhalla => 50,
            HornTemplate::HornOfTritons => 15,
            HornTemplate::HornOfFog => 25,
        }
    }
}