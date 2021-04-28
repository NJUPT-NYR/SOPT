use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use bendy::encoding;
use deadpool_redis::{cmd, redis::Value, Cmd};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AnnounceRequestData {
    // deprecated
    // pub info_hash: Vec<u8>,
    pub peer_id: String,
    pub port: u16,
    pub uid: i64,
    pub tid: i64,
    pub passkey: String,
    pub ip: Option<IpAddr>,
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
    #[serde(default)]
    pub event: Event,
    #[serde(default = "crate::config::default_num_want")]
    pub numwant: u16,
    pub upload: i64,
    pub download: i64,
}

impl AnnounceRequestData {
    pub fn fix_ip(&mut self, peer_addr: Option<IpAddr>) {
        let mut true_v4 = None;
        let mut true_v6 = None;
        if let Some(ip) = self.ip {
            match ip {
                IpAddr::V4(v4) => true_v4 = self.ipv4.or(Some(v4)),
                IpAddr::V6(v6) => true_v6 = self.ipv6.or(Some(v6)),
            }
        }
        if let Some(ip) = peer_addr {
            match ip {
                IpAddr::V4(v4) => true_v4 = true_v4.or(Some(v4)),
                IpAddr::V6(v6) => true_v6 = true_v6.or(Some(v6)),
            }
        }
        if true_v4.is_none() && true_v6.is_none() {
            panic!("unable to detect connection address");
        }
        self.ipv4 = true_v4;
        self.ipv6 = true_v6;
    }

    pub fn generate_announce_cmd(&self) -> Cmd {
        let ipv4 = match self.ipv4 {
            Some(ip) => ip.to_string(),
            None => String::from("none"),
        };
        let ipv6 = match self.ipv6 {
            Some(ip) => ip.to_string(),
            None => String::from("none"),
        };
        let mut acmd = cmd("ANNOUNCE");
        acmd.arg(self.tid)
            .arg(self.uid)
            .arg(ipv4)
            .arg(ipv6)
            .arg(self.port)
            .arg(self.numwant)
            .arg(self.event.to_string());
        acmd
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Event {
    Started = 0,
    Completed = 1,
    Stopped = 2,
}

impl Default for Event {
    fn default() -> Self {
        Event::Started
    }
}

impl ToString for Event {
    fn to_string(&self) -> String {
        match self {
            Event::Started => "started",
            Event::Completed => "completed",
            Event::Stopped => "stopped",
        }
        .into()
    }
}

#[derive(Serialize, Debug)]
pub struct AnnounceBypassData {
    uid: i64,
    tid: i64,
    upload: i64,
    download: i64,
    action: Option<Action>,
}

impl From<AnnounceRequestData> for AnnounceBypassData {
    fn from(t: AnnounceRequestData) -> Self {
        let action = match t.event {
            Event::Started => Action::Start,
            Event::Completed => Action::Complete,
            Event::Stopped => Action::Stop,
        };

        Self {
            uid: t.uid,
            tid: t.tid,
            upload: t.upload,
            download: t.download,
            action: Some(action),
        }
    }
}

#[repr(C)]
#[derive(Serialize, Debug, Copy, Clone)]
enum Action {
    Start = 0,
    Complete,
    Stop,
}

pub struct AnnounceResponseData {
    interval: i64,
    peers: Vec<u8>,
    peers6: Vec<u8>,
}

impl From<Vec<Value>> for AnnounceResponseData {
    fn from(t: Vec<Value>) -> Self {
        let mut iter = t.into_iter();
        let interval = match iter.next() {
            Some(Value::Int(i)) => i,
            _ => 1800,
        };
        let peers = match iter.next() {
            Some(Value::Data(peers)) => peers,
            _ => vec![],
        };
        let peers6 = match iter.next() {
            Some(Value::Data(peers6)) => peers6,
            _ => vec![],
        };
        Self {
            interval,
            peers,
            peers6,
        }
    }
}

impl encoding::ToBencode for AnnounceResponseData {
    const MAX_DEPTH: usize = 3;

    fn encode(&self, encoder: encoding::SingleItemEncoder) -> Result<(), encoding::Error> {
        encoder.emit_dict(|mut e| {
            e.emit_pair(b"inteval", self.interval)?;
            e.emit_pair(b"peers", &self.peers)?;
            e.emit_pair(b"peers6", &self.peers6)?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct UpdateFilterCommand {
    pub set: Option<String>,
    pub delete: Option<String>,
}
