use crate::model::item_subtype::SpikeSubType;

pub fn from_usize(subtype: usize) -> Option<SpikeSubType> {
    match subtype {
        1 => Some(SpikeSubType::IronSpike),
        _ => None,
    }
}

pub fn to_usize(subtype: SpikeSubType) -> usize {
    match subtype {
        SpikeSubType::IronSpike => 1,
    }
}
