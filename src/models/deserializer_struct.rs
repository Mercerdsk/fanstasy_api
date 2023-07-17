use serde::Deserialize;


#[derive(Deserialize)]
pub struct getotp_model{
pub    api_key: String,
pub    mobileno: String,
pub   provider_id: i32,
pub   otp_flag: String,
}

#[derive(Deserialize)]
pub struct player_creation_model{
    pub    api_key: String,
    pub    player_name: String,
    pub    mobileno: i32,
    pub    flag: i32,
    pub    provider_id: i32,
}

#[derive(Deserialize)]
pub struct player_login_model{
    pub    api_key:String,
    pub    mobileno:i32,
    pub    otp:String,
    pub    provider_id:i32,
}

#[derive(Deserialize)]
pub struct get_upcoming_match_model{
    pub    api_key:String,
    pub    provider_id:i32,
}

#[derive(Deserialize)]
pub struct get_match_key_model{
    pub    api_key:String,
    pub    match_key:String,
    pub    provider_id:i32,
}

#[derive(Deserialize)]
pub struct get_contests_model{
    pub    api_key:String,
    pub    match_id:String,
    pub    provider_id:i32,
}

#[derive(Deserialize)]
pub struct rank_winning_model{
    pub    api_key:String,
    pub    match_id:String,
    pub    contest_id:i32,
    pub    provider_id:i32,
}

#[derive(Deserialize)]
pub struct leader_board_model{
    pub    api_key:String,
    pub    match_id:String,
    pub    contest_id:i32,
    pub    provider_id:i32,
}

#[derive(Deserialize)]
pub struct match_create_model{
    pub    api_key:String,
    pub    mobileno:String,
    pub    match_id:String,
    pub    user_selection_info:String,
    pub    security_key:String,
    pub    provider_id:i32,
    pub    team_id:String,
}

#[derive(Deserialize)]
pub struct get_my_matches_model{
    pub    mobileno:String,
    pub    match_key:String,
    pub    provider_id:i32,
}

#[derive(Deserialize)]
pub struct my_contest_model{
    pub    mobileno:String,
    pub    match_id:String,
    pub    provider_id:i32,
}