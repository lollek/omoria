use logic::menu;
use player;
use term;

pub fn show_class_restrictions() {
    let class = player::class();
    term::clear_screen();
    menu::draw_help(
        class.name(),
        class.restriction_info());
}

