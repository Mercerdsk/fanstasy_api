use serde::Deserialize;
use serde::Serialize;


#[derive(Serialize,Deserialize,Debug)]
pub struct GetOtpModel{
pub    api_key: String,
pub    mobileno: String,
pub   provider_id: i32,
pub   otp_flag: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct PlayerCreationModel{
    pub api_key:String,
    pub player_name:String,
    pub mobileno:String,
    pub pwd:String,
    pub flag:i32,
    pub provider_id:i32,
    pub otp:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct PlayerLoginModel{
    pub api_key:String,
    pub mobileno:String,
    pub otp:String,
    pub provider_id:i32,
    pub flag:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct GetUpCommingMatchModel{
    pub api_key:String,
    pub provider_id:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct GetMatchKeyModel{
    pub api_key:String,
    pub match_key:String,
    pub provider_id:i32,
} 

#[derive(Serialize,Deserialize,Debug)]
pub struct GetContestsModel{
    pub api_key:String,
    pub match_id:String,
    pub contest_id:i32,
    pub provider_id:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct RankWinningModel{
    pub api_key:String,
    pub match_id:String,
    pub contest_id:i32,
    pub provider_id:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct LeaderBoardModel{
    pub  api_key:String,
    pub match_id:String,
    pub contest_id:i32,
    pub provider_id:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct MatchCreateModel{
    pub api_key:String,
    pub mobileno:String,
    pub match_id:String,
    pub user_selection_info:String,
    pub security_key:String,
    pub provider_id:i32,
    pub team_id:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct GetMyMatchModel{
    pub mobileno:String,
    pub match_key:String,
    pub provide_id:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct MyContestModel{
    pub mobileno:String,
    pub match_id:String,
    pub provider_id:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct UpdateTeamContestMappingModel{
    pub mobileno:String,
    pub match_id:String,
    pub team_ids:String,
    pub provider_id:i32,
    pub contest_id:i32,
    pub total_team_count:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ServerTimeModel{
    pub provider_id:i32,
    pub foo_int:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct MyMatchModel{
    pub mobileno:String,
    pub provider_id:i32,
    pub flag:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct PlayerInfoModel{
    pub match_key:String,
    pub provider_id:i32,
    pub flag:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct GetPlayerBalanceModel{
    pub mobileno:String,
    pub provider_id:i32,



}