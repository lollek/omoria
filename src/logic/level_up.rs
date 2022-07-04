use crate::data;
use crate::player;

pub fn gain_mana_from_level_up() {
    if let Some(magic) = data::class::magic_type(&player::class()) {
        player::modify_max_mp(player::modifier_from_stat(data::magic::modifier_stat(&magic)));
    }
}