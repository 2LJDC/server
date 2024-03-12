
use actix_web::{put, get, App, HttpResponse, HttpServer, Responder};
use std::process::Command;
use actix_files as fs;


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

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
	let settings = config::Config::builder()
		.add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml))
		.build()?;
	settings.try_deserialize::<Settings>()
}





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
	let s = req_body.replace("#", "");
	let data = json::parse(&s).unwrap();
	//println!("{}{}", data["name"], data["mail"]);
	add_customer(data);

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

// postgres
async fn add_customer(customer, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
	let query = "INSERT INTO kunde (anrede, name, geburtsdatum, mail, tel, vorlage, farbe, eigeneVorstellungen, sonstiges) VALUES ($1, $2, $3)";

	sqlx::query(query)
		.bind(customer[""])
		.bind(customer[""])
		.bind(customer[""])
		.execute(pool)
		.await?;
	
	Ok(())
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
