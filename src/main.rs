use axum::{
    Router,
    routing::get,
    http::StatusCode
};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn internal_error() -> Result<(), StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(internal_error));

    Ok(router.into())
}
