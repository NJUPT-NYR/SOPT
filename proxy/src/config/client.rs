// Mainly an adaption from bittorrent-peerid,
// a node.js module.
// Find more at https://github.com/webtorrent/bittorrent-peerid

use crate::error::ProxyError;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Client {
    Ares,
    Aria,
    ATorrent,
    Avicora,
    BitPump,
    BitBuddy,
    BitComet,
    BitSpirit,
    Bitflu,
    BTG,
    BitRocket,
    BTSlave,
    Bittorrent,
    BittorrentX,
    CTorrent,
    DelugeTorrent,
    PropagateDataClient,
    EBit,
    Electricsheep,
    FoxTorrent,
    FreeboxBitTorrent,
    GSTorrent,
    Halite,
    Hydranode,
    KGet,
    KTorrent,
    Lphant,
    LibTorrent,
    LimeWire,
    MonoTorrent,
    MooPolice,
    Miro,
    MoonlightTorrent,
    NetTransport,
    OneSwarm,
    Pando,
    PopcornTime,
    QBittorrent,
    QQDownload,
    Retriever,
    Shareazaalphabeta,
    Swiftbit,
    SwarmScope,
    SymTorrent,
    Sharktorrent,
    Shareaza,
    TorrentDotNET,
    Transmission,
    Torrentstorm,
    Tixati,
    TuoTu,
    ULeecher,
    UTorrent,
    UTorrentWeb,
    Vagaa,
    Vuze,
    WebTorrentDesktop,
    BitLet,
    WebTorrent,
    FireTorrent,
    Xunlei,
    XanTorrent,
    Xtorrent,
    ZipTorrent,
}

lazy_static! {
    static ref AZ_CLIENT: HashMap<&'static str, Client> = [
        ("A~", Client::Ares),
        ("AG", Client::Ares),
        ("AR", Client::Ares),
        ("AV", Client::Avicora),
        ("AX", Client::BitPump),
        ("AZ", Client::Vuze),
        ("BB", Client::BitBuddy),
        ("BC", Client::BitComet),
        ("BF", Client::Bitflu),
        ("BG", Client::BTG),
        ("BR", Client::BitRocket),
        ("BS", Client::BTSlave),
        ("BT", Client::Bittorrent),
        ("BX", Client::BittorrentX),
        ("CB", Client::Shareaza),
        ("CD", Client::CTorrent),
        ("CT", Client::CTorrent),
        ("DP", Client::PropagateDataClient),
        ("DE", Client::DelugeTorrent),
        ("EB", Client::EBit),
        ("ES", Client::Electricsheep),
        ("FX", Client::FreeboxBitTorrent),
        ("FT", Client::FoxTorrent),
        ("GS", Client::GSTorrent),
        ("HL", Client::Halite),
        ("HN", Client::Hydranode),
        ("KG", Client::KGet),
        ("KT", Client::KTorrent),
        ("LP", Client::Lphant),
        ("LT", Client::LibTorrent),
        ("lt", Client::LibTorrent),
        ("LW", Client::LimeWire),
        ("MO", Client::MonoTorrent),
        ("MP", Client::MooPolice),
        ("MR", Client::Miro),
        ("MT", Client::MoonlightTorrent),
        ("NX", Client::NetTransport),
        ("OS", Client::OneSwarm),
        ("PT", Client::PopcornTime),
        ("PD", Client::Pando),
        ("qB", Client::QBittorrent),
        ("QD", Client::QQDownload),
        ("RT", Client::Retriever),
        ("S~", Client::Shareazaalphabeta),
        ("SB", Client::Swiftbit),
        ("SD", Client::Xunlei),
        ("SG", Client::GSTorrent),
        ("SP", Client::BitSpirit),
        ("SS", Client::SwarmScope),
        ("ST", Client::SymTorrent),
        ("st", Client::Sharktorrent),
        ("SZ", Client::Shareaza),
        ("TN", Client::TorrentDotNET),
        ("TR", Client::Transmission),
        ("TS", Client::Torrentstorm),
        ("TT", Client::TuoTu),
        ("UL", Client::ULeecher),
        ("UE", Client::UTorrent),
        ("UT", Client::UTorrent),
        ("UM", Client::UTorrent),
        ("UW", Client::UTorrentWeb),
        ("WD", Client::WebTorrentDesktop),
        ("WT", Client::BitLet),
        ("WW", Client::WebTorrent),
        ("WY", Client::FireTorrent),
        ("VG", Client::Vagaa),
        ("XL", Client::Xunlei),
        ("XT", Client::XanTorrent),
        ("XX", Client::Xtorrent),
        ("XC", Client::Xtorrent),
        ("ZT", Client::ZipTorrent),
        ("7T", Client::ATorrent),
    ]
    .iter()
    .copied()
    .collect();
}

