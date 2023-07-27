use tokio_postgres::{NoTls,Config};
use std::env;
use dotenv::dotenv;    
use crate::repository::database_functions::db_connection;
use crate::logger::req_res_function::req_res_log;
use crate::logger::req_res_function::error_log;

// pub async fn get_otp_sp(mobileno:String,provider_id:i32,otp_flag:String) -> Result<String, Box<dyn std::error::Error>> {
pub async fn get_otp_sp(mobileno:String,provider_id:i32,otp_flag:String,method:&str,req_stamp:f64) -> Option<String> {    
    let _client =db_connection().await;
    match  _client{
            Ok(client_object) => {
                let qry:String = format!("Select json_agg(t)::text from udf_get_OTP('{}',{},'{}') t;", mobileno, provider_id, otp_flag);
                println!("{}",qry);
                req_res_log(method,req_stamp,"DB-IN",qry.to_string()).await;
                let rows = client_object.query(&qry, &[]).await;
                match rows{
                Ok(rows) =>{
                    let dummy:Option<&str>= rows[0].get(0);
                    let mut json_string_value= String::new();
                    if let Some(value) = dummy {
                        json_string_value= value.to_string();
                        req_res_log(method,req_stamp,"DB-OUT",json_string_value.to_string()).await;
                    }
                    let res = format!(r#"{{"result":{}}}"#,json_string_value);
                    return Some(res);
                }
                Err(e) => {
                    error_log(method,req_stamp,"ERROR",e.to_string()).await;
                    return None;
                }
                }
        
            },
            Err(e) => {
                error_log(method,req_stamp,"ERROR",e.to_string()).await;
                return None;
            }
    };
}

