use pancurses;
use model;
use item_template;

fn misc_object_symbol(subtype: item_template::MiscTemplate) -> pancurses::chtype {
        match subtype {
            item_template::MiscTemplate::RatSkeleton |
            item_template::MiscTemplate::GiantCentipedeSkeleton => 's',
            item_template::MiscTemplate::EmptyBottle => '!',
            item_template::MiscTemplate::PotteryShard => '(',
            item_template::MiscTemplate::HumanSkeleton |
            item_template::MiscTemplate::DwarfSkeleton |
            item_template::MiscTemplate::ElfSkeleton |
            item_template::MiscTemplate::GnomeSkeleton => 's',
            item_template::MiscTemplate::BrokenTeeth |
            item_template::MiscTemplate::LargeBrokenBone |
            item_template::MiscTemplate::BrokenStick => '~',
        }.into()
}

fn misc_symbol(subtype: item_template::MiscUsableTemplate) -> pancurses::chtype {
    match subtype {
        item_template::MiscUsableTemplate::FlaskOfOil => '!',
        item_template::MiscUsableTemplate::IronSpike => '~',
        item_template::MiscUsableTemplate::Statue => '~',
        item_template::MiscUsableTemplate::Teeth => 's',
        item_template::MiscUsableTemplate::SilverCross |
        item_template::MiscUsableTemplate::GoldCross |
        item_template::MiscUsableTemplate::MithrilCross |
        item_template::MiscUsableTemplate::Cross => '~',
        item_template::MiscUsableTemplate::CorkedBottle => '!'
    }.into()
}

pub fn generate(item: &model::Item, item_type: model::ItemType) -> pancurses::chtype {
    match item_type {
            model::ItemType::MiscObject(subtype) => misc_object_symbol(subtype),
            model::ItemType::Chest(_) => 's'.into(),
            model::ItemType::MiscUsable(subtype) => misc_symbol(subtype),
            model::ItemType::Spike(_) => '~'.into(),
            model::ItemType::FlaskOfOil(_) => '!'.into(),
            model::ItemType::Jewelry(_) => '*'.into(),
            model::ItemType::Bag(_) => '~'.into(),
            model::ItemType::Gem(_) => '*'.into(),
            model::ItemType::WearableGem(_) => '*'.into(),
            model::ItemType::SlingAmmo(_) |
            model::ItemType::Bolt(_) |
            model::ItemType::Arrow(_) => '{'.into(),
            model::ItemType::LightSource(_) => '~'.into(),
            model::ItemType::Bow(_) |
            model::ItemType::Crossbow(_) |
            model::ItemType::Sling(_) => '}'.into(),
            model::ItemType::Axe(_) => '\\'.into(),
            model::ItemType::Polearm(_) => '/'.into(),
            model::ItemType::Dagger(_) => '|'.into(),
            model::ItemType::Sword(_) => '|'.into(),
            model::ItemType::Pick(_) => '\\'.into(),
            model::ItemType::Mace(_) => '\\'.into(),
            model::ItemType::Boots(_) |
            model::ItemType::Gloves(_) |
            model::ItemType::Cloak(_) |
            model::ItemType::Helm(_) => ']'.into(),
            model::ItemType::Shield(_) => ')'.into(),
            model::ItemType::HardArmor(_) => '['.into(),
            model::ItemType::SoftArmor(_) => '('.into(),
            model::ItemType::Bracers(_) |
            model::ItemType::Belt(_) => ']'.into(),
            model::ItemType::Amulet(_) => '"'.into(),
            model::ItemType::Ring(_) => '='.into(),
            model::ItemType::Staff(_) => '_'.into(),
            model::ItemType::Wand(_) => '-'.into(),
            model::ItemType::Scroll(_) => '?'.into(),
            model::ItemType::Potion(_) => '!'.into(),
            model::ItemType::Food(_) |
            model::ItemType::JunkFood(_) => ','.into(),
            model::ItemType::Chime(_) |
            model::ItemType::Horn(_) => '%'.into(),
            model::ItemType::MagicBook(_) |
            model::ItemType::PrayerBook(_) => '?'.into(),
            model::ItemType::Instrument(_) => '%'.into(),
            model::ItemType::SongBook(_) => '?'.into(),
            model::ItemType::LodgingAtInn(_) => '.'.into(),
            model::ItemType::Money(_) => '$'.into(),
            model::ItemType::UnseenTrap(_) => '.'.into(),
            model::ItemType::SeenTrap(_) => '^'.into(),
            model::ItemType::Rubble(_) => '+'.into(),
            model::ItemType::OpenDoor(_) => '\''.into(),
            model::ItemType::ClosedDoor(_) => '+'.into(),
            model::ItemType::UpStaircase(_) => '<'.into(),
            model::ItemType::DownStaircase(_) => '>'.into(),
            model::ItemType::SecretDoor(_) => '#'.into(),
            model::ItemType::EntranceToStore(_) => {
                match item.subval {
                    101 => 'G' as pancurses::chtype| pancurses::A_BOLD,    // General store
                    102 => 'R' as pancurses::chtype| pancurses::A_BOLD,    // Armory
                    103 => 'W' as pancurses::chtype| pancurses::A_BOLD,    // Weapon smith
                    104 => 'T' as pancurses::chtype| pancurses::A_BOLD,    // Temple
                    105 => 'A' as pancurses::chtype| pancurses::A_BOLD,    // Alchemy Shop
                    106 => 'M' as pancurses::chtype| pancurses::A_BOLD,    // Magic shop
                    107 => 'I' as pancurses::chtype| pancurses::A_BOLD,    // Inn
                    109 => 'L' as pancurses::chtype| pancurses::A_BOLD,    // Library
                    110 => 'U' as pancurses::chtype| pancurses::A_BOLD,    // Music shop
                    113 => 'J' as pancurses::chtype| pancurses::A_BOLD,    // Gem store
                    116 => 'D' as pancurses::chtype| pancurses::A_BOLD,    // Deli
                    118 => '+' as pancurses::chtype,                       // Black market
                    108 => 'P' as pancurses::chtype| pancurses::A_BOLD,    // Trading post
                    111 => 'N' as pancurses::chtype| pancurses::A_BOLD,    // Insurance shop
                    112 => 'B' as pancurses::chtype| pancurses::A_BOLD,    // Bank
                    114 => 'X' as pancurses::chtype| pancurses::A_BOLD,    // Money exchange
                    115 => 'C' as pancurses::chtype| pancurses::A_BOLD,    // Casino
                    117 => 'Q' as pancurses::chtype| pancurses::A_BOLD,    // Quest house
                    _   => '+' as pancurses::chtype,
                }
            },
            model::ItemType::UpSteepStaircase(_) => '<'.into(),
            model::ItemType::DownSteepStaircase(_) => '>'.into(),
            model::ItemType::Whirlpool(_) => '&'.into(),
    }
}
