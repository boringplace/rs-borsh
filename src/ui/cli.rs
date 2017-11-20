use crate::user::User;
use crate::settings::Settings;

pub fn start(settings: &Settings) -> User {
    let new_user : User = User::new("Igor Chudov".to_string());
    return new_user;
}

