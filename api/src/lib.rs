use migration::sea_orm::DatabaseConnection;

pub mod post;

#[derive(Default)]
pub struct Api {
    pub connection: DatabaseConnection,
}