lazy_static! {
    static ref SP_CLIENT: HashMap<&'static str, Client> = [
        ("-aria2-", Client::Aria),
        ("BitLet", Client::BitLet),
        ("LIME", Client::LimeWire),
        ("Pando", Client::Pando),
        ("TIX", Client::Tixati),
        ("DansClient", Client::XanTorrent),
        ("-UM", Client::UTorrent),
        ("-UT", Client::UTorrent),
    ]
    .iter()
    .copied()
    .collect();
}

impl Client {
    pub fn new(id: &str) -> Result<Self, ProxyError> {
        let peer_id;
        {
            if id.len() != 20 {
                let decoded = hex::decode(id).map_err(|_e| ProxyError::EncodeError)?;
                if decoded.len() != 20 {
                    return Err(ProxyError::EncodeError);
                }
                peer_id = String::from_utf8(decoded).map_err(|_e| ProxyError::EncodeError)?;
            } else {
                peer_id = String::from(id);
            }
        }

        if is_az_style(&peer_id) {
            let pat = sub_str(&peer_id, 1, 3);
            let res = AZ_CLIENT.get(pat.as_str());
            if res.is_some() {
                return Ok(res.unwrap().clone());
            }
        }
        for pat in SP_CLIENT.keys() {
            if peer_id.starts_with(pat) {
                return Ok(SP_CLIENT.get(pat).unwrap().clone());
            }
        }

        Err(ProxyError::EncodeError)
    }
}

fn is_az_style(id: &str) -> bool {
    return if id.chars().nth(0).unwrap() != '-' {
        false
    } else if id.chars().nth(7).unwrap() == '-' {
        true
    } else {
        // Hack for KTorrent and BitSpirit
        match sub_str(id, 1, 3).as_str() {
            "KT" => true,
            "SP" => true,
            _ => false,
        }
    };
}

fn sub_str(id: &str, start: usize, end: usize) -> String {
    String::from(id.split_at(end).0.split_at(start).1)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn normal_utf8_az_peer_id_works() {
        let client = Client::new("-AG2053-Em6o1EmvwLtD").unwrap();
        assert_eq!(client, Client::Ares);
        let client = Client::new("-AZ2200-6wfG2wk6wWLc").unwrap();
        assert_eq!(client, Client::Vuze);
        let client = Client::new("-TR0072-8vd6hrmp04an").unwrap();
        assert_eq!(client, Client::Transmission);
        let client = Client::new("-WY0300-6huHF5Pr7Vde").unwrap();
        assert_eq!(client, Client::FireTorrent);
    }
    #[test]
    fn hex_encoded_az_peer_id_works() {
        let client = Client::new("2D535A323133322D000000000000000000000000").unwrap();
        assert_eq!(client, Client::Shareaza);
        let client = Client::new("2D5554474836372D6B6C414A6B40405955236A33").unwrap();
        assert_eq!(client, Client::UTorrent);
        let client = Client::new("2D4C576B6142472D4A4138376A616B5E6C6D6729").unwrap();
        assert_eq!(client, Client::LimeWire);
        let client = Client::new("2D4C50303330322D003833363536393537373030").unwrap();
        assert_eq!(client, Client::Lphant);
    }
    #[test]
    fn more_az_peer_id_works() {
        let client = Client::new("-BR0332-!XVceSn(*KIl").unwrap();
        assert_eq!(client, Client::BitRocket);
        let client = Client::new("-HL0290-xUO*9ugvENUE").unwrap();
        assert_eq!(client, Client::Halite);
        let client = Client::new("-KT11R16-93649213030").unwrap();
        assert_eq!(client, Client::KTorrent);
        let client = Client::new("2D4B543330302D006A7139727958377731756A4B").unwrap();
        assert_eq!(client, Client::KTorrent);
        let client = Client::new("2D6C74522535362D4B395554542D443637534140").unwrap();
        assert_eq!(client, Client::LibTorrent);
        let client = Client::new("-TT210w-dq!nWf~Qcext").unwrap();
        assert_eq!(client, Client::TuoTu);
    }
    #[test]
    fn special_peer_id_works() {
        let client = Client::new("Pando-6B511B691CAC2E").unwrap();
        assert_eq!(client, Client::Pando);
        let client = Client::new("2D554D416A613240612D2D6173666A26326D646C").unwrap();
        assert_eq!(client, Client::UTorrent);
        let client = Client::new("TIX0137-i6i6f0i5d5b7").unwrap();
        assert_eq!(client, Client::Tixati);
    }
    #[test]
    fn invalid_peer_id_works() {
        let client = Client::new("-#@0000-Em6o1EmvwLtD");
        assert!(client.is_err());
        let client = Client::new("E7F163BB0E5FCD35005C09A11BC274C42385A1A0");
        assert!(client.is_err());
        let client = Client::new("1145141919810");
        assert!(client.is_err());
    }
}
