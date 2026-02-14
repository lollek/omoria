#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PickSubType {
    Pick,
    Shovel,
    OrcishPick1,
    OrcishPick2,
    DwarvenPick,
    GnomishShovel,
    DwarvenShovel,
}
