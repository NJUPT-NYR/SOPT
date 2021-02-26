use crate::error::{error_string, Error};
use pest::Parser;
use pest_derive::*;
use rand::{thread_rng, Rng, RngCore};

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
        }
        Err(_) => None,
    }
}

pub fn generate_passkey(username: &str) -> Result<String, Error> {
    use sha2::{Digest, Sha256};
    use std::convert::TryInto;

    let mut hasher = Sha256::new();
    let mut rng = rand::thread_rng();
    hasher.update(format!("{}{}{}", username, get_timestamp(), rng.next_u64()));

    let res: Vec<u8> = hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(error_string)?;
    let string = hex::encode(res);
    Ok(String::from(&string[..32]))
}

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt: [u8; 20] = rand::thread_rng().gen();
    let config = argon2::Config::default();

    argon2::hash_encoded(password.as_ref(), &salt, &config).map_err(error_string)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    argon2::verify_encoded(hash, password.as_ref()).map_err(error_string)
}

pub fn send_mail(
    receiver: String,
    address: String,
    from: String,
    body: String,
) -> Result<(), Error> {
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};
    use std::thread::sleep;

    let mail = Message::builder()
        .from(
            format!("{} <{}>", from, "brethland@gmail.com")
                .parse()
                .unwrap(),
        )
        .to(format!("{} <{}>", receiver, address).parse().unwrap())
        .subject("Invitation Code")
        .body(body)
        .map_err(error_string)?;

    let client = SmtpTransport::relay("smtp.gmail.com")
        .map_err(error_string)?
        .credentials(Credentials::new(
            "brethland@gmail.com".to_string(),
            "fake_pass".to_string(),
        ))
        .build();

    let mut retry_count = 5;

    while let Err(err) = client.send(&mail) {
        retry_count = retry_count - 1;
        sleep(std::time::Duration::from_secs(1));
        if retry_count == 0 {
            return Err(Error::OtherError(err.to_string()));
        }
    }

    Ok(())
}

pub fn generate_invitation_code() -> String {
    use rand::distributions::Alphanumeric;

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    format!("{}_{}", rand_string, get_timestamp()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    impl std::cmp::PartialEq for EmailAddress {
        fn eq(&self, other: &EmailAddress) -> bool {
            return self.local == other.local && self.domain == other.domain;
        }
    }
    #[test]
    fn parse_email_works() {
        assert_eq!(
            parse_email("cattchen@njupt.edu.cn"),
            Some(EmailAddress {
                local: String::from("cattchen"),
                domain: String::from("njupt.edu.cn")
            })
        );
        assert_eq!(parse_email("just_an_invalid_string"), None);
        assert_eq!(parse_email(""), None);
    }
    #[test]
    fn hash_and_verify_passwords_works() {
        let hash_and_verify_password_works = |password: &str| {
            let hash = hash_password(password).unwrap();
            if !verify_password(password, hash.as_str()).unwrap() {
                panic!("Invalid Verify");
            }
        };
        let passwords = [
            "`1234567890-=~!@#$%^&*()_+",
            "qwertyuiop[]\\QWERTYUIOP{}|",
            "asdfghjkl;'ASDFGHJKL:\"",
            "zxcvbnm,./ZXCVBNM<>?",
            "🌿🌿🌿",
            "🤺🤺",
            "👨‍👩‍👧‍👦👨‍👩‍👧‍👦",
            "",
        ];
        for &password in passwords.iter() {
            hash_and_verify_password_works(password);
        }
    }
}
