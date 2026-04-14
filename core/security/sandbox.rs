use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Permission {
    Network,
    FileSystem,
    Execute,
}

pub struct Sandbox {
    allowed: HashSet<Permission>,
}

impl Sandbox {
    pub fn new() -> Self {
        Self {
            allowed: HashSet::new(),
        }
    }

    pub fn allow(&mut self, perm: Permission) {
        self.allowed.insert(perm);
    }

    pub fn check(&self, perm: &Permission) -> bool {
        self.allowed.contains(perm)
    }

    pub fn enforce(&self, perm: Permission) -> Result<(), String> {
        if self.check(&perm) {
            Ok(())
        } else {
            Err(format!("Permission denied: {:?}", perm))
        }
    }
}
