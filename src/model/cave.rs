#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
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

impl Default for Cave {
    fn default() -> Self {
        Cave {
            cptr: 0,
            tptr: 0,
            fval: 0,
            fopen: 0,
            fm: 0,
            pl: 0,
            tl: 0,
            moved: 0,
            oct: 0,
            h2o: 0,
        }
    }
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