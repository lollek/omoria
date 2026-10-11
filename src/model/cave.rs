use std::convert::TryFrom;

use super::FloorType;

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

impl Cave {
    // `fm` is a legacy "field mark" flag; its meaning is unknown but it counts as lit.
    pub fn is_lit(&self) -> bool {
        self.pl != 0 || self.tl != 0 || self.fm != 0
    }

    pub fn is_open(&self) -> bool {
        self.fopen != 0
    }

    pub fn floor_type(&self) -> Option<FloorType> {
        FloorType::try_from(self.fval).ok()
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

    #[test]
    fn default_cave_is_unlit() {
        assert!(!Cave::default().is_lit());
    }

    #[test]
    fn is_lit_when_any_lighting_flag_is_set() {
        let flags: [fn(&mut Cave); 3] = [
            |cave: &mut Cave| cave.pl = 1,
            |cave: &mut Cave| cave.tl = 1,
            |cave: &mut Cave| cave.fm = 1,
        ];
        for set_flag in flags {
            let mut cave = Cave::default();
            set_flag(&mut cave);
            assert!(cave.is_lit(), "lighting flag should light the cell");
        }
    }

    #[test]
    fn is_open_follows_fopen_flag() {
        let mut cave = Cave::default();
        assert!(!cave.is_open());
        cave.fopen = 1;
        assert!(cave.is_open());
    }

    #[test]
    fn floor_type_comes_from_fval() {
        let cave = Cave {
            fval: 10,
            ..Cave::default()
        };
        assert_eq!(cave.floor_type(), Some(FloorType::WallGranite));
    }

    #[test]
    fn floor_type_is_none_for_unassigned_fval() {
        let cave = Cave {
            fval: 3,
            ..Cave::default()
        };
        assert_eq!(cave.floor_type(), None);
    }
}
