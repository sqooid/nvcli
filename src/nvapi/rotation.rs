use std::fmt::Display;

pub struct Rotation(pub i32);

impl TryFrom<&u32> for Rotation {
    type Error = String;

    fn try_from(value: &u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self(0)),
            90 => Ok(Self(1)),
            180 => Ok(Self(2)),
            270 => Ok(Self(3)),
            _ => Err("Invalid rotation value".to_string()),
        }
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                0 => "0",
                90 => "90",
                180 => "180",
                270 => "270",
                _ => "Invalid",
            }
        )
    }
}
