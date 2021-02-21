use pest::Parser;
use pest_derive::*;
use rand::{RngCore, Rng};

pub fn get_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

#[derive(Parser)]
#[grammar = "email_address.pest"]
struct Rules;

#[derive(Debug)]
pub struct EmailAddress {
    pub local: String,
    pub domain: String,
}

pub fn parse_email(input: &str) -> Option<EmailAddress> {
    match Rules::parse(Rule::address, input) {
        Ok(mut parsed) => {
            let mut parsed = parsed.next().unwrap().into_inner();
            Some(EmailAddress {
                local: String::from(parsed.next().unwrap().as_str()),
                domain: String::from(parsed.next().unwrap().as_str()),
            })
        },
        Err(_) => None,
    }
}

pub fn generate_passkey(username: &str) -> String {
    use sha2::{Sha256, Digest};
    use std::convert::TryInto;

    let mut hasher = Sha256::new();
    let mut rng = rand::thread_rng();
    hasher.update(
        format!("{}{}{}", username, get_timestamp(), rng.next_u64())
    );

    let res: Vec<u8> = hasher.finalize().as_slice().try_into().expect("Damn!");
    String::from_utf8_lossy(&*res).into_owned()
}

pub fn hash_password(password: &str) -> String {
    let salt: [u8; 20] = rand::thread_rng().gen();
    let config = argon2::Config::default();

    argon2::hash_encoded(password.as_ref(), &salt, &config).expect("Damn!")
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    argon2::verify_encoded(hash, password.as_ref()).expect("Damn!")
}
