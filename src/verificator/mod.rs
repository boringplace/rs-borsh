pub mod method;

/* Various verification types */
mod email;
mod sms;

use crate::user::User;
use verificator::method::AuthMethod;

pub fn check(user: &User) -> bool {
    match user.auth_method() {
        AuthMethod::Email => email::check(user),
        AuthMethod::SMS   => sms::check(user),
    }
}

