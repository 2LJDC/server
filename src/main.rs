//use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{put, get, App, Error, HttpResponse, HttpServer, Responder};
//use actix_files as afs;
//use std::fs;
//use std::io::BufReader;
//use std::io::Read;

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
    //let css = fs::read_to_string("/var/www/style.css").expect("Cannot read CSS file");
    let css = std::fs::read("/var/www/style.css").expect("Cannot read CSS file");
    HttpResponse::Ok()
        .content_type("text/css")
        .body(css)
}

#[get("/srv-pic-2.webp")]
async fn get_pic() -> impl Responder {
    let data = std::fs::read("/var/www/srv-pic-2.webp").expect("Cannot read webp file");
    HttpResponse::Ok()
        .content_type("image/webp")
        .body(data)
}
    /*
    let file = afs::NamedFile::open("/var/www/srv-pic-2.webp").expect("Cannot read webp file");
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
    */

#[get("/formular.html")]
async fn formular() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/var/www/formular.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}
#[get("/formular.js")]
async fn formularjs() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/var/www/formular.js").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data)
}
#[get("/")]
async fn index() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/var/www/2LJDC.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}





#[put("/submit")]
async fn submit(req_body: String) -> impl Responder {
	println!("{}", req_body);
    HttpResponse::Ok()
}

/*
#[get("/")]
async fn index() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/var/www/index.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}
*/




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //.service(afs::Files::new("/style.css", "/var/www/style.css"))
            //.service(afs::Files::new("/srv-pic-2.webp", "/var/www/srv-pic-2.webp"))
            .service(get_pic)
            .service(get_css)
            .service(index)
            .service(submit)
	    .service(formular)
	    .service(formular.js)
	    
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
