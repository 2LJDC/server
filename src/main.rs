use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, App, Error, HttpResponse, HttpServer, Responder};
use actix_files as afs;
use std::fs;
use std::io::BufReader;
use std::io::Read;

/*
#[get("/Stylesheet.css")]
async fn get_css() -> impl Responder {
    let css = fs::read_to_string("/var/www/Stylesheet.css").expect("Cannot read CSS file");
    HttpResponse::Ok()
        .content_type("text/css")
        .body(css)
}
*/

#[get("/style.css")]
async fn get_css() -> impl Responder {
    let css = fs::read_to_string("/var/www/style.css").expect("Cannot read CSS file");
    HttpResponse::Ok()
        .content_type("text/css")
        .body(css)
}

#[get("/srv-pic-2.webp")]
async fn get_pic() -> Result<afs::NamedFile, Error> {
    let file = std::fs::read("/var/www/srv-pic-2.webp").expect("Cannot read webp file");
        HttpResponse::Ok()
        .content_type("image/webp")
        .body(data)
    /*
    let file = afs::NamedFile::open("/var/www/srv-pic-2.webp").expect("Cannot read webp file");
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
    */
}

#[get("/")]
async fn index() -> impl Responder {
    let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
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
