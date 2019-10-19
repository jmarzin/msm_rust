use actix_web::{Path, HttpRequest, HttpResponse};

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("Liste des treks")
}

pub fn new(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("Nouveau trek")
}

pub fn show(id: Path<u32>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Le trek numero {}",id))
}

pub fn edit(id: Path<u32>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Edition du trek numero {}",id))
}