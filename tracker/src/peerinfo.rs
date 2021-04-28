use std::net::{Ipv4Addr, Ipv6Addr};

/// Just like
/// ```
/// struct PeerInfo {
///     ipv4: Option<Ipv4Addr>,
///     ipv6: Option<Ipv6Addr>,
///     port: u16,
/// }
/// ```
/// but take lower memory
#[derive(Clone)]
pub struct PeerInfo {
    ipv4: Ipv4Addr,
    ipv6: Ipv6Addr,
    port: u16,
    has_v4: bool,
    has_v6: bool,
}

impl PeerInfo {
    pub fn new() -> Self {
        Self {
            ipv4: Ipv4Addr::UNSPECIFIED,
            ipv6: Ipv6Addr::UNSPECIFIED,
            port: 0,
            has_v4: false,
            has_v6: false,
        }
    }

    pub fn from(ipv4: Option<Ipv4Addr>, ipv6: Option<Ipv6Addr>, port: u16) -> Self {
        let (ipv4, has_v4) = (ipv4.unwrap_or(Ipv4Addr::UNSPECIFIED), ipv4.is_some());
        let (ipv6, has_v6) = (ipv6.unwrap_or(Ipv6Addr::UNSPECIFIED), ipv6.is_some());
        Self {
            ipv4,
            ipv6,
            port,
            has_v4,
            has_v6,
        }
    }

    pub fn get_ipv4(&self) -> Option<Ipv4Addr> {
        if self.has_v4 {
            Some(self.ipv4)
        } else {
            None
        }
    }

    pub fn get_ipv6(&self) -> Option<Ipv6Addr> {
        if self.has_v6 {
            Some(self.ipv6)
        } else {
            None
        }
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn update(&mut self, p2: &PeerInfo) {
        match p2.get_ipv4() {
            Some(ip) => {
                self.ipv4 = ip;
                self.has_v4 = true;
            }
            None => (),
        };
        match p2.get_ipv6() {
            Some(ip) => {
                self.ipv6 = ip;
                self.has_v6 = true;
            }
            None => (),
        };
    }
}

impl Default for PeerInfo {
    fn default() -> Self {
        Self::new()
    }
}
