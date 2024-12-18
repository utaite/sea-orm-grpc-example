use anyhow::Result;
use api::post::post_mod::post_api_client::PostApiClient;
use api::post::post_mod::{Post, PostPerPage};
use auth::auth_type::AuthType;
use auth::claims::Claims;
use auth::member_role::MemberRole;
use dotenv::dotenv;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::env;
use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{codegen::http::header::AUTHORIZATION, transport::Channel, Request};

#[tokio::test]
async fn main() -> Result<()> {
    dotenv().ok();
    let channel = Channel::from_static("http://0.0.0.0:50051")
        .connect()
        .await?;
    let access_token = {
        let jwt_duration: u64 = env::var("JWT_DURATION")?.parse()?;
        let jwt_secret_key = env::var("JWT_SECRET_KEY")?;

        [
            AuthType::Bearer.as_ref(),
            encode(
                &Header::new(Algorithm::HS256),
                &Claims {
                    sub: 1.to_string(),
                    role: MemberRole::User.as_ref().to_owned(),
                    exp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_secs()
                        .add(jwt_duration / 1000),
                },
                &EncodingKey::from_secret(jwt_secret_key.as_ref()),
            )?
            .as_str(),
        ]
        .join(" ")
    };
    let mut client = PostApiClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut()
            .insert(AUTHORIZATION.as_str(), access_token.parse().unwrap());
        Ok(req)
    });

    {
        let request = Request::new(Post {
            id: 1,
            title: "title".to_owned(),
            content: "content".to_owned(),
        });
        let response = client.add_post(request).await?;

        println!("{:#?}", response);
        println!("{:#?}", response.into_inner().id);
    }

    {
        let request = Request::new(PostPerPage { per_page: 10 });
        let response = client.get_posts(request).await?;

        println!("{:#?}", response);

        for post in response.into_inner().post.iter() {
            println!("{post:?}");
        }
    }

    Ok(())
}
