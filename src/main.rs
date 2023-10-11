use actix_web::{http, middleware::Logger};
use bank::services::controllers::*;  

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_cors::Cors;
    use actix_web::{App, HttpServer};
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::new("%a %r %s %b %{Rferer}i %{User-Agent}i %T"))
            .service(register_controller::register)
            .service(login_controller::login)
            .service(mail_controller::send_mail)
            .service(mail_controller::fetch_mails)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
