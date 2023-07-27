use serde::Serialize;

#[derive(Serialize)]
pub struct Pseres{
    api_key: String,
    provider_id: i32,
}

#[derive(Serialize)]
pub struct Info {
    pub name: String,
}
