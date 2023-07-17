use tokio_postgres::{NoTls,Config};
use std::env;
use dotenv::dotenv;    


pub async fn get_otp_sp(mobileno:String,provider_id:i32,otp_flag:String) -> Result<String, Box<dyn std::error::Error>> {
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

    let qry:String = format!("Select json_agg(t)::text from udf_get_OTP('{}',{},'{}') t;", mobileno, provider_id, otp_flag);
    println!("{}",qry);
    let rows = _client.query(&qry, &[]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: String = json_string_value.clone();
    Ok(parsed)
}


pub async fn player_creation_sp(player_name:String,mobileno:String,flag:i32,provider_id:i32) -> Result<String, Box<dyn std::error::Error>> {
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

    let qry:&str ="Select json_agg(t)::text from udf_Playercreation($1,$2,$3,$4) t";
    let rows = _client.query(qry, &[&player_name,&mobileno,&flag,&provider_id]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: String = json_string_value.clone();
    Ok(parsed)
}

pub async fn player_login_sp(mobileno:String,otp:String,provider_id:i32) -> Result<String, Box<dyn std::error::Error>> {
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

    let qry:&str ="Select json_agg(t)::text from udf_PlayerLogin($1,$2,$3) t";
    let rows = _client.query(qry, &[&mobileno,&otp,&provider_id]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: String = json_string_value.clone();
    Ok(parsed)
}



pub async fn get_upcoming_match_sp(provider_id:i32) -> Result<String, Box<dyn std::error::Error>> {
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

    let qry:&str ="Select json_agg(t)::text from udf_GetUpComingMatches($1) t";
    let rows = _client.query(qry, &[&provider_id]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: String = json_string_value.clone();
    Ok(parsed)
}


pub async fn get_upcoming_match_fav_sp(provider_id:i32) -> Result<String, Box<dyn std::error::Error>> {
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

    let qry:&str ="Select json_agg(t)::text from udf_getupcomingfavTour($1) t";
    let rows = _client.query(qry, &[&provider_id]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: String = json_string_value.clone();
    Ok(parsed)
}


pub async fn get_match_key_sp(matck_key:String,provider_id:i32) -> Result<String, Box<dyn std::error::Error>> {
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

    let qry:&str ="Select json_agg(t)::text from udf_getmatchkey($1,$2) t";
    let rows = _client.query(qry, &[&matck_key,&provider_id]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: String = json_string_value.clone();
    Ok(parsed)
}


pub async fn get_contests_sp(match_id:String,provider_id:i32) -> Result<String, Box<dyn std::error::Error>> {
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

    let qry:&str ="Select json_agg(t)::text from udf_GetContests($1,$2) t";
    let rows = _client.query(qry, &[&match_id,&provider_id]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: String = json_string_value.clone();
    Ok(parsed)
}


pub async fn get_rank_winning_sp(match_id:String,contest_id:i32,provider_id:i32) -> Result<String, Box<dyn std::error::Error>> {
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

    let qry:&str ="Select json_agg(t)::text from udf_RankWinning($1,$2,$3) t";
    let rows = _client.query(qry, &[&match_id,&contest_id,&provider_id]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: String = json_string_value.clone();
    Ok(parsed)
}


pub async fn leaderboard_list_sp(match_id:String,contest_id:i32,provider_id:i32) -> Result<String, Box<dyn std::error::Error>> {
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

    let qry:&str ="Select json_agg(t)::text from udf_leaderBoardlist($1,$2,$3) t";
    let rows = _client.query(qry, &[&match_id,&contest_id,&provider_id]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: String = json_string_value.clone();
    Ok(parsed)
}


pub async fn team_create_sp(mobileno:String,match_id:String,user_selection:String,security_key:String,provider_id:i32,team_id:String) -> Result<String, Box<dyn std::error::Error>> {
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

    let qry:&str ="Select json_agg(t)::text from udf_ins_teamcreate($1,$2,$3,$4,$5,$6) t";
    let rows = _client.query(qry, &[&mobileno,&match_id,&user_selection,&security_key,&provider_id,&team_id]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: String = json_string_value.clone();
    Ok(parsed)
}


pub async fn get_myteam_sp(mobileno:String,match_key:String,provider_id:i32) -> Result<String, Box<dyn std::error::Error>> {
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

    let qry:&str ="Select json_agg(t)::text from udf_get_myteam($1,$2,$3) t";
    let rows = _client.query(qry, &[&mobileno,&match_key,&provider_id]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: String = json_string_value.clone();
    Ok(parsed)
}



pub async fn my_contest_sp(mobileno:String,match_id:String,provider_id:i32) -> Result<String, Box<dyn std::error::Error>> {
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

    let qry:&str ="Select json_agg(t)::text from udf_mycontest($1,$2,$3) t";
    let rows = _client.query(qry, &[&mobileno,&match_id,&provider_id]).await?;
    let dummy:Option<&str>= rows[0].get(0);
    let mut json_string_value= String::new();
    if let Some(value) = dummy {
        json_string_value= value.to_string();
    }
    // println!("{}",json_string_value);

    let parsed: String = json_string_value.clone();
    Ok(parsed)
}


