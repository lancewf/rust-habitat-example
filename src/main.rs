extern crate actix_web;
extern crate mysql;
#[macro_use]
extern crate serde_derive;
use actix_web::{HttpServer, App, web, HttpRequest, HttpResponse};
use config::Config;

#[derive(Debug, Deserialize)]
struct Settings {
  port: i16,
  bind: String,
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

// Here is the handler, 
// we are returning a json response with an ok status 
// that contains the text Hello World
fn index(_req: HttpRequest) -> HttpResponse  {
    HttpResponse::Ok().json("Hello world!")
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

    // We are creating an Application instance and 
    // register the request handler with a route and a resource 
    // that creates a specific path, then the application instance 
    // can be used with HttpServer to listen for incoming connections.
    match HttpServer::new(|| App::new().service(
             web::resource("/").route(web::get().to_async(index))))
        .bind(connection_string)
        .unwrap()
        .run() {
            Ok(()) => println!("called !"),
            Err(e) => println!(" error {}", e),
        };
}
