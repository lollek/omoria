use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum GlovesTemplate {
    LeatherGloves,
    HeavyGloves,
    ClothGloves,
    ChainGloves,
    LightGauntlets,
    HeavyGauntlets,
    SharkskinGloves,
    WarGauntlets,
    DemonhideGloves,
    WyrmhideGloves,
}

impl GlovesTemplate {
    pub fn iter() -> impl Iterator<Item=GlovesTemplate> {
        [
            GlovesTemplate::LeatherGloves,
            GlovesTemplate::HeavyGloves,
            GlovesTemplate::ClothGloves,
            GlovesTemplate::ChainGloves,
            GlovesTemplate::LightGauntlets,
            GlovesTemplate::HeavyGauntlets,
            GlovesTemplate::SharkskinGloves,
            GlovesTemplate::WarGauntlets,
            GlovesTemplate::DemonhideGloves,
            GlovesTemplate::WyrmhideGloves,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::Gloves as u8,
            flags: 0,
            flags2: 0,
            p1: 0,
            cost: self.cost(),
            subval: self.subval(),
            weight: self.weight(),
            number: 1,
            tohit: 0,
            todam: 0,
            ac: self.ac(),
            toac: 0,
            damage: misc::rs2item_damage("0d0"),
            level: 0,
            identified: 0,
        }
    }

    fn name(&self) -> &str {
        match self {
            GlovesTemplate::LeatherGloves => "Leather Gloves^ [%P6,%P4]",
            GlovesTemplate::HeavyGloves => "Heavy Gloves^ [%P6,%P4]",
            GlovesTemplate::ClothGloves => "Cloth Gloves^ [%P6,%P4]",
            GlovesTemplate::ChainGloves => "Chain Gloves^ [%P6,%P4]",
            GlovesTemplate::LightGauntlets => "Light Gauntlets^ [%P6,%P4]",
            GlovesTemplate::HeavyGauntlets => "Heavy Gauntlets^ [%P6,%P4]",
            GlovesTemplate::SharkskinGloves => "Sharkskin Gloves^ [%P6,%P4]",
            GlovesTemplate::WarGauntlets => "War Gauntlets^ [%P6,%P4]",
            GlovesTemplate::DemonhideGloves => "Demonhide Gloves^ [%P6,%P4]",
            GlovesTemplate::WyrmhideGloves => "Wyrmhide Gloves^ [%P6,%P4]",
        }
    }

    fn cost(&self) -> i64 {
        match self {
            GlovesTemplate::LeatherGloves => 3,
            GlovesTemplate::HeavyGloves => 35,
            GlovesTemplate::ClothGloves => 1,
            GlovesTemplate::ChainGloves => 100,
            GlovesTemplate::LightGauntlets => 150,
            GlovesTemplate::HeavyGauntlets => 200,
            GlovesTemplate::SharkskinGloves => 300,
            GlovesTemplate::WarGauntlets => 300,
            GlovesTemplate::DemonhideGloves => 400,
            GlovesTemplate::WyrmhideGloves => 1000,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            GlovesTemplate::LeatherGloves => 1,
            GlovesTemplate::HeavyGloves => 2,
            GlovesTemplate::ClothGloves => 5,
            GlovesTemplate::ChainGloves => 6,
            GlovesTemplate::LightGauntlets => 7,
            GlovesTemplate::HeavyGauntlets => 8,
            GlovesTemplate::SharkskinGloves => 9,
            GlovesTemplate::WarGauntlets => 10,
            GlovesTemplate::DemonhideGloves => 11,
            GlovesTemplate::WyrmhideGloves => 12,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            GlovesTemplate::LeatherGloves => 5,
            GlovesTemplate::HeavyGloves => 25,
            GlovesTemplate::ClothGloves => 1,
            GlovesTemplate::ChainGloves => 30,
            GlovesTemplate::LightGauntlets => 60,
            GlovesTemplate::HeavyGauntlets => 100,
            GlovesTemplate::SharkskinGloves => 30,
            GlovesTemplate::WarGauntlets => 150,
            GlovesTemplate::DemonhideGloves => 30,
            GlovesTemplate::WyrmhideGloves => 30,
        }
    }

    fn ac(&self) -> i16 {
        match self {
            GlovesTemplate::LeatherGloves => 1,
            GlovesTemplate::HeavyGloves => 2,
            GlovesTemplate::ClothGloves => 0,
            GlovesTemplate::ChainGloves => 4,
            GlovesTemplate::LightGauntlets => 5,
            GlovesTemplate::HeavyGauntlets => 7,
            GlovesTemplate::SharkskinGloves => 6,
            GlovesTemplate::WarGauntlets => 8,
            GlovesTemplate::DemonhideGloves => 7,
            GlovesTemplate::WyrmhideGloves => 8,
        }
    }

    pub fn level(&self) -> u8 {
        match self {
            GlovesTemplate::LeatherGloves => 1,
            GlovesTemplate::HeavyGloves => 5,
            GlovesTemplate::ClothGloves => 1,
            GlovesTemplate::ChainGloves => 10,
            GlovesTemplate::LightGauntlets => 16,
            GlovesTemplate::HeavyGauntlets => 16,
            GlovesTemplate::SharkskinGloves => 30,
            GlovesTemplate::WarGauntlets => 30,
            GlovesTemplate::DemonhideGloves => 40,
            GlovesTemplate::WyrmhideGloves => 50,
        }
    }
}
