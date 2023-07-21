use reqwest::{Client, Error};
use std::collections::HashMap;

pub async fn req_res_log(method:&str,_reqid:&str,logroot:&str,data:String) {
    let api_url = "http://192.168.10.156:8007/get_otp/"; // Replace with your API URL
    let client = Client::new();
    let mut json_body = HashMap::new();
    
    json_body.insert("short_message",method.to_string());
    json_body.insert("host","fantasy_api".to_string());
    // json_body.insert("_ip",client_ip);
    json_body.insert("_reqid",_reqid.to_string());
    json_body.insert("facility","0".to_string());
    json_body.insert("_logroot",logroot.to_string());
    json_body.insert("_message","logroot".to_string());
    json_body.insert("full_message",data);

    // JSON data to send in the request body (replace with your data)

    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .json(&json_body)
        .send()
        .await;
    
    let body = response.unwrap().text().await;
    match body {
        Ok(res)=> {println!("{}",res)},
        Err(e)=> {println!("{}",e)}
    }
}