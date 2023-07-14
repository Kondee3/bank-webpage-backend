#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}



#[derive(Serialize, Deserialize)]
struct User {
    id: usize,
    user_name: String,
    email: String,
    date_of_birth: String,
    age: u32
}


#[get("/get")]
//Vec<User> if more values than 1
// let mut v: Vec<User> = vec![];
// v.push(user);
fn get_user() -> Json<User> {
    Json( User{
        id: 0,
        user_name: "Konrad".to_string(),
        email: "smyrakkonrad@gmail.com".to_string(),
        date_of_birth: "2000-07-05".to_string(),
        age: 23,
    })


}


#[launch]
fn rocket() -> Rocket<Build> {

    rocket::build()
        .mount("/api/v1/user", routes![get_user])
        .attach(CORS)
}


