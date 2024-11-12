use reqwest;
use serde::Deserialize;
use std::io;

#[derive(Debug, Deserialize)]
struct PokemonType {
    #[serde(rename = "type")]
    poke_type: TypeDetail,
}

#[derive(Debug, Deserialize)]
struct TypeDetail {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Pokemon {
    name: String,
    weight: u32,
    id: u32,
    types: Vec<PokemonType>,
}

fn main() {
    const URL: &str = "https://pokeapi.co/api/v2/pokemon/";

    let pokemon_id: u32 = loop {
        println!("Enter a Pokemon ID:");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        match user_input.trim().parse() {
            Ok(id) => break id,
            Err(_) => println!("Please enter a valid number"),
        }
    };

    let pokemon_url: String = format!("{}{}", URL.clone(), pokemon_id.to_string());

    match fetch_pokemon(&pokemon_url) {
        Ok(pokemon) => {
            println!("ID:\t{}", pokemon.id);
            println!("Name:\t{}", pokemon.name);
            println!("Name:\t{}", pokemon.weight);
            println!("Types:");
            for poke_type in pokemon.types {
                println!("-{}", poke_type.poke_type.name);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    };
}

fn fetch_pokemon(url: &str) -> Result<Pokemon, String> {
    let response = reqwest::blocking::get(url).map_err(|err| format!("Request Failed: {}", err))?;

    if response.status().is_success() {
        response
            .json::<Pokemon>()
            .map_err(|err| format!("JSON parsing error: {}", err))
    } else {
        Err(format!("HTTP Error: {}", response.status()))
    }
}
