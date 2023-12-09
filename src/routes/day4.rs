use axum::{extract::Json, routing::post, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[allow(dead_code)]
struct PartialReindeer {
    name: String,
    strength: i32,
}

async fn reindeer_group_strength(Json(payload): Json<Vec<PartialReindeer>>) -> String {
    payload
        .into_iter()
        .fold(0, |acc, e| acc + e.strength)
        .to_string()
}

// Task 2: Cursed candy eating contest
#[derive(Deserialize)]
struct Reindeer {
    name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

#[derive(Serialize)]
struct ContestResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

impl Default for ContestResponse {
    fn default() -> ContestResponse {
        ContestResponse {
            fastest: "".to_string(),
            tallest: "".to_string(),
            magician: "".to_string(),
            consumer: "".to_string(),
        }
    }
}

async fn reindeer_contest(
    Json(payload): Json<Vec<Reindeer>>,
) -> Json<ContestResponse> {
    let mut fastest: f32 = std::f32::MIN;
    let mut tallest: i32 = std::i32::MIN;
    let mut magician: i32 = std::i32::MIN;
    let mut consumer: i32 = std::i32::MIN;
    let mut resp = ContestResponse::default();

    for reindeer in payload {
        if reindeer.speed > fastest {
            fastest = reindeer.speed;
            resp.fastest = format!(
                "Speeding past the finish line with a strength of {} is {}",
                reindeer.strength, reindeer.name
            );
        }
        if reindeer.height > tallest {
            tallest = reindeer.height;
            resp.tallest = format!(
                "{} is standing tall with his {} cm wide antlers",
                reindeer.name, reindeer.antler_width
            );
        }
        if reindeer.snow_magic_power > magician {
            magician = reindeer.snow_magic_power;
            resp.magician = format!(
                "{} could blast you away with a snow magic power of {}",
                reindeer.name, reindeer.snow_magic_power
            );
        }
        if reindeer.candies_eaten_yesterday > consumer {
            consumer = reindeer.candies_eaten_yesterday;
            resp.consumer = format!(
                "{} ate lots of candies, but also some {}",
                reindeer.name, reindeer.favorite_food
            );
        }
    }

    Json(resp)
}

pub fn routes() -> Router {
    Router::new()
        .route("/strength", post(reindeer_group_strength))
        .route("/contest", post(reindeer_contest))
}
