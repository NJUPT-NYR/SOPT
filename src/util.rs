use crate::config::CONFIG;
use crate::error::{error_string, Error};
use chrono::Utc;
use pest::Parser;
use pest_derive::*;
use rand::{thread_rng, Rng};

#[derive(Parser)]
#[grammar = "email_address.pest"]
struct Rules;

/// Struct of email address, with two parts:
/// local and domain. It supports RFC 6530 e.g.
/// UTF-8 supported, like Göthe@Weimar-straßen.de
#[derive(Debug)]
pub struct EmailAddress {
    pub local: String,
    pub domain: String,
}

/// Use pest parser to parse email address inputs
/// Returns `None` when it it illegal address
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

/// Generate passkey used for torrents with sha256 and will be encoded with
/// hex, stripped into 32 bytes.
///
/// The format: {username}{timestamp}{random u64}
pub fn generate_passkey(username: &str) -> Result<String, Error> {
    use sha2::{Digest, Sha256};
    use std::convert::TryInto;

    let mut hasher = Sha256::new();
    let random: u64 = rand::thread_rng().gen();
    hasher.update(format!("{}{}{}", username, Utc::now().timestamp(), random));

    let res: Vec<u8> = hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(error_string)?;
    let string = hex::encode(res);
    Ok(String::from(&string[..32]))
}

/// Hash password for database.
/// Used crate: rust-argon2, champion of HPC.
pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt: [u8; 20] = rand::thread_rng().gen();
    let config = argon2::Config::default();

    argon2::hash_encoded(password.as_ref(), &salt, &config).map_err(error_string)
}

/// Verify password selected from password with
/// user input.
/// Used crate: rust-argon2, champion of HPC.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    argon2::verify_encoded(hash, password.as_ref()).map_err(error_string)
}

/// send invitation mail to someone
/// since it is not asynchronous we need another thread
/// to handle it. SMTP is used so config is need.
///
/// Default retry count: 5
pub fn send_mail(
    receiver: &str,
    address: &str,
    from: &str,
    body: String,
    subject: &str,
) -> Result<(), Error> {
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};
    use std::thread::sleep;

    let mail = Message::builder()
        .from(
            format!("{} <{}>", from, CONFIG.smtp.username)
                .parse()
                .unwrap(),
        )
        .to(format!("{} <{}>", receiver, address).parse().unwrap())
        .subject(subject)
        .body(body)
        .map_err(error_string)?;

    let client = SmtpTransport::relay(&CONFIG.smtp.server)
        .map_err(error_string)?
        .credentials(Credentials::new(
            CONFIG.smtp.username.clone(),
            CONFIG.smtp.password.clone(),
        ))
        .build();

    let mut retry_count = 5;

    while let Err(err) = client.send(&mail) {
        retry_count -= 1;
        // retry after 1 seconds
        sleep(std::time::Duration::from_secs(1));
        if retry_count == 0 {
            return Err(Error::OtherError(err.to_string()));
        }
    }

    Ok(())
}

/// Generate random code for invitations and activations, etc.
///
/// format: {random 10 chars}_{timestamp}
pub fn generate_random_code() -> String {
    use rand::distributions::Alphanumeric;

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    // is it proper to add a timestamp
    // maybe easier to check expiration
    format!("{}_{}", rand_string, Utc::now().timestamp())
}

/// split the random code and get timestamp
/// in it.
pub fn get_time_from_code(code: String) -> Result<i64, Error> {
    let strings: Vec<&str> = code.split('_').collect();
    if strings.len() < 2 {
        return Err(Error::OtherError("invalid code".to_string()));
    }
    i64::from_str_radix(strings[1].trim(), 10).map_err(error_string)
}

use crate::data::Claim;
/// try decode and verify if jwt is expired
pub fn decode_and_verify_jwt(token: &str, secret: &[u8]) -> Result<Claim, Error> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    let decoded = decode::<Claim>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map_err(error_string)?;

    let claim: Claim = decoded.claims;
    if claim.exp < Utc::now().timestamp() {
        Err(Error::AuthError)
    } else {
        Ok(claim)
    }
}

use crate::data::torrent::{Torrent, TorrentTable};
/// Parse uploaded torrent file and convert into a table row
pub fn parse_torrent_file(buf: &[u8]) -> Result<TorrentTable, Error> {
    use serde_bencode::{from_bytes, to_bytes};
    use sha1::{Digest, Sha1};
    use std::convert::TryInto;

    let mut ret = from_bytes::<Torrent>(buf).map_err(error_string)?;
    ret.info.private = Some(1);
    let info = to_bytes(&ret.info).map_err(error_string)?;
    let mut hasher = Sha1::new();
    hasher.update(&info);
    let res: Vec<u8> = hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(error_string)?;
    let infohash = hex::encode(res);

    let length = ret.info.length.unwrap_or(ret.info.piece_length * 8);
    let files = ret
        .info
        .files
        .unwrap_or_default()
        .iter()
        .map(|file| {
            file.path
                .iter()
                .fold("".to_string(), |acc, x| acc + "/" + x)
        })
        .collect();

    Ok(TorrentTable {
        id: 1919810,
        name: ret.info.name,
        length,
        comment: ret.comment,
        files,
        info,
        infohash,
    })
}

/// Generate torrent file buf with custom announce and passkey
///
/// Announce format: {announce_addr}?passkey={passkey}&tid={torrent id}&uid={user id}
pub fn generate_torrent_file(
    mut info: Vec<u8>,
    passkey: &str,
    tid: i64,
    uid: i64,
    comment: &str,
) -> Vec<u8> {
    use serde_bencode::to_string;

    let announce_address = to_string(&format!(
        "{}?passkey={}&tid={}&uid={}",
        CONFIG.announce_addr, passkey, tid, uid
    ))
    .unwrap();
    let comment = to_string(&comment).unwrap();

    let mut hand_ser = "d4:info".to_string().into_bytes();
    hand_ser.append(&mut info);
    hand_ser
        .append(&mut format!("8:announce{}7:comment{}e", announce_address, comment).into_bytes());

    hand_ser
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
    #[test]
    fn decode_and_verify_jwt_works() {
        use crate::data::Claim;
        use jsonwebtoken::{encode, EncodingKey, Header};

        let claim = Claim {
            sub: "YUKI.N".to_string(),
            role: 1,
            exp: (Utc::now() + chrono::Duration::days(30)).timestamp(),
        };
        let tokens = encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret("secret".as_bytes()),
        )
        .unwrap();
        let ret = decode_and_verify_jwt(&tokens, "secret".as_bytes()).unwrap();

        assert_eq!(ret.sub, "YUKI.N".to_string())
    }
    #[test]
    fn get_timestamp_from_code_works() {
        let code = format!("acdxfghjk_{}", 114514);
        let wrong1 = format!("akjashdka2189213");
        let wrong2 = format!("kajsjdncjk_212jas28");

        let time = get_time_from_code(code).unwrap();
        let ret1 = get_time_from_code(wrong1);
        let ret2 = get_time_from_code(wrong2);

        assert_eq!(time, 114514);
        assert!(ret1.is_err());
        assert!(ret2.is_err());
    }
}
