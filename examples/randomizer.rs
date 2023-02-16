use clap::{Parser, Subcommand, ValueEnum};
use rand::Rng;
use smog_strat_dex_rs::basics::{BasicsPokemon, BasicsResponse};
use smog_strat_dex_rs::pokemon::MoveSet;
use smog_strat_dex_rs::{Client, Generation};
use std::fmt::{Display, Formatter};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Any,
    Custom {
        #[arg(value_enum)]
        gen: Option<Gen>,
        #[arg(value_enum)]
        format: Option<Form>,
    },
}

#[derive(ValueEnum, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Gen {
    SV,
    SS,
    SM,
    XY,
    BW,
    DP,
    RS,
    GS,
    RB,
}

static GENS: [Gen; 9] = [
    Gen::SV,
    Gen::SS,
    Gen::SM,
    Gen::XY,
    Gen::BW,
    Gen::DP,
    Gen::RS,
    Gen::GS,
    Gen::RB,
];

impl From<Gen> for Generation {
    fn from(value: Gen) -> Self {
        match value {
            Gen::SV => Generation::ScarletViolet,
            Gen::SS => Generation::SwordShield,
            Gen::SM => Generation::SunMoon,
            Gen::XY => Generation::XY,
            Gen::BW => Generation::BlackWhite,
            Gen::DP => Generation::DiamondPearl,
            Gen::RS => Generation::RubySapphire,
            Gen::GS => Generation::GoldSilver,
            Gen::RB => Generation::RedBlue,
        }
    }
}

impl Display for Gen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Gen::SV => write!(f, "SV"),
            Gen::SS => write!(f, "SS"),
            Gen::SM => write!(f, "SM"),
            Gen::XY => write!(f, "XY"),
            Gen::BW => write!(f, "BW"),
            Gen::DP => write!(f, "DP"),
            Gen::RS => write!(f, "RS"),
            Gen::GS => write!(f, "GS"),
            Gen::RB => write!(f, "RB"),
        }
    }
}

#[derive(ValueEnum, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Form {
    OU,
    UU,
    NU,
    LC,
}

impl Display for Form {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Form::OU => write!(f, "OU"),
            Form::UU => write!(f, "UU"),
            Form::NU => write!(f, "NU"),
            Form::LC => write!(f, "LC"),
        }
    }
}

#[tokio::main]
async fn main() -> smog_strat_dex_rs::Result<()> {
    let cli = Cli::parse();

    let (gen, format) = if let Command::Custom { gen, format } = cli.command {
        (gen, format)
    } else {
        (None, None)
    };

    let gen = gen.unwrap_or_else(|| {
        let index = rand::thread_rng().gen_range(0..9);
        GENS[index]
    });

    let basics = Client::get_basics(gen.into()).await?;

    let mut pokemon;
    let mut move_set;
    while {
        pokemon = get_pokemon(&basics)?;
        move_set = get_move_set(pokemon, gen.into(), format).await?;
        move_set.is_none()
    } {}
    if let Some((format, move_set)) = move_set {
        println!("Generation: {gen}\nFormat: {format}\n{move_set}");
    }
    Ok(())
}

fn get_pokemon(basics: &BasicsResponse) -> smog_strat_dex_rs::Result<&BasicsPokemon> {
    let pokemon_list = basics
        .pokemon
        .iter()
        .filter(|p| p.is_non_standard.as_str() == "Standard")
        .collect::<Vec<_>>();

    let index: usize = rand::thread_rng().gen_range(0..pokemon_list.len());

    Ok(pokemon_list.into_iter().nth(index).unwrap())
}

async fn get_move_set(
    basic_pokemon: &BasicsPokemon,
    gen: Generation,
    format: Option<Form>,
) -> smog_strat_dex_rs::Result<Option<(String, MoveSet)>> {
    let name = basic_pokemon.name.to_lowercase().replace(' ', "-");
    let pokemon = if let Ok(resp) = Client::get_pokemon(gen, &name).await {
        resp
    } else {
        Client::get_pokemon(gen, name.replace("-mega", "")).await?
    };

    let strategies = if let Some(format) = format {
        pokemon
            .strategies
            .into_iter()
            .filter(|i| i.format == format.to_string())
            .collect::<Vec<_>>()
    } else {
        pokemon.strategies
    };

    if strategies.is_empty() {
        return Ok(None);
    }

    let strat_index: usize = rand::thread_rng().gen_range(0..strategies.len());

    let strategy = strategies.into_iter().nth(strat_index).unwrap();

    if strategy.move_sets.is_empty() {
        return Ok(None);
    }

    let move_set_index: usize = rand::thread_rng().gen_range(0..strategy.move_sets.len());

    Ok(strategy
        .move_sets
        .into_iter()
        .nth(move_set_index)
        .map(|s| (strategy.format, s)))
}
