use std::net::IpAddr;

#[derive(Debug, Clone)]
pub enum RuleAction {
    Allow,
    Deny,
}

#[derive(Debug, Clone)]
pub enum Protocol {
    TCP,
    UDP,
    Any,
}

#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub source_ip: Option<IpAddr>,
    pub dest_ip: Option<IpAddr>,
    pub port: Option<u16>,
    pub protocol: Protocol,
    pub action: RuleAction,
}

impl FirewallRule {
    pub fn matches(
        &self,
        src: IpAddr,
        dst: IpAddr,
        port: u16,
        proto: &Protocol,
    ) -> bool {
        if let Some(ip) = self.source_ip {
            if ip != src {
                return false;
            }
        }

        if let Some(ip) = self.dest_ip {
            if ip != dst {
                return false;
            }
        }

        if let Some(p) = self.port {
            if p != port {
                return false;
            }
        }

        match (&self.protocol, proto) {
            (Protocol::Any, _) => true,
            (p1, p2) => p1 == p2,
        }
    }
}
