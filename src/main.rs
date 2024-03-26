
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::process::Command;
use actix_files as fs;
use std::error::Error as stdError;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

//use actix_web::HttpRequest;
use actix_web::Error;


//config
#[derive(serde::Deserialize)]
pub struct Settings{
	pub database: DatabaseSettings,
	pub application_port: u16

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
	let configuration = match get_configuration() {
		Ok(c) => c,
		Err(_) => return HttpResponse::BadRequest(),
	};

	let url = configuration.database.connection_string();
	
	match add_customer(req_body, url).await {
		Ok(()) => HttpResponse::Ok(),
		Err(_) => HttpResponse::BadRequest(),
	};

	HttpResponse::Ok()
}

// DATABASE postgres
async fn add_customer(c_string: String, url: String) -> Result<(), Box<dyn stdError>> {
//fn add_customer(c_string: String, url: String) -> Result<(), Error> {

	let s = c_string.replace("#", "");
	let customer = json::parse(&s).unwrap();
	
	//let pool = sqlx::postgres::PgPool::connect(&url).await?;
	let pool = match sqlx::postgres::PgPool::connect(&url).await {
		Ok(p) => p,
		Err(e) => return Err(Box::new(e)),
	};


	
	let query = "INSERT INTO kunde (anrede, name, geburtsdatum, mail, tel, vorlage, farbe, eigeneVorstellungen, sonstiges) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";

	match sqlx::query(query)
		.bind(&customer["anrede"].to_string())
		.bind(&customer["name"].to_string())
		.bind(&customer["geburtstag"].to_string())
		.bind(&customer["mail"].to_string())
		.bind(&customer["tel"].to_string())
		.bind(&customer["vorlage"].to_string())
		.bind(&customer["farbe"].to_string())
		.bind(&customer["eigeneVorstellungen"].to_string())
		.bind(&customer["sonstiges"].to_string())
		.execute(&pool).await {
			Ok(_) => Ok(()),
			Err(e) => Err(Box::new(e)),
		}

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //let configuration = get_configuration().expect("Failed to read config");
    //let address = format!("{}:{}", configuration.database.host, configuration.database.port);
    //let address = configuration.database.connection_string();
    //println!("databse: {}", address);

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    
	builder.set_certificate_chain_file("cert.pem").unwrap();

	HttpServer::new(|| {
		App::new()
			.route("/status", web::get().to(status))
			.route("/", web::get().to(index))
			.route("/submit", web::get().to(submit))
			.route("/update", web::get().to(update))
			.service(fs::Files::new("/", "/app/www"))
			.default_service(web::get().to(index))
	    
    })
    .bind_openssl("0.0.0.0:8000", builder)?
    .run()
    .await
}
