use pest::Parser;
use pest_derive::*;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::RngCore;
use std::convert::TryInto;

pub fn get_timestamp() -> u64 {
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
    let mut hasher = Sha256::new();
    let mut rng = rand::thread_rng();
    hasher.update(
        format!("{}{}{}", username, get_timestamp(), rng.next_u64())
    );

    let res: [u8; 32] = hasher.finalize().as_slice().try_into().expect("Damn!");
    String::from_utf8_lossy(res.as_slice()).into_owned()
}
