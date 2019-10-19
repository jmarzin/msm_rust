extern crate actix_web;
use actix_web::{http::Method, server, App};

pub mod controllers;

fn main() {


    server::new(|| App::new()
        .route("/traces", Method::GET, controllers::traces::index)
        .route("/trace/new", Method::GET, controllers::traces::new)
        .resource(
            "/trace/{id}",|r| r.method(Method::GET).with(controllers::traces::show))
        .resource(
            "/trace/{id}/edit",|r| r.method(Method::GET).with(controllers::traces::edit))
        .route("/treks", Method::GET, controllers::treks::index)
        .route("/trek/new", Method::GET, controllers::treks::new)
        .resource(
            "/trek/{id}", |r| r.method(Method::GET).with(controllers::treks::show))
        .resource(
            "/trek/{id}/edit", |r| r.method(Method::GET).with(controllers::treks::edit))
        .finish())
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
