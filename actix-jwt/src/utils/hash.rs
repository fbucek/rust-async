use argon2::{self, Config, Error};

type Result<T> = std::result::Result<T, Error>;

pub static SALT: [u8; 10] = *include_bytes!("../../salt.key");


/// Argon2 hashing method using salt
pub fn argon_hash<'a>(password: &'a [u8]) -> Result<String> {
    // Hash password
    let salt = SALT;
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config)
}
