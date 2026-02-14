#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WearableGemSubType {
    GemOfTeleportation,
    GemOfResistCold,
    GemOfResistAcid,
    GemOfSeeInvisible,
    GemOfStealth,
    GemOfSlowDigestion,
    GemOfProtectFire,
}
