use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::APPLICATION_JSON;
pub type Tweets = Respond<Tweets>;

#[derive(Debug, Deserialize, Serialize)]

pub struct Tweets {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
}

impl Tweet {
    pub fn new(message: String) -> self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

impl TweetRequest {
    pub fn to_tweet(&self) -> Option<Tweet>{
        match &self.message{
            Some(message) => Some(Tweet::new(Message.to_string())),
            None => None,
        }
    }
}

#[get("/tweets")]
pub async fn list() -> HttpResponse {
    let tweets = Tweets {results: vec![]};

    HttpResponse::Ok().content_type(APPLICATION_JSON).json(tweets);
}

#[post("/tweets")]
pub async fn create(tweet_req:Json<TweetRequest>) -> HttpResponse {
    HttpResponse::Created().content_type(APPLICATION_JSON).json(tweet_req.to_tweet())
}

#[get("/tweets/{id}")]

pub async fn get(path: Path<(String, )>) -> HttpResponse{
    let found_tweets : Option<Tweet> =None;
}