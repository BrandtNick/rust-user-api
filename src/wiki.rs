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


#[derive(Serialize, Deserialize, Debug)]
pub struct Wiki {
    page: String,
    title: String,
    subtitle: String,
    article: String,
    references: String
}

pub fn wiki_collection() -> mongodb::coll::Collection {
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");
    client.db("nexus").collection("wiki")
}

pub fn post_wiki_page(wiki: Json<Wiki>) -> Result<String> {

}

pub fn post_wiki_article(wiki: Json<Wiki>) -> Result<String> {

}

pub fn get_wiki(wiki: Json<Wiki>) -> Result<String> {
    
}