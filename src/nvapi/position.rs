use std::str::FromStr;

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error_msg = "Invalid position option".to_string();
        let mut str_iter = s.split(",");
        let x =
            i32::from_str(str_iter.next().ok_or_else(|| &error_msg)?).map_err(|_| &error_msg)?;
        let y =
            i32::from_str(str_iter.next().ok_or_else(|| &error_msg)?).map_err(|_| &error_msg)?;
        Ok(Self { x, y })
    }
}
