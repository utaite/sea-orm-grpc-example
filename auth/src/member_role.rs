use strum_macros::AsRefStr;

#[derive(AsRefStr)]
#[strum(serialize_all = "UPPERCASE")]
pub enum MemberRole {
    User,
}
