use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum AIPermission {
    Move,
    Communicate,
    AccessNetwork,
    ModifyScene,
}

pub struct PermissionSet {
    allowed: HashSet<AIPermission>,
}

impl PermissionSet {
    pub fn new() -> Self {
        Self {
            allowed: HashSet::new(),
        }
    }

    pub fn allow(&mut self, perm: AIPermission) {
        self.allowed.insert(perm);
    }

    pub fn check(&self, perm: &AIPermission) -> bool {
        self.allowed.contains(perm)
    }
}
