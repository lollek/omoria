use crate::data;
use crate::logic::menu;
use crate::player;
use crate::term;

pub fn show_class_restrictions() {
    let class = player::class();
    term::clear_screen();
    menu::draw_help(
        data::class::name(&class),
        data::class::restriction_info(&class),
    );
}
