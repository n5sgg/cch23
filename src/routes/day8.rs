use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PokemonWeightById {
    weight: i32,
}

async fn weight(
    State(client): State<reqwest::Client>,
    Path(id): Path<String>,
) -> Result<String, StatusCode> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{id}");
    match client.get(url).send().await {
        Ok(resp) => {
            let pokemon = resp
                .json::<PokemonWeightById>()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((pokemon.weight / 10).to_string())
        }
        Err(err) => {
            println!("Request Error: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn drop(
    State(client): State<reqwest::Client>,
    Path(id): Path<String>,
) -> Result<String, StatusCode> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{id}");
    const GRAVITATIONAL_ACCELERATION: f32 = 9.825;
    match client.get(url).send().await {
        Ok(resp) => {
            let pokemon = resp
                .json::<PokemonWeightById>()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let speed = (2f32 * GRAVITATIONAL_ACCELERATION * 10f32).sqrt();
            let momentum = ((pokemon.weight / 10) as f32) * speed;
            Ok(momentum.to_string())
        }
        Err(err) => {
            println!("Request Error: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn routes() -> Router<reqwest::Client> {
    Router::new()
        .route("/weight/:id", get(weight))
        .route("/drop/:id", get(drop))
}
