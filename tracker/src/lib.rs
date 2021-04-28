#[macro_use]
extern crate redis_module;

use peerinfo::PeerInfo;
use redis_module::{native_types::RedisType, Status};
use redis_module::{raw, Context, RedisError, RedisResult, RedisValue};
use seederinfo::SeederInfo;
use std::os::raw::c_void;
use std::time::Duration;
use std::{convert::TryFrom, str::FromStr};

mod peerinfo;
mod seederinfo;
mod util;

#[derive(Debug, PartialEq)]
enum Event {
    Started = 0,
    Completed = 1,
    Stopped = 2,
}

impl Event {
    fn is_stop(&self) -> bool {
        match self {
            Event::Stopped => true,
            _ => false,
        }
    }
}

impl FromStr for Event {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "started" => Event::Started,
            "completed" => Event::Completed,
            "stopped" => Event::Stopped,
            _ => Event::Started,
        })
    }
}

struct AnnounceRequest {
    pid: u64,
    uid: u64,
    peer: PeerInfo,
    numwant: usize,
    event: Event,
}

static SEEDER_MAP_TYPE: RedisType = RedisType::new(
    "SeederMap",
    0,
    raw::RedisModuleTypeMethods {
        version: raw::REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: None,
        rdb_save: None,
        aof_rewrite: None,
        free: Some(free),
        // Currently unused by Redis
        mem_usage: None,
        digest: None,
        // Aux data
        aux_load: None,
        aux_save: None,
        aux_save_triggers: 0,
    },
);

unsafe extern "C" fn free(value: *mut c_void) {
    Box::from_raw(value as *mut SeederInfo);
}

impl TryFrom<Vec<String>> for AnnounceRequest {
    type Error = RedisError;
    fn try_from(args: Vec<String>) -> Result<AnnounceRequest, RedisError> {
        if args.len() < 6 {
            return Err(RedisError::Str("FUCK U"));
        }
        let mut iter = args.into_iter().skip(1);
        let pid = iter.next().unwrap().parse::<u64>()?;
        let uid = iter.next().unwrap().parse::<u64>()?;
        let ipv4 = match iter.next().unwrap().as_str() {
            "none" => None,
            s @ _ => Some(s.parse()?),
        };
        let ipv6 = match iter.next().unwrap().as_str() {
            "none" => None,
            s @ _ => Some(s.parse()?),
        };
        let port: u16 = iter.next().unwrap().parse()?;
        let peer = PeerInfo::from(ipv4, ipv6, port);

        let numwant = match iter.next() {
            None => 50,
            Some(s) => s.parse()?,
        };
        let event = match iter.next() {
            None => Event::Started,
            Some(s) => s.parse()?,
        };
        return Ok(Self {
            pid,
            uid,
            peer,
            numwant,
            event,
        });
    }
}

/* ANNOUNCE <pid> <uid> <v4ip> <v6ip> <port> <EVENT> <NUMWANT> */
fn announce(ctx: &Context, args: Vec<String>) -> RedisResult {
    let AnnounceRequest {
        pid,
        uid,
        peer,
        numwant,
        event,
    } = AnnounceRequest::try_from(args)?;
    let key = ctx.open_key_writable(pid.to_string().as_str());
    if key.is_empty() {
        // as he left, no need to create an empty key.
        if event.is_stop() {
            return Ok(RedisValue::SimpleStringStatic("?"));
        }
        let value = SeederInfo::new();
        key.set_value(&SEEDER_MAP_TYPE, value)?;
    }

    let sm: &mut SeederInfo;
    sm = match key.get_value::<SeederInfo>(&SEEDER_MAP_TYPE)? {
        Some(value) => value,
        None => return Err(RedisError::Str("FUCK U")),
    };
    sm.compaction();
    let response;
    if event.is_stop() {
        sm.delete(uid);
        response = RedisValue::SimpleStringStatic("?");
    } else {
        sm.insert(uid, peer);
        response = sm.gen_response(numwant);
    }
    key.set_expire(Duration::from_secs(2700))?;
    Ok(response)
}

fn init(_: &Context, _: &Vec<String>) -> Status {
    Status::Ok
}

redis_module! {
    name: "redistracker",
    version: 1,
    data_types: [SEEDER_MAP_TYPE],
    init: init,
    commands: [["announce", announce, "write deny-oom", 1, 1, 1]],
}

#[cfg(test)]
mod tests {
    use std::{convert::TryFrom, net::Ipv4Addr, net::Ipv6Addr, str::FromStr};

    use crate::{AnnounceRequest, Event};

    fn dummy_request() -> Vec<String> {
        vec![
            "announce".into(),
            "1".into(),
            "1".into(),
            "1.1.1.1".into(),
            "::".into(),
            "1".into(),
        ]
    }
    #[test]
    fn check_parse_event() {
        assert!(Event::from_str("").is_ok());
        assert_eq!("completed".parse(), Ok(Event::Completed));
        assert_eq!("started".parse(), Ok(Event::Started));
        assert_eq!("stopped".parse(), Ok(Event::Stopped));
        assert_eq!("˚¬˚".parse(), Ok(Event::Started));
    }

    #[test]
    fn check_parse_request0() {
        let raw = dummy_request();
        let req = AnnounceRequest::try_from(raw);
        assert!(req.is_ok());
        let req = req.unwrap();
        assert_eq!(req.event, Event::Started);
        assert_eq!(req.numwant, 50);
        assert_eq!(req.pid, 1);
        assert_eq!(req.uid, 1);
        let p = req.peer;
        assert_eq!(p.get_port(), 1);
        assert_eq!(p.get_ipv4(), Some(Ipv4Addr::new(1, 1, 1, 1)));
        assert_eq!(p.get_ipv6(), Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)));
    }
    #[test]
    fn check_parse_request1() {
        let mut raw = dummy_request();
        raw.push("100".into());
        raw.push("stopped".into());
        let req = AnnounceRequest::try_from(raw);
        assert!(req.is_ok());
        let req = req.unwrap();
        assert_eq!(req.event, Event::Stopped);
        assert_eq!(req.numwant, 100);
    }
    #[test]
    fn check_parse_request2() {
        let mut raw = dummy_request();
        raw.push("-100".into());
        let req = AnnounceRequest::try_from(raw);
        assert!(req.is_err());
    }

    #[test]
    fn check_parse_request3() {
        let mut raw = dummy_request();
        raw.pop();
        let req = AnnounceRequest::try_from(raw);
        assert!(req.is_err());
    }

    #[test]
    fn check_parse_request4() {
        let mut raw = dummy_request();
        raw[5] = "-1".into();
        let req = AnnounceRequest::try_from(raw);
        assert!(req.is_err());
    }

    #[test]
    fn check_parse_request5() {
        let mut raw = dummy_request();
        raw[5] = "65536".into();
        let req = AnnounceRequest::try_from(raw);
        assert!(req.is_err());
    }

    #[test]
    fn check_parse_request6() {
        let mut raw = dummy_request();
        raw[4] = "none".into();
        let req = AnnounceRequest::try_from(raw);
        assert!(req.is_ok());
        let p = req.unwrap().peer;
        assert!(p.get_ipv6().is_none());
    }
}
