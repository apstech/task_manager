#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};

use rocket::http::Status;
use rocket::response::{Redirect, Responder};
use rocket::request::{self, FromRequest, Request,Outcome};
use rocket::State;
use rocket::http::{Cookie, CookieJar};
use rocket::response::status;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use serde::Serialize;
use serde::Deserialize;


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn generate_token() -> Result<String, jsonwebtoken::errors::Error> {
    let my_claims = Claims {
        sub: "user123".to_owned(),
        exp: 10000000000, // Expira em 10 bilhões de segundos (aproximadamente 317 anos)
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret_key".as_ref()),
    )?;

    Ok(token)
}

struct AuthGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");

        match token {
            Some(token) => {
                let token = token.replace("Bearer ", "");
                let decoding_key = DecodingKey::from_secret("secret_key".as_ref());

                match decode::<Claims>(&token, &decoding_key, &Validation::default()) {
                    Ok(_) => Outcome::Success(AuthGuard),
                    Outcome::Failure((Status::Unauthorized, Redirect::to("/error"))),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, Redirect::to("/error"))),
        }
    }
}

#[get("/error")]
fn error_page() -> Redirect {
    // Lógica para a página de erro
    Redirect::to("/error.html")
}

#[get("/protected")]
fn protected_route() -> Redirect {
    "This is a protected route!"
}

#[post("/login")]
fn login(cookies: &CookieJar<'_>) -> Result<status::Custom<String>, status::Custom<String>> {
    match generate_token() {
        Ok(token) => {
            cookies.add(Cookie::new("token", token.to_string()));
            Ok(status::Custom(Status::Ok, "Login successful".to_string()))
        }
        Err(_) => Err(status::Custom(Status::InternalServerError, "Failed to generate token".to_string())),
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![protected_route,login])
}
