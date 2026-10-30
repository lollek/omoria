#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HelpLine {
    pub row: libc::c_int,
    pub text: &'static str,
}

const COMMAND_HELP: &[HelpLine] = &[
    HelpLine {
        row: 1,
        text: " Movement keys: hjklyubn. Dot to wait one turn. ~ means a direction. Hold shift to run   ",
    },
    HelpLine {
        row: 4,
        text: " a     Aim projectile         | A     Aim (throw) an item    |^A     Special abilities      |M-A    Armor (manual)         ",
    },
    HelpLine {
        row: 5,
        text: " c ~   Close a door           | C     Display character      |^C     Quit the game          |M-C    Class restrictions     ",
    },
    HelpLine {
        row: 6,
        text: " d     Drop an item           | D ~   Disarm a trap/chest    |^D     Show game date         |M-D    (Nothing)              ",
    },
    HelpLine {
        row: 7,
        text: " e     (Nothing)              | E     Eat some food          |^E     (Nothing               |M-E    Experience (manual)    ",
    },
    HelpLine {
        row: 8,
        text: " f ~   Force / bash           | F     Fill lamp with oil     |^F     (Nothing               |M-F    (Nothing)              ",
    },
    HelpLine {
        row: 9,
        text: " g     Pick up an item        | G     (Nothing)              |^G     (Nothing)              |M-G    (Nothing)              ",
    },
    HelpLine {
        row: 10,
        text: " i     Inventory list         | I     Inven of one item type |^I     Use instrument         |M-I    Inventory list (old)   ",
    },
    HelpLine {
        row: 11,
        text: " m     Magic / Music / Monk   | M     (Nothing)              |^M     (Nothing)              |M-M    Show manual            ",
    },
    HelpLine {
        row: 12,
        text: " o ~   Open a door or chest   | O     (Nothing)              |^O     (Nothing               |M-O    (Nothing)              ",
    },
    HelpLine {
        row: 13,
        text: " p     Prayer / Play          | P     Print spell list       |^P     Print last messages    |M-P    (Nothing)              ",
    },
    HelpLine {
        row: 14,
        text: " q     Quaff a potion         | Q     See current quest      |^Q     (Nothing               |M-Q    (Nothing)              ",
    },
    HelpLine {
        row: 15,
        text: " r     Read a scroll          | R     Rest for a while       |^R     Redraw the screen      |M-R    Toggle terse mode      ",
    },
    HelpLine {
        row: 16,
        text: " s     Search for hidden doors| S     Search mode            |^S ~   Spike a door           |M-S    Save and quit          ",
    },
    HelpLine {
        row: 17,
        text: " t     Take off an item       | T ~   Tunnel                 |^T     (Nothing)              |M-T    Show real world time   ",
    },
    HelpLine {
        row: 18,
        text: " v     Version and credits    | V     (Nothing)              |^V     (Nothing               |M-V    Toggle combat calcs    ",
    },
    HelpLine {
        row: 19,
        text: " w     Wear/Wield an item     | W     (Nothing)              |^W     (Nothing)              |M-W    Weapons (manual)       ",
    },
    HelpLine {
        row: 20,
        text: " x     Exchange weapon        | X     Toggle light source    |^X     Examine a direction    |M-X    (Nothing)              ",
    },
    HelpLine {
        row: 21,
        text: " z     Zap a wand             | Z     Zap a staff            |^Z     (Nothing               |M-Z    (Nothing)              ",
    },
    HelpLine {
        row: 22,
        text: " <     Go up an up-staircase  | >     Go down a down-staircase",
    },
    HelpLine {
        row: 23,
        text: " /     Identify a character   | ?     Display this panel      ",
    },
];

const SHOP_HELP: &[HelpLine] = &[
    HelpLine {
        row: 1,
        text: "Directory of Shops:                                ",
    },
    HelpLine {
        row: 3,
        text: "     A   Alchemy Shop                M   Magic Shop",
    },
    HelpLine {
        row: 4,
        text: "     B   Bank                        P   Trading Post",
    },
    HelpLine {
        row: 5,
        text: "     C   Casino                      Q   Questor's Home",
    },
    HelpLine {
        row: 6,
        text: "     D   All-Nite Deli               R   Armory",
    },
    HelpLine {
        row: 7,
        text: "     G   General Store               T   Temple",
    },
    HelpLine {
        row: 8,
        text: "     I   Inn                         U   Music Shop",
    },
    HelpLine {
        row: 9,
        text: "     J   Gem Shop                    W   Weapon Smith",
    },
    HelpLine {
        row: 10,
        text: "     L   Library                     X   Money Exchange",
    },
    HelpLine {
        row: 11,
        text: "     +   Unknown",
    },
];

