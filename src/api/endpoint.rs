use actix_web::{post, web,Result, Responder};
use serde_json::Value;
use crate::models::deserializer_struct::*;
use crate::repository::postgres_repo::*;


#[post("/get_otp/")]
async fn  get_otp_handler(info:web::Json<getotp_model>)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let api_key_var:String=info.api_key.to_string();
    let mobileno_var:String=info.mobileno.to_string();
    let provider_id_var:i32=info.provider_id;
    let otp_flag_var:String=info.otp_flag.to_string();
    let res:String= get_otp_sp(mobileno_var,provider_id_var,otp_flag_var).await?;
    let parsed: Value = serde_json::from_str(&res)?;
    
    Ok(web::Json(parsed))
}

