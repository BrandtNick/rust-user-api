// user.rs

/* 
    Modules
 */
extern crate mongodb;
extern crate serde_json;
use std::time::{SystemTime, UNIX_EPOCH};
use bson::Bson;
use jwt::{encode, Header, Algorithm};
use bcrypt::{hash, verify, DEFAULT_COST};
use actix_web::{HttpRequest, HttpResponse, Responder, Json, Error, Result};
use actix_web::http::header::HeaderValue;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use auth;


/* 
    Structs
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    password: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    token: String,
}

/* 
    Custom JSON Responders
 */
impl Responder for User {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}
impl Responder for Token {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

/* 
    User collection
 */
pub fn users_collection() -> mongodb::coll::Collection {
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");
    client.db("glaive").collection("users")
}


/* 
    User login function. 
    Generates and returns a json web token on success. 
    Uses HS512 algorithm for token.
 */
pub fn login_user(user: Json<User>) -> Result<Json<Token>> {
    let password = &user.password;
    let user = doc! {
        "username": &user.username
    };

    let coll = users_collection();
    let mut cursor = coll.find(Some(user.clone()), None).ok().expect("Failed to execute find");
    let mut hashed_password: String = "".to_owned();
    let item = cursor.next();

    match item {
        Some(Ok(doc)) => match doc.get("password") {
            Some(&Bson::String(ref password)) => hashed_password.push_str(password),
            _ => panic!("Expected password to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }

    // Get time and set date a week from now, for token
    let time = SystemTime::now();
    let time_in_seconds = time.duration_since(UNIX_EPOCH).expect("Failed to get time");
    let exp = time_in_seconds.as_secs()+604800;
    
    // Token
    let my_claims = auth::Claims {
        sub: "claim".to_owned(),
        exp: exp as i64
    };

    let secret = "123";
    let mut header = Header::default();
    header.alg = Algorithm::HS512;

    let token = match encode(&header, &my_claims, secret.as_ref()) {
        Ok(t) => t,
        Err(_) => format!("Token error")
    };
    
    let valid = verify(&password, &hashed_password).unwrap();
    if valid == true {
        Ok(Json(Token {
            token: token.to_string(),
        }))
    } else {
        Ok(Json(Token {
            token: "Wrong Password".to_string(),
        }))
    }
}

/* 
    Create a new user function and hash password
 */
pub fn post_user(user: Json<User>) -> Result<String> {
    let hashed = hash(&user.password, DEFAULT_COST).unwrap();
    let coll = users_collection();
    coll.insert_one(doc!{ "username": &user.username, "password": hashed }, None).unwrap();
    Ok(format!("Added user: {}", &user.username))
}