const TOWN_IDENTIFICATIONS: &[(u8, &str)] = &[
    (b'#', "# - A stone wall."),
    (b'+', "+ - Entrance to a building."),
    (b'.', ". - Ground."),
    (b'>', "> - A down staircase."),
    (b'p', "p - A townsperson."),
    (b'A', "A - Alchemy Shop."),
    (b'B', "B - First Moria National Bank."),
    (b'C', "C - Casino."),
    (b'D', "D - All-Nite Deli"),
    (b'G', "G - General Store."),
    (b'I', "I - Inn."),
    (b'J', "J - Gem and Jewelry Store."),
    (b'L', "L - Library."),
    (b'M', "M - Magic Shop."),
    (b'N', "N - Insurance Shop."),
    (b'P', "P - Trading Post."),
    (b'Q', "Q - Home of the Questor."),
    (b'R', "R - Armory."),
    (b'T', "T - Temple."),
    (b'U', "U - Music Shop."),
    (b'W', "W - Weapon Smith."),
    (b'X', "X - Money Exchange."),
];

const DUNGEON_IDENTIFICATIONS: &[(u8, &str)] = &[
    (b' ', "  - An open pit."),
    (b'!', "! - A potion."),
    (b'"', "\" - An amulet, periapt, or necklace."),
    (b'#', "# - A stone wall."),
    (b'$', "$ - Treasure."),
    (b'%', "% - A musical instrument or song book."),
    (b'&', "& - Treasure chest."),
    (b'\'', "' - An open door."),
    (b'(', "( - Soft armor."),
    (b')', ") - A shield."),
    (b'*', "* - Gems or Jewelry."),
    (b'+', "+ - A closed door."),
    (b',', ", - Food or mushroom patch."),
    (b'-', "- - A wand"),
    (b'.', ". - Floor."),
    (b'/', "/ - A pole weapon."),
    (b':', ": - Rubble."),
    (b';', "; - A loose rock."),
    (b'<', "< - An up staircase."),
    (b'=', "= - A ring."),
    (b'>', "> - A down staircase."),
    (b'?', "? - A scroll."),
    (b'A', "A - Giant Ant Lion."),
    (b'B', "B - Demon."),
    (b'C', "C - Gelentanious Cube."),
    (b'D', "D - An Ancient Dragon (Beware)."),
    (b'E', "E - Elemental."),
    (b'F', "F - Giant Fly or Faerie Dragon."),
    (b'G', "G - Ghost."),
    (b'H', "H - Hobgoblin."),
    (b'J', "J - Jelly."),
    (b'K', "K - Killer Beetle."),
    (b'L', "L - Lich."),
    (b'M', "M - Mummy."),
    (b'N', "N - Nymph"),
    (b'O', "O - Ooze."),
    (b'P', "P - Giant humanoid."),
    (b'Q', "Q - Quylthulg (Pulsing Flesh Mound)."),
    (b'R', "R - Reptile."),
    (b'S', "S - Giant Scorpion/Sandgorgon."),
    (b'T', "T - Troll."),
    (b'U', "U - Umber Hulk."),
    (b'V', "V - Vampire."),
    (b'W', "W - Wight or Wraith."),
    (b'X', "X - Xorn."),
    (b'Y', "Y - Yeti."),
    (b'Z', "Z - Nazgul"),
    (b'[', "[ - Hard armor."),
    (b'\\', "\\ - A hafted weapon."),
    (b']', "] - Misc. armor."),
    (b'^', "^ - A trap."),
    (b'_', "_ - A staff."),
    (b'`', "` - Water."),
    (b'a', "a - Amphibian."),
    (b'b', "b - Giant Bat."),
    (b'c', "c - Insect."),
    (b'd', "d - Dragon."),
    (b'e', "e - Floating Eye."),
    (b'f', "f - Fish."),
    (b'g', "g - Golem."),
    (b'h', "h - Harpy."),
    (b'i', "i - Icky Thing."),
    (b'j', "j - Canine."),
    (b'k', "k - Kobold."),
    (b'l', "l - Giant Lice."),
    (b'm', "m - Mold."),
    (b'n', "n - Naga."),
    (b'o', "o - Orc or Ogre."),
    (b'p', "p - Person (Humanoid)."),
    (b'q', "q - Quasit."),
    (b'r', "r - Rodent."),
    (b's', "s - Skeleton."),
    (b't', "t - Giant tick."),
    (b'v', "v - Swirling Vapor."),
    (b'w', "w - Worm(s)."),
    (b'x', "x - Spider."),
    (b'y', "y - Yeek."),
    (b'z', "z - Zombie."),
    (b'{', "{ - Arrow, bolt, or bullet."),
    (b'|', "| - A sword or dagger."),
    (b'}', "} - Bow, crossbow, or sling."),
    (b'~', "~ - Miscellaneous item."),
];

