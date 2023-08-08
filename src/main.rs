use self::models::*;
use actix_web::{middleware::Logger, get, http, post, web::Json, Responder};
use diesel::prelude::*;
use hello_rocket::{schema::bank_users, *};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[get("/api/v1/user/get")]
//Vec<User> if more values than 1
// let mut v: Vec<User> = vec![];
// v.push(user);
async fn get_user() -> impl Responder {
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: "Konrad".to_string(),
        email: "smyrakkonrad@gmail.com".to_string(),
        dateofbirth: "2000-07-05".to_string(),
        age: 23,
        password: "123".to_string(),
    };

    Json(user)
}

#[get("/api/v1/user/getusers")]
//Vec<User> if more values than 1
// let mut v: Vec<User> = vec![];
// v.push(user);
async fn get_users() -> impl Responder {
    use self::schema::bank_users::dsl::*;
    let connection = &mut establish_connection();
    let results = bank_users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");
    Json(results)
}

#[derive(Serialize, Deserialize)]
struct UserForm{
    username_form: String,
    password_form: String,
}
#[post("/api/v1/user/add")]
async fn add_user( user_data: Json<UserForm>) -> impl Responder{
    let connection =&mut establish_connection();
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: user_data.username_form.to_string(),
        email: "smyrakkonrad@gmail.com".to_string(),
        dateofbirth: "2000-07-05".to_string(),
        age: 23,
        password: user_data.password_form.to_string(),
    };
    diesel::insert_into(bank_users::table)
        .values(&user)
        .execute(connection)
        .expect("Error saving new user");
    Json(user)
}

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
        App::new().wrap(cors)
            .wrap(Logger::new("%a %r %s %b %{Rferer}i %{User-Agent}i %T"))
            .service(get_user).service(add_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
