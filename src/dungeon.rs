use menu::helpers;
use player;
use term;

pub fn show_class_restrictions() {
    let class = player::class();
    term::clear_screen();
    helpers::draw_help(
        class.name(),
        class.restriction_info());
}
