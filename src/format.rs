use crate::Generation;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct FormatRequest<'a> {
    pub alias: &'a str,
    pub gen: Generation,
    pub language: &'static str,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FormatResponse {
    pub languages: Vec<String>,
    pub description: String,
    pub pokemon_with_strategies: Vec<String>,
}
