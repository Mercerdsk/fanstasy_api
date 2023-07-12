

use actix_web::{get,post, web, App, HttpServer, Result, Responder};
use tokio_postgres::{NoTls,Config};
//use tokio_postgres::Client;
//use tokio_postgres::types::Json;
use serde_json::{Value, Map};
use std::fmt::Display;

pub mod serializers;

// use serializers::Dtype;
// use serializers::Pseres;







#[post("/rust_test")]
async fn submit(info:web::Json<serializers::Dtype>)-> Result<impl Responder,Box<dyn std::error::Error>>{
//------------------------------------
let vic = serializers::Pseres{
    api_key:info.api_key.to_string(),
    provider_id:info.provider_id,
};

// Ok(web::Json(vic))
//---------------------------------------
let mut config = Config::new();
    config.host("192.168.10.211");
    config.user("postgres");
    config.password("GiChn32*");
    config.dbname("RoanuzFantasy");

    // Establish a connection
    let (_client, connection) = config.connect(NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = _client.query("Select json_agg(t)::text from udf_getupcomingmatches(1) t", &[]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: Value = serde_json::from_str(&json_string_value)?;
    // println!("here first match is {}",parsed[0]["matchid"]);
    Ok(web::Json(parsed))
    // Ok(web::Json(vic))
}