const WATER_IDENTIFICATIONS: &[(u8, &str)] = &[
    (b'a', "a  - Amphibian."),
    (b'f', "f  - Fish."),
    (b'l', "l  - Leach."),
    (b'm', "m  - Man-o-War."),
    (b'w', "w  - Kelp."),
    (b'C', "C  - Alligator or Crocodile."),
    (b'D', "D  - Dinosaur."),
    (b'H', "H  - Hippo."),
    (b'R', "R  - Remora."),
    (b'S', "S  - Shark."),
];

pub mod interop;

pub fn help_page_lines(page: usize) -> Option<&'static [HelpLine]> {
    match page {
        0 => Some(COMMAND_HELP),
        1 => Some(SHOP_HELP),
        _ => None,
    }
}

pub fn ident_char_description(environment: u8, character: u8) -> Option<&'static str> {
    if character == b'@' {
        return None;
    }

    let (entries, fallback) = match environment {
        b't' => (TOWN_IDENTIFICATIONS, "Not normally used."),
        b'd' => (DUNGEON_IDENTIFICATIONS, "Not Used."),
        b'w' => (WATER_IDENTIFICATIONS, "Not Used."),
        _ => return None,
    };

    Some(
        entries
            .iter()
            .find(|(key, _)| *key == character)
            .map(|(_, description)| *description)
            .unwrap_or(fallback),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_pages_match_legacy_text_and_rows() {
        let commands = [
            (
                1,
                " Movement keys: hjklyubn. Dot to wait one turn. ~ means a direction. Hold shift to run   ",
            ),
            (
                4,
                " a     Aim projectile         | A     Aim (throw) an item    |^A     Special abilities      |M-A    Armor (manual)         ",
            ),
            (
                5,
                " c ~   Close a door           | C     Display character      |^C     Quit the game          |M-C    Class restrictions     ",
            ),
            (
                6,
                " d     Drop an item           | D ~   Disarm a trap/chest    |^D     Show game date         |M-D    (Nothing)              ",
            ),
            (
                7,
                " e     (Nothing)              | E     Eat some food          |^E     (Nothing               |M-E    Experience (manual)    ",
            ),
            (
                8,
                " f ~   Force / bash           | F     Fill lamp with oil     |^F     (Nothing               |M-F    (Nothing)              ",
            ),
            (
                9,
                " g     Pick up an item        | G     (Nothing)              |^G     (Nothing)              |M-G    (Nothing)              ",
            ),
            (
                10,
                " i     Inventory list         | I     Inven of one item type |^I     Use instrument         |M-I    Inventory list (old)   ",
            ),
            (
                11,
                " m     Magic / Music / Monk   | M     (Nothing)              |^M     (Nothing)              |M-M    Show manual            ",
            ),
            (
                12,
                " o ~   Open a door or chest   | O     (Nothing)              |^O     (Nothing               |M-O    (Nothing)              ",
            ),
            (
                13,
                " p     Prayer / Play          | P     Print spell list       |^P     Print last messages    |M-P    (Nothing)              ",
            ),
            (
                14,
                " q     Quaff a potion         | Q     See current quest      |^Q     (Nothing               |M-Q    (Nothing)              ",
            ),
            (
                15,
                " r     Read a scroll          | R     Rest for a while       |^R     Redraw the screen      |M-R    Toggle terse mode      ",
            ),
            (
                16,
                " s     Search for hidden doors| S     Search mode            |^S ~   Spike a door           |M-S    Save and quit          ",
            ),
            (
                17,
                " t     Take off an item       | T ~   Tunnel                 |^T     (Nothing)              |M-T    Show real world time   ",
            ),
            (
                18,
                " v     Version and credits    | V     (Nothing)              |^V     (Nothing               |M-V    Toggle combat calcs    ",
            ),
            (
                19,
                " w     Wear/Wield an item     | W     (Nothing)              |^W     (Nothing)              |M-W    Weapons (manual)       ",
            ),
            (
                20,
                " x     Exchange weapon        | X     Toggle light source    |^X     Examine a direction    |M-X    (Nothing)              ",
            ),
            (
                21,
                " z     Zap a wand             | Z     Zap a staff            |^Z     (Nothing               |M-Z    (Nothing)              ",
            ),
            (22, " <     Go up an up-staircase  | >     Go down a down-staircase"),
            (23, " /     Identify a character   | ?     Display this panel      "),
        ];
        let shops = [
            (1, "Directory of Shops:                                "),
            (3, "     A   Alchemy Shop                M   Magic Shop"),
            (4, "     B   Bank                        P   Trading Post"),
            (5, "     C   Casino                      Q   Questor's Home"),
            (6, "     D   All-Nite Deli               R   Armory"),
            (7, "     G   General Store               T   Temple"),
            (8, "     I   Inn                         U   Music Shop"),
            (9, "     J   Gem Shop                    W   Weapon Smith"),
            (
                10,
                "     L   Library                     X   Money Exchange",
            ),
            (11, "     +   Unknown"),
        ];

        assert_eq!(
            help_page_lines(0)
                .expect("command help page")
                .iter()
                .map(|line| (line.row, line.text))
                .collect::<Vec<_>>(),
            commands
        );
        assert_eq!(
            help_page_lines(1)
                .expect("shop help page")
                .iter()
                .map(|line| (line.row, line.text))
                .collect::<Vec<_>>(),
            shops
        );
        assert!(help_page_lines(2).is_none());
    }

    #[test]
    fn ident_char_lookup_matches_legacy_descriptions() {
        let cases = [
            (b't', b'#', Some("# - A stone wall.")),
            (b't', b'+', Some("+ - Entrance to a building.")),
            (b't', b'.', Some(". - Ground.")),
            (b't', b'>', Some("> - A down staircase.")),
            (b't', b'p', Some("p - A townsperson.")),
            (b't', b'A', Some("A - Alchemy Shop.")),
            (b't', b'B', Some("B - First Moria National Bank.")),
            (b't', b'C', Some("C - Casino.")),
            (b't', b'D', Some("D - All-Nite Deli")),
            (b't', b'G', Some("G - General Store.")),
            (b't', b'I', Some("I - Inn.")),
            (b't', b'J', Some("J - Gem and Jewelry Store.")),
            (b't', b'L', Some("L - Library.")),
            (b't', b'M', Some("M - Magic Shop.")),
            (b't', b'N', Some("N - Insurance Shop.")),
            (b't', b'P', Some("P - Trading Post.")),
            (b't', b'Q', Some("Q - Home of the Questor.")),
            (b't', b'R', Some("R - Armory.")),
            (b't', b'T', Some("T - Temple.")),
            (b't', b'U', Some("U - Music Shop.")),
            (b't', b'W', Some("W - Weapon Smith.")),
            (b't', b'X', Some("X - Money Exchange.")),
            (b'd', b' ', Some("  - An open pit.")),
            (b'd', b'!', Some("! - A potion.")),
            (b'd', b'"', Some("\" - An amulet, periapt, or necklace.")),
            (b'd', b'#', Some("# - A stone wall.")),
            (b'd', b'$', Some("$ - Treasure.")),
            (b'd', b'%', Some("% - A musical instrument or song book.")),
            (b'd', b'&', Some("& - Treasure chest.")),
            (b'd', b'\'', Some("' - An open door.")),
            (b'd', b'(', Some("( - Soft armor.")),
            (b'd', b')', Some(") - A shield.")),
            (b'd', b'*', Some("* - Gems or Jewelry.")),
            (b'd', b'+', Some("+ - A closed door.")),
            (b'd', b',', Some(", - Food or mushroom patch.")),
            (b'd', b'-', Some("- - A wand")),
            (b'd', b'.', Some(". - Floor.")),
            (b'd', b'/', Some("/ - A pole weapon.")),
            (b'd', b':', Some(": - Rubble.")),
            (b'd', b';', Some("; - A loose rock.")),
            (b'd', b'<', Some("< - An up staircase.")),
            (b'd', b'=', Some("= - A ring.")),
            (b'd', b'>', Some("> - A down staircase.")),
            (b'd', b'?', Some("? - A scroll.")),
            (b'd', b'A', Some("A - Giant Ant Lion.")),
            (b'd', b'B', Some("B - Demon.")),
            (b'd', b'C', Some("C - Gelentanious Cube.")),
            (b'd', b'D', Some("D - An Ancient Dragon (Beware).")),
            (b'd', b'E', Some("E - Elemental.")),
            (b'd', b'F', Some("F - Giant Fly or Faerie Dragon.")),
            (b'd', b'G', Some("G - Ghost.")),
            (b'd', b'H', Some("H - Hobgoblin.")),
            (b'd', b'J', Some("J - Jelly.")),
            (b'd', b'K', Some("K - Killer Beetle.")),
            (b'd', b'L', Some("L - Lich.")),
            (b'd', b'M', Some("M - Mummy.")),
            (b'd', b'N', Some("N - Nymph")),
            (b'd', b'O', Some("O - Ooze.")),
            (b'd', b'P', Some("P - Giant humanoid.")),
            (b'd', b'Q', Some("Q - Quylthulg (Pulsing Flesh Mound).")),
            (b'd', b'R', Some("R - Reptile.")),
            (b'd', b'S', Some("S - Giant Scorpion/Sandgorgon.")),
            (b'd', b'T', Some("T - Troll.")),
            (b'd', b'U', Some("U - Umber Hulk.")),
            (b'd', b'V', Some("V - Vampire.")),
            (b'd', b'W', Some("W - Wight or Wraith.")),
            (b'd', b'X', Some("X - Xorn.")),
            (b'd', b'Y', Some("Y - Yeti.")),
            (b'd', b'Z', Some("Z - Nazgul")),
            (b'd', b'[', Some("[ - Hard armor.")),
            (b'd', b'\\', Some("\\ - A hafted weapon.")),
            (b'd', b']', Some("] - Misc. armor.")),
            (b'd', b'^', Some("^ - A trap.")),
            (b'd', b'_', Some("_ - A staff.")),
            (b'd', b'`', Some("` - Water.")),
            (b'd', b'a', Some("a - Amphibian.")),
            (b'd', b'b', Some("b - Giant Bat.")),
            (b'd', b'c', Some("c - Insect.")),
            (b'd', b'd', Some("d - Dragon.")),
            (b'd', b'e', Some("e - Floating Eye.")),
            (b'd', b'f', Some("f - Fish.")),
            (b'd', b'g', Some("g - Golem.")),
            (b'd', b'h', Some("h - Harpy.")),
            (b'd', b'i', Some("i - Icky Thing.")),
            (b'd', b'j', Some("j - Canine.")),
            (b'd', b'k', Some("k - Kobold.")),
            (b'd', b'l', Some("l - Giant Lice.")),
            (b'd', b'm', Some("m - Mold.")),
            (b'd', b'n', Some("n - Naga.")),
            (b'd', b'o', Some("o - Orc or Ogre.")),
            (b'd', b'p', Some("p - Person (Humanoid).")),
            (b'd', b'q', Some("q - Quasit.")),
            (b'd', b'r', Some("r - Rodent.")),
            (b'd', b's', Some("s - Skeleton.")),
            (b'd', b't', Some("t - Giant tick.")),
            (b'd', b'v', Some("v - Swirling Vapor.")),
            (b'd', b'w', Some("w - Worm(s).")),
            (b'd', b'x', Some("x - Spider.")),
            (b'd', b'y', Some("y - Yeek.")),
            (b'd', b'z', Some("z - Zombie.")),
            (b'd', b'{', Some("{ - Arrow, bolt, or bullet.")),
            (b'd', b'|', Some("| - A sword or dagger.")),
            (b'd', b'}', Some("} - Bow, crossbow, or sling.")),
            (b'd', b'~', Some("~ - Miscellaneous item.")),
            (b'w', b'a', Some("a  - Amphibian.")),
            (b'w', b'f', Some("f  - Fish.")),
            (b'w', b'l', Some("l  - Leach.")),
            (b'w', b'm', Some("m  - Man-o-War.")),
            (b'w', b'w', Some("w  - Kelp.")),
            (b'w', b'C', Some("C  - Alligator or Crocodile.")),
            (b'w', b'D', Some("D  - Dinosaur.")),
            (b'w', b'H', Some("H  - Hippo.")),
            (b'w', b'R', Some("R  - Remora.")),
            (b'w', b'S', Some("S  - Shark.")),
        ];

        for (environment, character, expected) in cases {
            assert_eq!(
                ident_char_description(environment, character),
                expected,
                "environment {:?}, character {:?}",
                environment as char,
                character as char
            );
        }
    }

    #[test]
    fn ident_char_lookup_preserves_dynamic_and_default_responses() {
        assert_eq!(
            ident_char_description(b't', b'@'),
            None,
            "the player marker is rendered from the current character name in C"
        );
        assert_eq!(
            ident_char_description(b't', b'?'),
            Some("Not normally used.")
        );
        assert_eq!(ident_char_description(b'd', b'@'), None);
        assert_eq!(ident_char_description(b'd', b'\0'), Some("Not Used."));
        assert_eq!(ident_char_description(b'w', b'@'), None);
        assert_eq!(ident_char_description(b'w', b'\0'), Some("Not Used."));
        assert_eq!(ident_char_description(b'x', b'?'), None);
    }
}
