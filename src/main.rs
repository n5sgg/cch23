mod routes;

use axum::{routing::get, Router};
use reqwest::Client;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let client = Client::new();
    let router = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/-1", routes::day0::routes())
        .nest("/1", routes::day1::routes())
        .nest("/4", routes::day4::routes())
        .nest("/6", routes::day6::routes())
        .nest("/7", routes::day7::routes())
        .nest("/8", routes::day8::routes().with_state(client));

    Ok(router.into())
}
