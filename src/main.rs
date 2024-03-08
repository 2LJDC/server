//use actix_web::http::header::{ContentDisposition, DispositionType};
//use actix_web::{put, get, App, Error, HttpResponse, HttpServer, Responder};
use actix_web::{web, put, get, App, HttpResponse, HttpServer, Responder};
use std::process::Command;

use actix_files as afs;
//use std::fs;
//use std::io::BufReader;
//use std::io::Read;


#[get("/style.css")]
async fn get_css() -> impl Responder {
    //let css = fs::read_to_string("/var/www/style.css").expect("Cannot read CSS file");
    let css = std::fs::read("/var/www/style.css").expect("Cannot read CSS file");
    HttpResponse::Ok()
        .content_type("text/css")
        .body(css)
}

// favicon.ico
#[get("/graphics/favicon.ico")]
async fn get_icon() -> impl Responder {
    let data = std::fs::read("/var/www/graphics/favicon.ico").expect("Cannot read webp file");
    HttpResponse::Ok()
        .content_type("image/vnd.microsoft.icon")
        .body(data)
}

// Logo.png
#[get("/graphics/Logo.png")]
async fn get_logo() -> impl Responder {
    let data = std::fs::read("/var/www/graphics/Logo.png").expect("Cannot read webp file");
    HttpResponse::Ok()
        .content_type("image/vnd.microsoft.icon")
        .body(data)
}
// Logo2x.png
#[get("/graphics/Logo2x.png")]
async fn get_logo2x() -> impl Responder {
    let data = std::fs::read("/var/www/graphics/Logo2x.png").expect("Cannot read webp file");
    HttpResponse::Ok()
        .content_type("image/vnd.microsoft.icon")
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

#[get("/2LJDC.html")]
async fn index() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/var/www/2LJDC.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}

#[get("/library.js")]
async fn lib() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/var/www/library.js").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data)
}

#[get("/")]
async fn root() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/var/www/2LJDC.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}


// cookie cookies.js
#[get("/cookies.js")]
async fn cookies() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/var/www/cookies.js").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data)
}


// API
#[put("/submit")]
async fn submit(req_body: String) -> impl Responder {
	println!("{:?}", req_body);
    HttpResponse::Ok()
}
// UPDATE
#[put("/update")]
async fn update(req_body: String) -> impl Responder {
	if req_body == "kekw" {
		println!("update...");
		let mut cmd = Command::new("bash");
		let out = cmd.arg("-c").arg("update-www.sh").output().expect("failed to execute update");
		println!("{:?}", out);
	}
    HttpResponse::Ok()
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
	    //.service(fs::Files::new("/static", ".")
            //.service(afs::Files::new("/style.css", "/var/www/style.css"))
            //.service(afs::Files::new("/srv-pic-2.webp", "/var/www/srv-pic-2.webp"))
            .service(get_icon)
            .service(get_css)
            .service(index)
            .service(submit)
	    .service(formular)
	    .service(formularjs)
	    .service(root)
	    .service(lib)
	    .service(update)
	    .service(cookies)
	    //.service(get_logo)
	    //.service(get_logo2x)
	    .service(afs::Files::new("/", "/var/www"))

	    //.service(afs::Files::new("/var/www", "."))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
