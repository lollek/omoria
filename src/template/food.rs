use misc;
use model::{ Item, ItemType };


pub fn mushroom() -> Item {
    Item {
        name: misc::rs2item_name("& Mushroom~"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0,
        p1: 3000,
        cost: 2,
        subval: 256,
        weight: 5,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 1,
        identified: 0,
    }
}

pub fn mushroom_poison() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Poison"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000001,
        p1: 500,
        cost: 0,
        subval: 257,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 7,
        identified: 0,
    }
}

pub fn mushroom_blindness() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Blindness"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000002,
        p1: 500,
        cost: 0,
        subval: 258,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 9,
        identified: 0,
    }
}

pub fn mushroom_paranoia() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Paranoia"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000004,
        p1: 500,
        cost: 0,
        subval: 259,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 9,
        identified: 0,
    }
}

pub fn mushroom_confusion() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Confusion"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000008,
        p1: 500,
        cost: 0,
        subval: 260,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 7,
        identified: 0,
    }
}

pub fn mushroom_hallucination() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Hallucination"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000010,
        p1: 500,
        cost: 0,
        subval: 261,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 13,
        identified: 0,
    }
}

pub fn mushroom_cure_poison() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Cure Poison"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000020,
        p1: 500,
        cost: 60,
        subval: 262,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 8,
        identified: 0,
    }
}

pub fn mushroom_cure_blindness() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Cure Blindness"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000040,
        p1: 500,
        cost: 50,
        subval: 263,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 10,
        identified: 0,
    }
}

pub fn mushroom_cure_paranoia() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Cure Paranoia"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000080,
        p1: 500,
        cost: 25,
        subval: 264,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 12,
        identified: 0,
    }
}

pub fn mushroom_cure_confusion() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Cure Confusion"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000100,
        p1: 500,
        cost: 50,
        subval: 265,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 6,
        identified: 0,
    }
}

pub fn mushroom_weakness() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Weakness"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000200,
        p1: 500,
        cost: 0,
        subval: 266,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 7,
        identified: 0,
    }
}

pub fn mushroom_unhealth() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Unhealth"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000400,
        p1: 500,
        cost: 50,
        subval: 267,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("10d10"),
        level: 15,
        identified: 0,
    }
}

pub fn mushroom_restore_constitution() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Restore Constitution"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00010000,
        p1: 500,
        cost: 350,
        subval: 268,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 20,
        identified: 0,
    }
}

pub fn mushroom_first_air() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of First-Aid"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00020000,
        p1: 500,
        cost: 5,
        subval: 269,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 6,
        identified: 0,
    }
}

pub fn mushroom_minor_cures() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Minor Cures"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00040000,
        p1: 500,
        cost: 20,
        subval: 270,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 7,
        identified: 0,
    }
}

pub fn mushroom_light_cures() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Light Cures"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00080000,
        p1: 500,
        cost: 30,
        subval: 271,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 10,
        identified: 0,
    }
}

pub fn mushroom_restoring() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Restoring"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x001F8040,
        p1: 500,
        cost: 1000,
        subval: 272,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 30,
        identified: 0,
    }
}

pub fn mushroom_poison2() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Poison"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000001,
        p1: 1200,
        cost: 0,
        subval: 273,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 15,
        identified: 0,
    }
}

pub fn mushroom_hallucination2() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Hallucination"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000010,
        p1: 1200,
        cost: 0,
        subval: 274,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 18,
        identified: 0,
    }
}

pub fn mushroom_cure_poison2() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Cure Poison"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000020,
        p1: 1200,
        cost: 75,
        subval: 275,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 19,
        identified: 0,
    }
}

pub fn mushroom_unhealth2() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Unhealth"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000400,
        p1: 1200,
        cost: 25,
        subval: 276,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("6d8"),
        level: 28,
        identified: 0,
    }
}

