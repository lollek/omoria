use crate::model::Sex;

pub fn name(sex: &Sex) -> &'static str {
    match sex {
        Sex::Female => "Female",
        Sex::Male => "Male",
    }
}