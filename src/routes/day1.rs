use std::num::ParseIntError;

use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn sled_id(Path(path): Path<String>) -> Result<String, StatusCode> {
    let sled_ids: Result<Vec<i32>, ParseIntError> = path
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
    ids.into_iter()
        .reduce(|acc, e| acc ^ e)
        .expect("should be valid")
        .pow(3)
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

pub fn routes() -> Router {
    Router::new().route("/*ids", get(sled_id))
}
