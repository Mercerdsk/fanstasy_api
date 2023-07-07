use actix_web::{get,post, web, App, HttpServer, Result, Responder};
use serde::{Deserialize, Serialize};

use tokio_postgres::{NoTls,Config};
//use tokio_postgres::Client;
//use tokio_postgres::types::Json;
use serde_json::{Value, Map};
use std::fmt::Display;



struct AppState {
    app_name: String,
}

#[derive(Deserialize)]
struct Dtype{
api_key: String,
provider_id: u32,
}

#[derive(Serialize)]
struct Pseres{
    api_key: String,
    provider_id: u32,
}


#[post("/posting")]
async fn submit(info:web::Json<Dtype>)-> Result<impl Responder,Box<dyn std::error::Error>>{
//------------------------------------
let vic = Pseres{
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
	    .service(submit)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
