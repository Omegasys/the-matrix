use std::net::IpAddr;
use crate::core::networking::firewall::rules::{FirewallRule, RuleAction, Protocol};

pub struct Firewall {
    rules: Vec<FirewallRule>,
    default_action: RuleAction,
}

impl Firewall {
    pub fn new(default_action: RuleAction) -> Self {
        Self {
            rules: Vec::new(),
            default_action,
        }
    }

    pub fn add_rule(&mut self, rule: FirewallRule) {
        self.rules.push(rule);
    }

    pub fn check_packet(
        &self,
        src: IpAddr,
        dst: IpAddr,
        port: u16,
        proto: Protocol,
    ) -> bool {
        for rule in &self.rules {
            if rule.matches(src, dst, port, &proto) {
                return matches!(rule.action, RuleAction::Allow);
            }
        }

        matches!(self.default_action, RuleAction::Allow)
    }
}
