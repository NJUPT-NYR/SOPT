use pest::Parser;
use pest_derive::*;
use rand::{thread_rng, Rng};
use crate::config::CONFIG;
use crate::error::{error_string, Error};

/// Get timestamp of current time with unix standard.
///
/// it will return a `u64`.
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
/// hex, stripped into 24bytes（append uid when generating torrents).
///
/// The format: {username}{timestamp}{random u64}
pub fn generate_passkey(username: &str) -> Result<String, Error> {
    use sha2::{Digest, Sha256};
    use std::convert::TryInto;

    let mut hasher = Sha256::new();
    let random: u64 = rand::thread_rng().gen();
    hasher.update(format!("{}{}{}", username, get_timestamp(), random));

    let res: Vec<u8> = hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(error_string)?;
    let string = hex::encode(res);
    Ok(String::from(&string[..24]))
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
            format!("{} <{}>", from, CONFIG.smtp.username)
                .parse()
                .unwrap(),
        )
        .to(
            format!("{} <{}>", receiver, address)
                .parse()
                .unwrap(),
        )
        .subject("Invitation Code")
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
        retry_count = retry_count - 1;
        // retry after 1 seconds
        sleep(std::time::Duration::from_secs(1));
        if retry_count == 0 {
            return Err(Error::OtherError(err.to_string()));
        }
    }

    Ok(())
}

/// Generate invite code for invitations
///
/// format: {random 10 chars}_{timestamp}
pub fn generate_invitation_code() -> String {
    use rand::distributions::Alphanumeric;

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    // is it proper to add a timestamp
    // maybe easier to check expiration
    format!("{}_{}", rand_string, get_timestamp()).to_string()
}

use crate::data::Claim;
/// try decode and verify if jwt is expired
pub fn decode_and_verify_jwt(token: &str, secret: &[u8]) -> Result<Claim, Error> {
    use jsonwebtoken::{decode, Validation, DecodingKey};

    let decoded =
        decode::<Claim>(token, &DecodingKey::from_secret(secret), &Validation::default())
            .map_err(error_string)?;

    let claim: Claim = decoded.claims;
    let now = get_timestamp();
    if claim.exp < now {
        Err(Error::AuthError)
    } else {
        Ok(claim)
    }
}

use crate::data::torrent::{Torrent, TorrentTable};
/// Parse uploaded torrent file and convert into a table row
pub fn parse_torrent_file(buf: &[u8]) -> Result<TorrentTable, Error> {
    use serde_bencode::{from_bytes, to_bytes};
    use sha1::{Sha1, Digest};
    use std::convert::TryInto;

    let mut ret = from_bytes::<Torrent>(buf).map_err(error_string)?;
    ret.info.private = Some(1);
    let info = to_bytes(&ret.info).map_err(error_string)?;
    let mut hasher = Sha1::new();
    hasher.update(&info);
    let res: Vec<u8> = hasher.finalize()
        .as_slice()
        .try_into()
        .map_err(error_string)?;
    let infohash = hex::encode(res);

    let length = ret.info.length.unwrap_or(ret.info.piece_length * 8);
    let files = ret.info.files.unwrap_or_default()
        .iter()
        .map(|file| file.path.iter()
            .fold("".to_string(),
                  |acc, x| acc + "/" + x))
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
/// Announce format: {announce_addr}?passkey={{passkey}{user id}}&tid={torrent id}
pub fn generate_torrent_file(mut info: Vec<u8>, passkey: &str, tid: i64, uid: i64, comment: &str) -> Vec<u8> {
    use serde_bencode::to_string;

    // now passkey is 40 bytes long
    let ext_passkey = format!("{}{}", passkey, hex::encode(uid.to_ne_bytes()));
    let announce_address = to_string(&format!("{}?passkey={}&tid={}", CONFIG.announce_addr, ext_passkey, tid)).unwrap();
    let comment = to_string(&comment).unwrap();

    let mut hand_ser = "d4:info".to_string().into_bytes();
    hand_ser.append(&mut info);
    hand_ser.append(&mut format!("8:announce{}7:comment{}e", announce_address, comment).into_bytes());

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
        use chrono::{Utc, Duration};
        use jsonwebtoken::{encode, EncodingKey, Header};
        use crate::data::Claim;

        let claim = Claim {
            sub: "YUKI.N".to_string(),
            role: 1,
            exp: (Utc::now() + Duration::days(30)).timestamp() as u64,
        };
        let tokens = encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret("secret".as_bytes())).unwrap();
        let ret = decode_and_verify_jwt(&tokens, "secret".as_bytes()).unwrap();

        assert_eq!(ret.sub, "YUKI.N".to_string())
    }
}
