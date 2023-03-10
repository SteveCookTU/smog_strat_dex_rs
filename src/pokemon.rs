use crate::Generation;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize)]
pub(crate) struct PokemonRequest<'a> {
    pub gen: Generation,
    pub alias: &'a str,
    pub language: &'static str,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PokemonResponse {
    pub languages: Vec<String>,
    pub learnset: Vec<String>,
    pub strategies: Vec<Strategy>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Strategy {
    pub format: String,
    pub overview: String,
    pub comments: String,
    #[serde(rename = "movesets")]
    pub move_sets: Vec<MoveSet>,
    pub credits: Credits,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MoveSet {
    pub name: String,
    pub pokemon: String,
    pub shiny: bool,
    pub gender: String,
    pub description: String,
    pub abilities: Vec<String>,
    pub items: Vec<String>,
    #[serde(rename = "teratypes")]
    pub tera_types: Vec<String>,
    #[serde(rename = "moveslots")]
    pub move_slots: Vec<Vec<MoveSlot>>,
    #[serde(rename = "evconfigs")]
    pub ev_configs: Vec<EvConfig>,
    #[serde(rename = "ivconfigs")]
    pub iv_configs: Vec<IvConfig>,
    pub natures: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MoveSlot {
    #[serde(rename = "move")]
    pub move_name: String,
    #[serde(rename = "type")]
    pub move_type: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq)]
pub struct EvConfig {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spa: u8,
    pub spd: u8,
    pub spe: u8,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq)]
pub struct IvConfig {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spa: u8,
    pub spd: u8,
    pub spe: u8,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Credits {
    pub teams: Vec<Team>,
    #[serde(rename = "writtenBy")]
    pub written_by: Vec<Member>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Team {
    pub name: String,
    pub members: Vec<Member>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Member {
    pub user_id: Option<usize>,
    pub username: String,
}

impl Display for MoveSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} @ {}", self.pokemon, self.items.join(" / "))?;
        if !self.abilities.is_empty() {
            writeln!(f, "Ability: {}", self.abilities.join(" / "))?;
        }
        if !self.tera_types.is_empty() {
            writeln!(f, "Tera Type: {}", self.tera_types.join(" / "))?;
        }
        let ev_configs = self.get_ev_configs();
        if !ev_configs.is_empty() {
            writeln!(f, "EVs: {ev_configs}")?;
        }
        let iv_configs = self.get_iv_configs();
        if !iv_configs.is_empty() {
            writeln!(f, "IVs: {iv_configs}")?;
        }
        if !self.natures.is_empty() {
            writeln!(f, "{} Nature", self.natures.join(" / "))?;
        }
        for moves in self.move_slots.iter() {
            let moves = moves
                .iter()
                .map(|m| {
                    if let Some(mt) = m.move_type.as_ref() {
                        format!("{} {}", m.move_name, mt)
                    } else {
                        m.move_name.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(" / ");
            writeln!(f, "- {moves}")?;
        }
        Ok(())
    }
}

impl MoveSet {
    pub fn get_ev_configs(&self) -> String {
        let mut ev_configs = Vec::new();
        for ev_config in self.ev_configs.iter() {
            let &EvConfig {
                hp,
                atk,
                def,
                spa,
                spd,
                spe,
            } = ev_config;
            let mut evs = Vec::with_capacity(6);
            if hp != 0 {
                evs.push(format!("{hp} HP"));
            }
            if atk != 0 {
                evs.push(format!("{atk} Atk"));
            }
            if def != 0 {
                evs.push(format!("{def} Def"));
            }
            if spa != 0 {
                evs.push(format!("{spa} SpA"));
            }
            if spd != 0 {
                evs.push(format!("{spd} SpD"));
            }
            if spe != 0 {
                evs.push(format!("{spe} Spe"));
            }
            ev_configs.push(evs.join(" / "));
        }
        ev_configs.join(" | ")
    }

    pub fn get_iv_configs(&self) -> String {
        let mut iv_configs = Vec::new();
        for iv_config in self.iv_configs.iter() {
            let &IvConfig {
                hp,
                atk,
                def,
                spa,
                spd,
                spe,
            } = iv_config;
            let mut ivs = Vec::with_capacity(6);
            if hp != 31 {
                ivs.push(format!("{hp} HP"));
            }
            if atk != 31 {
                ivs.push(format!("{atk} Atk"));
            }
            if def != 31 {
                ivs.push(format!("{def} Def"));
            }
            if spa != 31 {
                ivs.push(format!("{spa} SpA"));
            }
            if spd != 31 {
                ivs.push(format!("{spd} SpD"));
            }
            if spe != 31 {
                ivs.push(format!("{spe} Spe"));
            }
            iv_configs.push(ivs.join(" / "));
        }
        iv_configs.join(" | ")
    }
}
