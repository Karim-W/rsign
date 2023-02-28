#[macro_use]
extern crate rocket;

use rocket::http::{ContentType, Status};
use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

struct Token {
    token: String,
}
impl Token {
    fn sign_token(&self) -> String {
        //TO-DO: sign token
        self.token.clone()
    }
}

#[get("/")]
fn index() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::JSON, "{ \"status\": \"alive\" }"))
}

#[post("/", data = "<token>")]
fn token_string(token: Json<Token>) -> String {
    token.sign_token()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1/token/sign", routes![token_string])
}