pub fn mushroom_cure_serious_wounds() -> Item {
    Item {
        name: misc::rs2item_name("& %M Mushroom~| of Cure Serious Wounds"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x01800000,
        p1: 1200,
        cost: 75,
        subval: 277,
        weight: 1,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 16,
        identified: 0,
    }
}

pub fn pint_of_fine_grade_mush() -> Item {
    Item {
        name: misc::rs2item_name("& pint~ of fine grade mush"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000000,
        p1: 1500,
        cost: 0,
        subval: 306,
        weight: 10,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: -1,
        identified: 0,
    }
}

pub fn ration_of_food() -> Item {
    Item {
        name: misc::rs2item_name("& Ration~ of Food"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0,
        p1: 5000,
        cost: 3,
        subval: 307,
        weight: 10,
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

pub fn mushroom2() -> Item {
    Item {
        name: misc::rs2item_name("& Mushroom~"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0,
        p1: 3000,
        cost: 2,
        subval: 308,
        weight: 5,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: -1,
        identified: 0,
    }
}

pub fn hard_biscuit() -> Item {
    Item {
        name: misc::rs2item_name("& Hard Biscuit~"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0,
        p1: 500,
        cost: 1,
        subval: 309,
        weight: 2,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: -1,
        identified: 0,
    }
}

pub fn beef_jerky() -> Item {
    Item {
        name: misc::rs2item_name("& Strip~ of Beef Jerky"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0,
        p1: 1750,
        cost: 2,
        subval: 310,
        weight: 2,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: -1,
        identified: 0,
    }
}

pub fn fine_ale() -> Item {
    Item {
        name: misc::rs2item_name("& Pint~ of Fine Ale"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0,
        p1: 500,
        cost: 1,
        subval: 311,
        weight: 10,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: -1,
        identified: 0,
    }
}

pub fn fine_wine() -> Item {
    Item {
        name: misc::rs2item_name("& Pint~ of Fine Wine"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0,
        p1: 400,
        cost: 2,
        subval: 312,
        weight: 10,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: -1,
        identified: 0,
    }
}

pub fn elvish_waybreak() -> Item {
    Item {
        name: misc::rs2item_name("& Piece~ of Elvish Waybread"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x21800020,
        p1: 3500,
        cost: 10,
        subval: 313,
        weight: 3,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 6,
        identified: 0,
    }
}

pub fn stew() -> Item {
    Item {
        name: misc::rs2item_name("& Stew~"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x330001C0,
        p1: 2000,
        cost: 0,
        subval: 314,
        weight: 3,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: -1,
        identified: 0,
    }
}

pub fn jolly_green_jelly() -> Item {
    Item {
        name: misc::rs2item_name("& Jolly Green Jelly~| (Ho Ho Ho!)"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x224001C0,
        p1: 4000,
        cost: 0,
        subval: 315,
        weight: 3,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 50,
        identified: 0,
    }
}

pub fn green_jelly() -> Item {
    Item {
        name: misc::rs2item_name("& Green Jelly~"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x22400060,
        p1: 4000,
        cost: 50,
        subval: 315,
        weight: 3,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: -1,
        identified: 0,
    }
}

pub fn berries_poisonous() -> Item {
    Item {
        name: misc::rs2item_name("& Handful~ of Berries| (Poisonous)"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x0C0000000,
        p1: 1000,
        cost: 0,
        subval: 316,
        weight: 3,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 8,
        identified: 0,
    }
}

pub fn berries_smurfberries() -> Item {
    Item {
        name: misc::rs2item_name("& Handful~ of Berries| (Smurfberries)"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x10400000,
        p1: 1000,
        cost: 0,
        subval: 317,
        weight: 3,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 20,
        identified: 0,
    }
}

pub fn berries_smurfberries2() -> Item {
    Item {
        name: misc::rs2item_name("& Handful~ of Berries| (Smurfberries)"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x30400000,
        p1: 1000,
        cost: 0,
        subval: 317,
        weight: 3,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: -1,
        identified: 0,
    }
}

pub fn berries_goodberries() -> Item {
    Item {
        name: misc::rs2item_name("& Handful~ of Berries| (Goodberries)"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x10C00080,
        p1: 1000,
        cost: 0,
        subval: 318,
        weight: 3,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: 40,
        identified: 0,
    }
}

pub fn berries_goodberries2() -> Item {
    Item {
        name: misc::rs2item_name("& Handful~ of Berries| (Goodberries)"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x30C00080,
        p1: 1000,
        cost: 0,
        subval: 318,
        weight: 3,
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: -1,
        identified: 0,
    }
}

pub fn eyeball_of_ned() -> Item {
    Item {
        name: misc::rs2item_name("& Eyeball~| of Ned"),
        tval: ItemType::Food as u8,
        flags: 0,
        flags2: 0x00000053,
        p1: 200,
        cost: 50,
        subval: 319,
        weight: 2,
        number: 2,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("6d5"),
        level: 20,
        identified: 0,
    }
}
