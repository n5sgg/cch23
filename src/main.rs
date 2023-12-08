use std::num::ParseIntError;

use axum::{
    extract::{Json, Path},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

// -----------------------------------------------
// Day -1: Get your winter boots on!
// -----------------------------------------------
// Task 1: Everything is OK
async fn hello_world() -> &'static str {
    "Hello, world!"
}

// Task 2: Fake error (0 bonus points)
async fn internal_error() -> Result<(), StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
// Day -1: End

// -----------------------------------------------
// Day 1: Packet "exclusive-cube" not found
// -----------------------------------------------
// Task 1 and 2: The sled ID system
async fn sled_id(Path(ids): Path<String>) -> Result<String, StatusCode> {
    let sled_ids: Result<Vec<i32>, ParseIntError> = ids
        .split("/")
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    let sled_ids: Vec<i32> = match sled_ids {
        Ok(ids) => ids,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    if sled_ids.len() < 1 || sled_ids.len() > 20 {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(compute_sled_ids(sled_ids).to_string())
}

fn compute_sled_ids(ids: Vec<i32>) -> i32 {
    // All packet IDs (integers) are XOR'ed with each other, and then the result is (again) raised to the power of 3
    let xor_result = ids.into_iter().reduce(|acc, e| acc ^ e).unwrap();
    i32::pow(xor_result, 3)
}

#[cfg(test)]
mod tests {
    use super::compute_sled_ids;

    // Inspired by https://over-codes.github.io/rust-table-driven-tests.html
    macro_rules! sled_ids_test {
        ($($name:ident: $input:expr, $exp:expr,)*) => {
            mod sled_ids_test {
                use super::compute_sled_ids;
                $(
                    #[test]
                    fn $name() {
                        let rec = compute_sled_ids($input);
                        assert_eq!($exp, rec);
                    }
                )*
            }
        }
    }

    sled_ids_test!(
        given_3_9_12_13_15_exp_64: vec![3, 9, 12, 13, 15], 64,
        given_10_exp_1000: vec![10], 1000,
        given_4_5_8_10_exp_27: vec![4, 5, 8, 10], 27,
    );
}
// Day 1: End

// -----------------------------------------------
// Day 4: What do you call a serialized reindeer? Serdeer!
// -----------------------------------------------
// Task 1: Reindeer cheer
#[derive(Deserialize)]
#[allow(dead_code)]
struct ReindeerGroupRequest {
    name: String,
    strength: i32,
}

async fn reindeer_group_strength(Json(payload): Json<Vec<ReindeerGroupRequest>>) -> String {
    payload
        .into_iter()
        .fold(0, |acc, e| acc + e.strength)
        .to_string()
}

// Task 2: Cursed candy eating contest
#[derive(Deserialize)]
struct ReindeerContestRequest {
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
struct ReindeerContestResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

impl Default for ReindeerContestResponse {
    fn default() -> ReindeerContestResponse {
        ReindeerContestResponse {
            fastest: "".to_string(),
            tallest: "".to_string(),
            magician: "".to_string(),
            consumer: "".to_string(),
        }
    }
}

async fn reindeer_contest(
    Json(payload): Json<Vec<ReindeerContestRequest>>,
) -> Json<ReindeerContestResponse> {
    let mut fastest: f32 = std::f32::MIN;
    let mut tallest: i32 = std::i32::MIN;
    let mut magician: i32 = std::i32::MIN;
    let mut consumer: i32 = std::i32::MIN;
    let mut resp = ReindeerContestResponse::default();

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
// Day 4: End

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(internal_error))
        .route("/1/*ids", get(sled_id))
        .route("/4/strength", post(reindeer_group_strength))
        .route("/4/contest", post(reindeer_contest));

    Ok(router.into())
}
