use std::net::IpAddr;
use salvo::prelude::*;
use salvo::rate_limiter::{BasicQuota, FixedGuard, MokaStore, RateIssuer, RateLimiter};

#[derive(Copy, Clone, Debug)]
pub struct SafeIpIssuer;

impl RateIssuer for SafeIpIssuer {
    type Key = IpAddr;
    async fn issue(&self, req: &mut Request, _depot: &Depot) -> Option<Self::Key> {
        if let Some(ip) = req.remote_addr().ip() {
            Some(ip)
        } else {
            "127.0.0.1".parse().ok()
        }
    }
}

pub fn global_rate_limiter() -> impl Handler {
    RateLimiter::new(
        FixedGuard::default(),
        MokaStore::default(),
        SafeIpIssuer,
        BasicQuota::per_minute(120),
    )
    .add_headers(true)
    .skipper(|req: &mut Request, _: &Depot| {
        cfg!(test) || req.uri().path().ends_with("/health")
    })
}

pub fn auth_rate_limiter() -> impl Handler {
    RateLimiter::new(
        FixedGuard::default(),
        MokaStore::default(),
        SafeIpIssuer,
        BasicQuota::per_minute(20),
    )
    .add_headers(true)
    .skipper(|_req: &mut Request, _: &Depot| cfg!(test))
}
