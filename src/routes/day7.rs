use axum::{http::StatusCode, routing::get, Json, Router};

use std::{collections::HashMap, str};
use axum_extra::extract::cookie::CookieJar;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};

fn decode_cookie_recipe(jar: CookieJar) -> Result<String, StatusCode> {
    if let Some(cookie) = jar.get("recipe") {
        return match STANDARD.decode(cookie.value()) {
            Ok(decoded) => String::from_utf8(decoded).map_err(|_| StatusCode::BAD_REQUEST),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        };
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }
}

async fn decode(jar: CookieJar) -> Result<String, StatusCode> {
    decode_cookie_recipe(jar)
}

#[derive(Deserialize)]
struct BakePlan {
    recipe: HashMap<String, u32>,
    pantry: HashMap<String, u32>,
}

#[derive(Serialize, Debug)]
struct BakeResult {
    cookies: u32,
    pantry: HashMap<String, u32>,
}

async fn bake(jar: CookieJar) -> Result<Json<BakeResult>, StatusCode> {
    let decoded = match decode_cookie_recipe(jar) {
        Ok(n) => n,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let plan: BakePlan = match serde_json::from_str(&decoded) {
        Ok(value) => value,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let recipe = plan.recipe;
    let pantry = plan.pantry;

    let mut flour_ratio: u32 = 0;

    if recipe.get("flour").unwrap_or(&0u32) > &0u32 {
        flour_ratio = pantry.get("flour").unwrap_or(&0u32) / recipe.get("flour").unwrap_or(&0u32);
    }

    let mut remaining_ingredients = HashMap::new();

    for ingredient in pantry {
        let value = ingredient.1 - recipe.get(&ingredient.0).unwrap_or(&0u32) * flour_ratio;
        remaining_ingredients.insert(ingredient.0, value);
    }

    let bake_result = BakeResult {
        cookies: flour_ratio,
        pantry: remaining_ingredients,
    };
    
    Ok(Json(bake_result))
}

pub fn routes() -> Router {
    Router::new()
        .route("/decode", get(decode))
        .route("/bake", get(bake))
}
