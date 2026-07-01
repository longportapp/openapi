//! Geo-detection helper for Longport OpenAPI.
//!
//! Determines whether the current access point is in China Mainland so that
//! callers can choose between `*.longport.cn` and `*.longport.com`
//! endpoints.

use std::{
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

// Process-wide cache so the probe is done at most once regardless of which
// tokio worker thread calls `is_cn()`.
static IS_CN_DONE: OnceLock<bool> = OnceLock::new();

// Used to prevent multiple concurrent probes racing at startup.
static IS_CN_PROBING: AtomicBool = AtomicBool::new(false);

/// Do the best to guess whether the access point is in China Mainland or not.
///
/// Detection priority:
/// 1. `LONGPORT_REGION` environment variable (takes precedence).
/// 2. `LONGPORT_REGION` environment variable (fallback alias).
/// 3. Process-wide cached result from a previous probe.
/// 4. Live HTTP probe to `https://geotest.lbkrs.com` — HTTP 200 → CN, anything
///    else (error or non-200) → not CN.
pub async fn is_cn() -> bool {
    // 1 & 2: explicit region override
    let user_region = std::env::var("LONGPORT_REGION")
        .ok()
        .or_else(|| std::env::var("LONGPORT_REGION").ok());
    if let Some(region) = user_region {
        return region.eq_ignore_ascii_case("CN");
    }

    // 3: already probed
    if let Some(&cached) = IS_CN_DONE.get() {
        return cached;
    }

    // 4: live probe — only one task does the actual probe; others fall back
    //    to `false` (global endpoint) which is safe and avoids a pile-up.
    if IS_CN_PROBING
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
    {
        let result = reqwest::Client::new()
            .get("https://geotest.lbkrs.com")
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .is_ok_and(|resp| resp.status().is_success());

        let _ = IS_CN_DONE.set(result);
        result
    } else {
        // Another task is probing; use the cached value if it finished in the
        // meantime, otherwise default to global endpoint.
        IS_CN_DONE.get().copied().unwrap_or(false)
    }
}
