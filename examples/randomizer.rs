use rand::Rng;
use smog_strat_dex_rs::{Client, Generation};
use smog_strat_dex_rs::basics::{BasicsPokemon, BasicsResponse};
use smog_strat_dex_rs::pokemon::MoveSet;

#[tokio::main]
async fn main() -> smog_strat_dex_rs::Result<()> {
    let basics = Client::get_basics(Generation::ScarletViolet).await?;

    let mut pokemon;
    let mut move_set;
    while {pokemon = get_pokemon(&basics).await?; move_set = get_move_set(pokemon).await?; move_set.is_none() } {}
    if let Some((format, move_set)) = move_set {
        println!("Format: {format}\n{move_set}");
    }
    Ok(())
}

async fn get_pokemon(basics: &BasicsResponse) -> smog_strat_dex_rs::Result<&BasicsPokemon> {
    let pokemon_list = basics
        .pokemon
        .iter()
        .filter(|p| p.is_non_standard.as_str() == "Standard")
        .collect::<Vec<_>>();

    let index: usize = rand::thread_rng().gen_range(0..pokemon_list.len());

    Ok(pokemon_list.into_iter().nth(index).unwrap())
}

async fn get_move_set(basic_pokemon: &BasicsPokemon) -> smog_strat_dex_rs::Result<Option<(String, MoveSet)>> {
    let pokemon = Client::get_pokemon(Generation::ScarletViolet, basic_pokemon.name.to_lowercase().replace(' ', "-")).await?;
    if pokemon.strategies.is_empty() {
        return Ok(None);
    }

    let strat_index: usize = rand::thread_rng().gen_range(0..pokemon.strategies.len());

    let strategy = pokemon.strategies.into_iter().nth(strat_index).unwrap();

    if strategy.move_sets.is_empty() {
        return Ok(None);
    }

    let move_set_index: usize = rand::thread_rng().gen_range(0..strategy.move_sets.len());

    Ok(strategy.move_sets.into_iter().nth(move_set_index).map(|s| (strategy.format, s)))
}