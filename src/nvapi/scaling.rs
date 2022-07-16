use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
#[repr(i32)]
pub enum Scaling {
    Default = 0,
    BalancedFullScreen = 1,
    ForcedFullScreen = 2,
    ForcedCentered = 3,
    ForcedAspectRatio = 5,
    BalancedAspectRatio = 6,
    BalancedCentered = 7,
    ForcedIntegerScaling = 8,
}

impl FromStr for Scaling {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "bfs" => Ok(Self::BalancedFullScreen),
            "ffs" => Ok(Self::ForcedFullScreen),
            "fc" => Ok(Self::ForcedCentered),
            "far" => Ok(Self::ForcedAspectRatio),
            "bar" => Ok(Self::BalancedAspectRatio),
            "bc" => Ok(Self::BalancedCentered),
            "fis" => Ok(Self::ForcedIntegerScaling),
            _ => Err("Invalid scaling argument".to_string()),
        }
    }
}

impl From<i32> for Scaling {
    fn from(num: i32) -> Self {
        match num {
            0 => Self::Default,
            1 => Self::BalancedFullScreen,
            2 => Self::ForcedFullScreen,
            3 => Self::ForcedCentered,
            5 => Self::ForcedAspectRatio,
            6 => Self::BalancedAspectRatio,
            7 => Self::BalancedCentered,
            8 => Self::ForcedIntegerScaling,
            _ => Self::Default,
        }
    }
}

impl Display for Scaling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Scaling::Default => "default",
                Scaling::BalancedFullScreen => "balanced full screen",
                Scaling::ForcedFullScreen => "forced full screen",
                Scaling::ForcedCentered => "forced centered",
                Scaling::ForcedAspectRatio => "forced aspect ratio",
                Scaling::BalancedAspectRatio => "balanced aspect ratio",
                Scaling::BalancedCentered => "balanced centered",
                Scaling::ForcedIntegerScaling => "forced integer scaling",
            }
        )
    }
}
