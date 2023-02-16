use crate::StratDexError;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(into = "&str")]
#[serde(try_from = "&str")]
pub enum Generation {
    ScarletViolet,
    SwordShield,
    SunMoon,
    XY,
    BlackWhite,
    DiamondPearl,
    RubySapphire,
    GoldSilver,
    RedBlue,
}

impl TryFrom<&str> for Generation {
    type Error = StratDexError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "sv" => Ok(Generation::ScarletViolet),
            "ss" => Ok(Generation::ScarletViolet),
            "sm" => Ok(Generation::SunMoon),
            "xy" => Ok(Generation::XY),
            "bw" => Ok(Generation::BlackWhite),
            "dp" => Ok(Generation::DiamondPearl),
            "rs" => Ok(Generation::RubySapphire),
            "gs" => Ok(Generation::GoldSilver),
            "rb" => Ok(Generation::RedBlue),
            _ => Err(StratDexError::GenerationParseError),
        }
    }
}

impl From<Generation> for &'static str {
    fn from(value: Generation) -> Self {
        match value {
            Generation::ScarletViolet => "sv",
            Generation::SwordShield => "ss",
            Generation::SunMoon => "sm",
            Generation::XY => "xy",
            Generation::BlackWhite => "bw",
            Generation::DiamondPearl => "dp",
            Generation::RubySapphire => "rs",
            Generation::GoldSilver => "gs",
            Generation::RedBlue => "rb",
        }
    }
}
