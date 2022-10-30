use crate::model::item_subtype::CloakSubType;

pub fn from_usize(subtype: usize) -> Option<CloakSubType> {
    match subtype {
        1 => Some(CloakSubType::LightCloak),
        2 => Some(CloakSubType::HeavyCloak),
        3 => Some(CloakSubType::SharkskinCloak),
        4 => Some(CloakSubType::DemonhideCloak),
        5 => Some(CloakSubType::WyrmhideCloak),
        _ => None,
    }
}

pub fn to_usize(subtype: CloakSubType) -> usize {
    match subtype {
        CloakSubType::LightCloak => 1,
        CloakSubType::HeavyCloak => 2,
        CloakSubType::SharkskinCloak => 3,
        CloakSubType::DemonhideCloak => 4,
        CloakSubType::WyrmhideCloak => 5,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_double_conversion() {
        (0..1000).for_each(|i| {
            if let Some(subtype) = from_usize(i) {
                assert_eq!(i, to_usize(subtype));
            }
        })
    }
}
