#[derive(Debug, PartialEq, Eq)]
pub enum ItemSubType {
    LightSource(LightSourceSubType),
}

#[derive(Debug, PartialEq, Eq)]
pub enum LightSourceSubType {
    WoodenTorch,
    BrassLantern,
    MagicTorch,
    MagicLantern,
}
