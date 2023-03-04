#[macro_use]
extern crate rocket;
use std::collections::HashMap;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rocket::http::{ContentType, Status};
use rocket::serde::{json::Json, Deserialize,Serialize};

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}


#[derive(Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
    dat: HashMap<String, String>,
    iss: String,
    iat: usize,
    nbf: usize,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Token {
    claims: Claims,
}
impl Token {
    fn sign_token(&self) -> TokenResponse {
    let header = Header::new(Algorithm::HS512);
    let token_string = encode(&header, &self.claims, &EncodingKey::from_base64_secret("bG9sb2xvbGlkZWttYW4=").unwrap()).unwrap();
    TokenResponse { token: token_string }
    }
}

#[get("/")]
fn index() -> (Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::JSON, "{ \"status\": \"alive\" }"))
}

#[post("/", data = "<token>")]
fn token_string(token: Json<Token>) -> Json<TokenResponse> {
    Json(token.sign_token())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1/token/sign", routes![token_string])
}
