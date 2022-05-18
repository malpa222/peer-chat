use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    error::Error,
    io::prelude::*,
    path::Path,
};

use serde::{Serialize, Deserialize};
use reqwest;

use crate::models::user::AuthUser;

macro_rules! getenv {
    ($a:expr) => {
        std::env::var($a).expect(format!("{} is not defined", $a).as_str())
    };
}

#[derive(Deserialize)]
#[serde(transparent)]
struct AuthRes {
    auth_users: Vec<AuthResUser>,
}

#[derive(Deserialize)]
struct AuthResUser {
    user_id: String,
    email: String,
    nickname: String
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

pub async fn get_user_by_id(id: &String) -> Result<Option<AuthUser>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!(r#"https://{}/api/v2/users?q=user_id:"{}"&include_fields=true&fields=user_id,email,nickname"#,
        getenv!("AUTH0_DOMAIN"),
        id);

    let res = client
        .get(url)
        .bearer_auth(get_access_token().await?)
        .send()
        .await?;
    
    match res.json::<AuthRes>().await?.auth_users.into_iter().next() {
        Some(user) => {
            Ok(Some(AuthUser {
                auth0_id: user.user_id,
                email: user.email,
                username: user.nickname
            }))
        },
        None => Err(Box::from("User not found"))
    }
}

pub async fn get_user_by_email(email: &String) -> Result<Option<AuthUser>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!(r#"https://{}/api/v2/users?q=email:"{}"&include_fields=true&fields=user_id,t email,nickname"#,
        getenv!("AUTH0_DOMAIN"),
        email);

    let res = client
        .get(url)
        .bearer_auth(get_access_token().await?)
        .send()
        .await?;

    match res.json::<AuthRes>().await?.auth_users.into_iter().next() {
        Some(user) => {
            Ok(Some(AuthUser {
                auth0_id: user.user_id,
                email: user.email,
                username: user.nickname
            }))
        },
        None => Err(Box::from("User not found"))
    }
}

pub async fn delete_user(id: &String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!(r#"https://{}/api/v2/users/{}"#,
        getenv!("AUTH0_DOMAIN"),
        id);

    let res = client
        .delete(url)
        .bearer_auth(get_access_token().await?)
        .send()
        .await?;

    if res.status() == 204 {
        return Ok(());
    }

    Err(Box::from("Cannot delete user"))
}

fn update_user_meta() {
    // update user in auth0
    todo!()
}