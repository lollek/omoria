#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Default)]
pub struct Cave {
    #[serde(skip_serializing, default)]
    pub cptr: u8,
    #[serde(skip_serializing, default)]
    pub tptr: u8,
    pub fval: u8,
    pub fopen: u8,
    pub fm: u8,
    pub pl: u8,
    #[serde(skip_serializing, default)]
    pub tl: u8,
    #[serde(skip_serializing, default)]
    pub moved: u8,
    #[serde(skip_serializing, default)]
    pub oct: u8,
    #[serde(skip_serializing, default)]
    pub h2o: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let cave = Cave::default();
        serde_json::to_string(&cave).expect("Failed to serialize Cave");
    }
}
