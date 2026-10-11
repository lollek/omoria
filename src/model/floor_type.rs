use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FloorType {
    BlankFloor,
    DarkOpenFloor,
    LightOpenFloor,
    CorrOpenFloor,
    CorrRoomJunction,
    CorrDoor,
    CorrSecretDoor,
    WallGranite,
    WallMagma,
    WallQuartz,
    BoundaryWall,
    WaterOnFloor,
    WaterOnRoomFloor,
    WaterOnFloorLit,
}

impl TryFrom<u8> for FloorType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FloorType::BlankFloor),
            1 => Ok(FloorType::DarkOpenFloor),
            2 => Ok(FloorType::LightOpenFloor),
            4 => Ok(FloorType::CorrOpenFloor),
            5 => Ok(FloorType::CorrRoomJunction),
            6 => Ok(FloorType::CorrDoor),
            7 => Ok(FloorType::CorrSecretDoor),
            10 => Ok(FloorType::WallGranite),
            11 => Ok(FloorType::WallMagma),
            12 => Ok(FloorType::WallQuartz),
            15 => Ok(FloorType::BoundaryWall),
            16 => Ok(FloorType::WaterOnFloor),
            17 => Ok(FloorType::WaterOnRoomFloor),
            18 => Ok(FloorType::WaterOnFloorLit),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_floor_values_map_to_floor_types() {
        let cases = [
            (0, FloorType::BlankFloor),
            (1, FloorType::DarkOpenFloor),
            (2, FloorType::LightOpenFloor),
            (4, FloorType::CorrOpenFloor),
            (5, FloorType::CorrRoomJunction),
            (6, FloorType::CorrDoor),
            (7, FloorType::CorrSecretDoor),
            (10, FloorType::WallGranite),
            (11, FloorType::WallMagma),
            (12, FloorType::WallQuartz),
            (15, FloorType::BoundaryWall),
            (16, FloorType::WaterOnFloor),
            (17, FloorType::WaterOnRoomFloor),
            (18, FloorType::WaterOnFloorLit),
        ];
        for (value, expected) in cases {
            assert_eq!(FloorType::try_from(value), Ok(expected), "fval {}", value);
        }
    }

    #[test]
    fn unassigned_floor_values_are_rejected() {
        for value in [3, 8, 9, 13, 14, 19, u8::MAX] {
            assert_eq!(FloorType::try_from(value), Err(()), "fval {}", value);
        }
    }
}
