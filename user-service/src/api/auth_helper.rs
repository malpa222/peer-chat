use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    error::Error,
    io::prelude::*,
    path::Path,
};

use serde::{Serialize, Deserialize};
use reqwest;
// use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

macro_rules! getenv {
    ($a:expr) => {
        std::env::var($a).expect(format!("{} is not defined", $a).as_str())
    };
}

// return path to the file containg access token
async fn get_access_token() -> Result<Box<String>, Box<dyn Error>> {
    #[derive(Serialize, Deserialize)]
    struct AuthPayload {
        access_token: Option<String>,
        expires_in: Option<u32>,
        scope: Option<String>,
        token_type: Option<String>,
    }

    if Path::new("./token.txt").exists() {
        let mut token = String::new();
        File::open("token.txt")?.read_to_string(&mut token)?;

        return Ok(Box::new(token));
    }

    let client = reqwest::Client::new();

    let url = format!("https://{}/oauth/token/", getenv!("AUTH0_DOMAIN"));
    let mut body = HashMap::new();
    body.insert("grant_type", "client_credentials".to_string());
    body.insert("client_id", getenv!("AUTH0_CLIENTID"));
    body.insert("client_secret", getenv!("AUTH0_CLIENT_SECRET"));
    body.insert("audience", format!("https://{}/api/v2/", getenv!("AUTH0_DOMAIN")));
    body.insert("scope", String::from("read:users"));

    let response = client.post(url)
        .json(&body)
        .send()
        .await?;

    let payload = response.json::<AuthPayload>().await?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("token.txt")?;

    match payload.access_token {
        Some(token) => {
            file.write_all(token.as_bytes())?;
            Ok(Box::from(token))
        },
        None => Err(Box::from("No token received"))
    }
}

pub async fn get_user_meta(email: &String) -> Result<Box<HashMap<String, String>>, Box<dyn Error>> {
    // get user metadata
    let client = reqwest::Client::new();
    let url = format!("https://{}/api/v2/users?q=email:\"{}\"", getenv!("AUTH0_DOMAIN"), email);

    let response = client
        .get(url)
        .bearer_auth(get_access_token().await?)
        .send()
        .await?;

    return Err(Box::from(response.text().await?));
}

fn update_user_meta() {
    // update user in auth0

    todo!()
}

fn delete_user() {
    // delete user from auth0 database

    todo!()
}