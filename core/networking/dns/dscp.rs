#[derive(Debug, Clone, Copy)]
pub enum DscpClass {
    Default = 0,
    ExpeditedForwarding = 46,
    AssuredForwarding11 = 10,
    AssuredForwarding21 = 18,
    AssuredForwarding31 = 26,
}

impl DscpClass {
    pub fn value(self) -> u8 {
        self as u8
    }
}

/// Apply DSCP value to a socket (Linux only for now)
#[cfg(target_os = "linux")]
pub fn apply_dscp(fd: i32, dscp: DscpClass) -> Result<(), String> {
    use libc::{setsockopt, IPPROTO_IP, IP_TOS};

    let val = (dscp.value() << 2) as i32;

    let ret = unsafe {
        setsockopt(
            fd,
            IPPROTO_IP,
            IP_TOS,
            &val as *const _ as *const _,
            std::mem::size_of::<i32>() as u32,
        )
    };

    if ret != 0 {
        return Err("Failed to set DSCP".into());
    }

    Ok(())
}
