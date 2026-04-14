use std::net::IpAddr;
use tokio::net::lookup_host;

#[derive(Debug)]
pub struct DnsResolver;

impl DnsResolver {
    pub async fn resolve(domain: &str) -> Result<Vec<IpAddr>, String> {
        let result = lookup_host((domain, 0))
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.map(|addr| addr.ip()).collect())
    }
}
