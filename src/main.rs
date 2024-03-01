use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::fs;


#[get("/Stylesheet.css")]
async fn get_css() -> impl Responder {
    let css = fs::read_to_string("/var/www/Stylesheet.css").expect("Cannot read CSS file");
    HttpResponse::Ok()
        .content_type("text/css")
        .body(css)
}

#[get("/")]
async fn index() -> impl Responder {
    let data = fs::read_to_string("/var/www/index.html").expect("Cannot read CSS file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_css)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
