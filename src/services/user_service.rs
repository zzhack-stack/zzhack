use once_cell::sync::Lazy;

pub struct UserService {}

pub struct User {}

impl UserService {
    pub fn new() -> UserService {
        UserService {}
    }

    pub fn get_profile() {}
}

pub static user_service: Lazy<UserService> = Lazy::new(|| UserService::new());
