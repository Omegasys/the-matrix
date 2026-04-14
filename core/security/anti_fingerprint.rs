use rand::Rng;

pub struct FingerprintMasker;

impl FingerprintMasker {
    pub fn random_user_agent() -> String {
        let agents = vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
            "Mozilla/5.0 (X11; Linux x86_64)",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
        ];

        let idx = rand::thread_rng().gen_range(0..agents.len());
        agents[idx].to_string()
    }

    pub fn jitter_timing(base_ms: u64) -> u64 {
        let jitter: u64 = rand::thread_rng().gen_range(0..50);
        base_ms + jitter
    }

    pub fn randomize_packet_size(size: usize) -> usize {
        let variation: i32 = rand::thread_rng().gen_range(-16..16);
        (size as i32 + variation).max(1) as usize
    }
}
