use std::num::ParseIntError;

use axum::{extract::Path, http::StatusCode, routing::get, Router};

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


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(internal_error))
        .route("/1/*ids", get(sled_id));

    Ok(router.into())
}
