// main.rs

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate actix_web;
extern crate bcrypt;
extern crate jsonwebtoken as jwt;

use actix_web::{server, http, App};

mod user;
mod auth;

fn main() {
    let host = "http://localhost:8000";

    println!("Started API service at: {}", host);
    server::new(|| App::new()
        .resource("/user/create", |r| r.method(http::Method::POST).with(user::post_user))
        .resource("/user/auth", |r| r.method(http::Method::POST).with(user::login_user))
        .bind(host)
        .unwrap()
        .run();
}
