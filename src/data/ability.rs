use model;

pub fn name(ability: &model::Ability) -> &'static str {
    match ability {
        model::Ability::Rage => "Rage",
    }
}