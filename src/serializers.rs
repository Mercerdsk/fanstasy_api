
use serde::{Deserialize, Serialize};

pub struct AppState {
    app_name: String,
}


#[derive(Deserialize)]
pub struct Dtype{
api_key: String,
provider_id: u32,
}

#[derive(Serialize)]
pub struct Pseres{
    api_key: String,
    provider_id: u32,
}