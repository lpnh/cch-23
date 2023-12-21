use rustemon::model::pokemon::Pokemon;

pub async fn get_pokemon_weight(id: u32) -> f64 {
    let poke_api_url = format!("https://pokeapi.co/api/v2/pokemon/{}/", id);
    
    let api_response = reqwest::get(poke_api_url).await.unwrap();

    let pokemon = api_response.json::<Pokemon>().await.unwrap();

    pokemon.weight as f64 / 10.0
}
