use smog_strat_dex_rs::{Client, Generation};

#[tokio::main]
async fn main() -> smog_strat_dex_rs::Result<()> {
    let pokemon = Client::get_pokemon(Generation::ScarletViolet, "venusaur").await?;
    println!("{}", pokemon.strategies[0].move_sets[0]);
    Ok(())
}
