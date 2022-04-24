extern crate actix_web;
extern crate mysql;
#[macro_use]
extern crate serde_derive;
use actix_web::{get, web, App, HttpServer, Responder};
use config::Config;

#[derive(Debug, Deserialize)]
struct Settings {
  port: i16,
  bind: String,
  message: String,
  database: Database,
}

#[derive(Debug, Deserialize)]
struct Database {
    user: String,
    password: String,
    bind: String,
    port: i16,
    name: String,
}

#[get("/{id}/{name}/index.html")]
async fn index(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    format!("Hello {}! id:{}", name, id)
}

fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("config/config"))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap();

    let database_connection_string = format!("mysql://{}:{}@{}:{}/{}", 
        config.database.user, config.database.password, config.database.bind, 
        config.database.port, config.database.name);
    println!("database_connection_string: {}", database_connection_string);

    let pool = mysql::Pool::new(database_connection_string).unwrap();

    pool.prep_exec(r"CREATE TABLE if not exists payment (
        customer_id int not null,
        amount int not null,
        account_name text
    )", ()).unwrap();

    let connection_string = format!("{}:{}", config.bind, config.port);
    println!("connection_string: {}", connection_string);

    start_server(connection_string);
}

#[actix_web::main]
async fn start_server(connection_string: String) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(connection_string)?
        .run()
        .await
}

