use actix_web::{Path, HttpRequest, HttpResponse};

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("Liste des traces")
}

pub fn new(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("Nouvelle trace")
}

pub fn show(id: Path<u32>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("La trace numero {}",id))
}

pub fn edit(id: Path<u32>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Edition de la trace numero {}",id))
}

