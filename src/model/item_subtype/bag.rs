#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BagSubType {
    BagOfHolding250,
    BagOfHolding500,
    BagOfHolding1000,
    BagOfHolding1500,
    BagOfDevouring,
}
