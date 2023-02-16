use thiserror::Error;

pub mod basics;
pub mod format;
mod generation;
pub mod pokemon;

use crate::basics::{BasicsRequest, BasicsResponse};
use crate::format::{FormatRequest, FormatResponse};
use crate::pokemon::{PokemonRequest, PokemonResponse};
pub use generation::*;

pub type Result<T> = std::result::Result<T, StratDexError>;

#[derive(Error, Debug)]
pub enum StratDexError {
    #[error("Failed to parse generation")]
    GenerationParseError,
    #[error("Failed to fetch: {0}")]
    FetchError(#[from] reqwest::Error),
}

pub struct Client;

impl Client {
    pub async fn get_basics(gen: Generation) -> Result<BasicsResponse> {
        let client = reqwest::Client::new();
        let basics_request = BasicsRequest { gen };
        let response = client
            .post("https://www.smogon.com/dex/_rpc/dump-basics")
            .json(&basics_request)
            .send()
            .await?
            .json::<BasicsResponse>()
            .await?;
        Ok(response)
    }

    pub async fn get_pokemon(gen: Generation, pokemon: impl AsRef<str>) -> Result<PokemonResponse> {
        let client = reqwest::Client::new();
        let request = PokemonRequest {
            gen,
            alias: pokemon.as_ref(),
            language: "en",
        };
        let response = client
            .post("https://www.smogon.com/dex/_rpc/dump-pokemon")
            .json(&request)
            .send()
            .await?
            .json::<PokemonResponse>()
            .await?;
        Ok(response)
    }

    pub async fn get_format(gen: Generation, format: impl AsRef<str>) -> Result<FormatResponse> {
        let client = reqwest::Client::new();
        let request = FormatRequest {
            gen,
            alias: format.as_ref(),
            language: "en",
        };
        let response = client
            .post("https://www.smogon.com/dex/_rpc/dump-format")
            .json(&request)
            .send()
            .await?
            .json::<FormatResponse>()
            .await?;
        Ok(response)
    }
}
