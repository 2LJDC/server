
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::process::Command;
use actix_files as fs;
//use std::error::Error as stdError;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

//use actix_web::HttpRequest;
use actix_web::Error;

use jason_to_postgres::add_customer;

//config
#[derive(serde::Deserialize)]
pub struct Settings{
	pub database: DatabaseSettings,
	pub application_port: u16,
	pub password: String

}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
	pub username: String,
	pub password: String,
	pub port: u16,
	pub host: String,
	pub database_name: String

}

impl DatabaseSettings {
	pub fn connection_string(&self) -> String {
		//format!("postgres://{}:{}@{}:{}/{}",self.username, self.password, self.host, self.port, self.database_name)
		format!("postgres://{}:{}@{}:{}",self.username, self.password, self.host, self.port)
	}
}

// get config
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
	let settings = config::Config::builder()
		.add_source(config::File::new("/app/configuration.yaml", config::FileFormat::Yaml))
		.build()?;
	settings.try_deserialize::<Settings>()
}


// index
async fn index() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("/app/www/index.html")?)
}


// status
async fn status() -> String {
    "Server is up and running.".to_string()
}

// ------API------


// UPDATE
async fn update(req_body: String) -> impl Responder {
	if req_body == "kekw" {
		println!("update...");
		let mut cmd = Command::new("bash");
		let out = cmd.arg("-c").arg("update-www.sh").output().expect("failed to execute update");
		println!("{:?}", out);
	}
    HttpResponse::Ok()
}


// submit
async fn submit(req_body: String) -> impl Responder {

	//get config
	let configuration = match get_configuration() {
		Ok(c) => c,
		Err(_) => return HttpResponse::BadRequest(),
	};

	let url = configuration.database.connection_string();
	
	match add_customer(req_body, &url).await {
		Ok(()) => HttpResponse::Ok(),
		Err(_) => HttpResponse::BadRequest(),
	};

	HttpResponse::Ok()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    
	builder.set_certificate_chain_file("cert.pem").unwrap();

	HttpServer::new(|| {
		App::new()
			.route("/status", web::get().to(status))
			.route("/", web::get().to(index))
			.route("/submit", web::put().to(submit))
			.route("/update", web::get().to(update))
			.service(fs::Files::new("/", "/app/www"))
			.default_service(web::get().to(index))
	    
    })
    .bind_openssl("0.0.0.0:8000", builder)?
    .run()
    .await
}
