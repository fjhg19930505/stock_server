use actix_web::{
    web::{post, scope, Path},
    HttpResponse, Scope,
};
use std::fs::read_to_string;

pub fn register_email_routes() -> Scope {
    scope("/user").route("/login", post().to(user_login))
}

fn user_login(info: Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("登录成功")
}