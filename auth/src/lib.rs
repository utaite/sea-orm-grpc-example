use crate::auth_type::AuthType;
use crate::claims::Claims;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::env;
use tonic::{codegen::http::header::AUTHORIZATION, Request, Status};

pub mod auth_type;
pub mod claims;
pub mod member_role;

pub fn jwt_interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    let bearer = AuthType::Bearer.as_ref();
    let access_token = req
        .metadata()
        .get(AUTHORIZATION.as_str())
        .and_then(|x| x.to_str().ok())
        .filter(|x| x.starts_with(bearer))
        .map(|x| x[bearer.len()..].trim());
    let token_data = access_token.and_then(|x| {
        let jwt_secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
        decode::<Claims>(x, &DecodingKey::from_secret(jwt_secret_key.as_ref()), &{
            let mut validation = Validation::new(Algorithm::HS256);
            validation.leeway = 0;
            validation
        })
        .ok()
    });

    if token_data.is_some() {
        Ok(req)
    } else {
        Err(Status::unauthenticated(
            "The access token provided is invalid",
        ))
    }
}
