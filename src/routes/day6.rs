use axum::{extract::Json, routing::post, Router};
use serde::Serialize;

#[derive(Serialize)]
struct ElfCountingResponse {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf: usize,
}

async fn elf_counting(body: String) -> Json<ElfCountingResponse> {
    let elf = body.matches("elf").count();
    let elf_on_a_shelf = body.matches("elf on a shelf").count();
    let shelf = body.matches("shelf").count();
    Json(ElfCountingResponse {
        elf: elf,
        elf_on_a_shelf: elf_on_a_shelf,
        shelf_with_no_elf: shelf - elf_on_a_shelf,
    })
}

pub fn routes() -> Router {
    Router::new().route("/", post(elf_counting))
}
