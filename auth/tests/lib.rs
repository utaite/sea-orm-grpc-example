use anyhow::Result;
use auth::auth_type::AuthType;
use auth::claims::Claims;
use auth::member_role::MemberRole;
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn jwt를_검증한다() -> Result<()> {
    dotenv().ok();
    let jwt_duration: u64 = env::var("JWT_DURATION")?.parse()?;
    let jwt_secret_key = env::var("JWT_SECRET_KEY")?;
    let claims = Claims {
        sub: 1.to_string(),
        role: MemberRole::User.as_ref().to_owned(),
        exp: SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .add(jwt_duration / 1000),
    };
    let bearer = AuthType::Bearer.as_ref();
    let access_token = [
        bearer,
        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(jwt_secret_key.as_ref()),
        )?
        .as_str(),
    ]
    .join(" ");
    println!("{access_token:?}");

    let token_data = decode::<Claims>(
        access_token[bearer.len()..].trim(),
        &DecodingKey::from_secret(jwt_secret_key.as_ref()),
        &{
            let mut validation = Validation::new(Algorithm::HS256);
            validation.leeway = 0;
            validation
        },
    )?;
    assert_eq!(claims, token_data.claims);

    Ok(())
}
