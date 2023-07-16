use diesel::associations::HasTable;
use diesel::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::http::{Cookie, CookieJar};
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct Error {
    message: String,
    // Outros campos opcionais podem ser adicionados aqui
}

fn generate_token() -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now() + chrono::Duration::seconds(3600);
    let my_claims = Claims {
        sub: "apstech.info".to_owned(),
        exp: expiration.timestamp() as usize, // Expira em 10 bilhões de segundos (aproximadamente 317 anos)
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("QDtAQDMzMzRkJSUlIyMrPT09eGN4Y3hAO0BAMzMzNGQlJSUjIys9PT14Y3hjeEA7QEAzMzM0ZCUlJSMjKz09PXhjeGN4".as_ref()),
    )?;

    Ok(token)
}

pub struct AuthGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");

        match token {
            Some(token) => {
                let token = token.replace("Bearer ", "");
                let decoding_key = DecodingKey::from_secret("QDtAQDMzMzRkJSUlIyMrPT09eGN4Y3hAO0BAMzMzNGQlJSUjIys9PT14Y3hjeEA7QEAzMzM0ZCUlJSMjKz09PXhjeGN4".as_ref());
                //let json_error = json!({"Unauthorized"});
                match decode::<Claims>(&token, &decoding_key, &Validation::default()) {
                    Ok(_) => Outcome::Success(AuthGuard),
                    Err(_) => Outcome::Failure((Status::Unauthorized, ())),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

#[get("/unauthorized")]
pub fn unauthorized() -> std::string::String {
    let string_value = r#"["Unauthorized"]"#.to_string();
    let result: Result<std::string::String, ()> = Result::Ok(string_value);
    result.expect("REASON").to_string()
}

#[post("/login")]
pub fn login(cookies: &CookieJar<'_>) -> Result<status::Custom<String>, status::Custom<String>> {
    match generate_token() {
        Ok(token) => {
            cookies.add(Cookie::new("token", token.to_string()));
            let data = json!({
                "token": token.to_string(),
                "status": "OK"
            });

            Ok(status::Custom(Status::Ok, data.to_string()))
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Failed to generate token".to_string(),
        )),
    }
}

use crate::database::database;
use crate::models::models::*;

// usuarios
#[get("/users")]
pub fn users(_auth_guard: AuthGuard) -> Json<Vec<Users>> {
    use crate::schemas::schema::users::dsl::users;
    let connection = &mut database::establish_connection();
    users
        .load::<Users>(connection)
        .map(Json)
        .expect("Error loading users")
}

// times
#[get("/teams")]
pub fn teams(_auth_guard: AuthGuard) -> Json<Vec<Teams>> {
    use crate::schemas::schema::teams::dsl::teams;
    let connection = &mut database::establish_connection();
    teams
        .load::<Teams>(connection)
        .map(Json)
        .expect("Error loading teams")
}

// novo usuario
//novo time
#[post("/newuser", data = "<sighting>")]
pub fn new_user(_auth_guard: AuthGuard, sighting: Json<New_Users>) -> Json<Users> {
    use crate::schemas::schema::users;

    let connection = &mut database::establish_connection();
    diesel::insert_into(users::table)
        .values(sighting.into_inner())
        .execute(connection)
        .expect("Error adding sighting");

    Json(
        users::table
            .order(users::user_id.desc())
            .first(connection)
            .unwrap(),
    )
}

//novo time
#[post("/newteam", data = "<sighting>")]
pub fn new_team(_auth_guard: AuthGuard, sighting: Json<New_Teams>) -> Json<Teams> {
    use crate::schemas::schema::teams;

    let connection = &mut database::establish_connection();
    diesel::insert_into(teams::table)
        .values(sighting.into_inner())
        .execute(connection)
        .expect("Error adding sighting");

    Json(
        teams::table
            .order(teams::team_id.desc())
            .first(connection)
            .unwrap(),
    )
}

#[catch(404)]
pub fn not_found() -> String {
    r#"{"error": 404,"message": "Not Found"}"#.to_string()
}

#[catch(401)]
pub fn not_autorized() -> String {
    r#"{"error": 401,"message": "Unauthorized"}"#.to_string()
}
