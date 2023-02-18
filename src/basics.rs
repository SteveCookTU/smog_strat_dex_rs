use crate::Generation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
pub(crate) struct BasicsRequest {
    pub gen: Generation,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BasicsResponse {
    pub pokemon: Vec<BasicsPokemon>,
    pub formats: Vec<Format>,
    pub natures: Vec<Nature>,
    pub abilities: Vec<Ability>,
    pub moves: Vec<Move>,
    pub types: Vec<Type>,
    pub items: Vec<Item>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BasicsPokemon {
    pub name: String,
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spa: u8,
    pub spd: u8,
    pub spe: u8,
    pub weight: f32,
    pub height: f32,
    pub types: Vec<String>,
    pub abilities: Vec<String>,
    pub formats: Vec<String>,
    #[serde(rename = "isNonstandard")]
    pub is_non_standard: String,
    pub oob: Option<PokemonOob>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PokemonOob {
    pub dex_number: i16,
    pub evos: Vec<String>,
    pub alts: Vec<String>,
    #[serde(rename = "genfamily")]
    pub gen_family: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Format {
    pub name: String,
    pub shorthand: String,
    #[serde(rename = "genfamily")]
    pub gen_family: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Nature {
    pub name: String,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub spa: f32,
    pub spd: f32,
    pub spe: f32,
    pub summary: String,
    #[serde(rename = "genfamily")]
    pub gen_family: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Ability {
    pub name: String,
    pub description: String,
    #[serde(rename = "isNonstandard")]
    pub is_non_standard: String,
    #[serde(rename = "genfamily")]
    pub gen_family: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Move {
    pub name: String,
    #[serde(rename = "isNonstandard")]
    pub is_non_standard: String,
    pub category: String,
    pub power: u8,
    pub accuracy: u8,
    pub priority: i8,
    pub pp: u8,
    pub description: String,
    #[serde(rename = "type")]
    pub move_type: String,
    pub flags: Vec<String>,
    #[serde(rename = "genfamily")]
    pub gen_family: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Type {
    pub name: String,
    pub atk_effectives: Vec<(String, f32)>,
    #[serde(rename = "genfamily")]
    pub gen_family: Vec<String>,
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Item {
    pub name: String,
    pub description: String,
    #[serde(rename = "isNonstandard")]
    pub is_non_standard: String,
    #[serde(rename = "genfamily")]
    pub gen_family: Vec<String>,
}
