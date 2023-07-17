use serde::Serialize;

#[derive(Serialize)]
pub struct Pseres{
    api_key: String,
    provider_id: i32,
}

