use strum_macros::AsRefStr;

#[derive(AsRefStr)]
pub enum AuthType {
    Bearer,
}
