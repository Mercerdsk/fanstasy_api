use actix_web::{post, web,Result, Responder,http::header,HttpRequest,FromRequest};
use serde_json::Value;
use std::fmt::Display;
use crate::models::deserializer_struct::*;
use crate::repository::postgres_repo::*;
use crate::models::serializer_struct::Info;
use crate::logger::req_res_function::req_res_log;
use chrono::{Utc, TimeZone};

#[post("/get_otp/")]
// async fn  get_otp_handler(info:web::Json<getotp_model>)-> Result<impl Responder>{
async fn  get_otp_handler(info:web::Json<GetOtpModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    println!("{:?}",req.headers().get("APIKEY").unwrap());
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get otp";
    println!("{:?}",info);
    let data = serde_json::to_string(&info).expect("Failed to serialize JSON");
    req_res_log(method,req_stamp,"REQ",data).await;
    let api_key_var:String=info.api_key.to_string();
    let mobileno_var:String=info.mobileno.to_string();
    let provider_id_var:i32=info.provider_id;
    let otp_flag_var:String=info.otp_flag.to_string();
    let res= get_otp_sp(mobileno_var,provider_id_var,otp_flag_var,method,req_stamp).await;
    match res {
        Some(res_string)=>{
            req_res_log(method,req_stamp,"REQ",res_string.to_string()).await;
            let parsed: Value = serde_json::from_str(&res_string)?;
            return Ok(web::Json(parsed))
        }
        None=>{
            // println!("{}",e);
            let parsed: Value = serde_json::from_str("{\"resid\":1,\"resdesc\":\"Internal Server Error\"}")?;
            return Ok(web::Json(parsed))}
    } 
}
