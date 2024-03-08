
use actix_web::{put, get, App, HttpResponse, HttpServer, Responder};
use std::process::Command;
use actix_files as fs;

// index
#[get("/")]
async fn index() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/var/www/2LJDC.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}


// ------API------

// submit
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
            .service(index)
            .service(submit)
	    .service(update)
	    .service(fs::Files::new("/", "/var/www"))
	    
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
