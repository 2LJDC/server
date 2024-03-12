
use actix_web::{put, get, App, HttpResponse, HttpServer, Responder};
use std::process::Command;
use actix_files as fs;
use std::error::Error;



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
		format!("postgres://{}:{}@{}:{}/{}",self.username, self.password, self.host, self.port, self.database_name)
	}
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
	let configuration = get_configuration().expect("Failed to read config");
	let url = configuration.database.connection_string();
		
	match add_customer(req_body, url).await {
		Ok(()) => HttpResponse::Ok(),
		Err(_) => HttpResponse::Ok(),
	};

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
async fn add_customer(c_string: String, url: String) -> Result<(), Box<dyn Error>> {
	let s = c_string.replace("#", "");
	let customer = json::parse(&s).unwrap();
	println!("add:{},{}", customer["name"], customer["mail"]);
	
	//let url = "postgres://postgres:deeznuts@85.215.154.152:5432";
	let pool = sqlx::postgres::PgPool::connect(&url).await?;
	
	let query = "INSERT INTO kunde (anrede, name, geburtsdatum, mail, tel, vorlage, farbe, eigeneVorstellungen, sonstiges) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";
	println!("send data...");
	sqlx::query(query)
		.bind(&customer["anrede"].to_string())
		.bind(&customer["name"].to_string())
		.bind(&customer["geburtstag"].to_string())
		.bind(&customer["mail"].to_string())
		.bind(&customer["tel"].to_string())
		.bind(&customer["vorlage"].to_string())
		.bind(&customer["farbe"].to_string())
		.bind(&customer["eigeneVorstellungen"].to_string())
		.bind(&customer["sonstiges"].to_string())
		.execute(&pool)
		.await?;
	
	println!("done");

	Ok(())
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read config");
    //let address = format!("{}:{}", configuration.database.host, configuration.database.port);
    let address = format!("postgres://{}:{}@{}:{}/{}",self.username, self.password, self.host, self.port, self.database_name)
    println!("databse: {}", address);
	
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
