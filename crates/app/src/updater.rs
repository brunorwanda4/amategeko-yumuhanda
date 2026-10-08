//! The only module allowed to access the network.

#[cfg(feature = "self-update")]
use amategeko_core::update::{parse_release, UPDATE_PUBKEY};
use amategeko_core::update::{pick_asset, Release, ReleaseAsset, UpdatePlatform};
#[cfg(feature = "self-update")]
use minisign_verify::{PublicKey, Signature};
#[cfg(feature = "self-update")]
use std::fs::{self, File};
#[cfg(feature = "self-update")]
use std::io::{Read, Write};
#[cfg(feature = "self-update")]
use std::path::Path;
use std::path::PathBuf;
#[cfg(feature = "self-update")]
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

#[cfg(feature = "self-update")]
const LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/brunorwanda4/amategeko-yumuhanda/releases/latest";
pub const RELEASES_URL: &str = "https://github.com/brunorwanda4/amategeko-yumuhanda/releases";
#[cfg(feature = "self-update")]
const MAX_RELEASE_BYTES: u64 = 2 * 1024 * 1024;
#[cfg(feature = "self-update")]
const MAX_SIGNATURE_BYTES: u64 = 64 * 1024;
#[cfg(feature = "self-update")]
const MAX_DOWNLOAD_BYTES: u64 = 200 * 1024 * 1024;
#[cfg(feature = "self-update")]
const ALLOWED_HOSTS: &[&str] = &[
    "api.github.com",
    "github.com",
    "objects.githubusercontent.com",
    "release-assets.githubusercontent.com",
];

#[derive(Debug, Clone)]
pub struct AvailableUpdate {
    pub release: Release,
    pub asset: ReleaseAsset,
    pub signature: ReleaseAsset,
}

#[derive(Debug, Clone)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpToDate,
    Available(AvailableUpdate),
    Downloading {
        update: AvailableUpdate,
        pct: u8,
    },
    Ready {
        update: AvailableUpdate,
        path: PathBuf,
    },
    PermissionRequired {
        update: AvailableUpdate,
        path: PathBuf,
    },
    Error(UpdateError),
}

static UPDATE_STATUS: OnceLock<Mutex<UpdateStatus>> = OnceLock::new();

pub fn status() -> UpdateStatus {
    UPDATE_STATUS
        .get_or_init(|| Mutex::new(UpdateStatus::Idle))
        .lock()
        .map(|status| status.clone())
        .unwrap_or(UpdateStatus::Idle)
}

pub fn set_status(status: UpdateStatus) {
    if let Ok(mut current) = UPDATE_STATUS
        .get_or_init(|| Mutex::new(UpdateStatus::Idle))
        .lock()
    {
        *current = status;
    }
}

pub fn select_update(release: Release) -> Option<AvailableUpdate> {
    let platform = if cfg!(target_os = "android") {
        UpdatePlatform::Android
    } else if cfg!(target_os = "windows") {
        UpdatePlatform::Windows
    } else {
        return None;
    };
    let (asset, signature) = pick_asset(&release, platform)?;
    Some(AvailableUpdate {
        release,
        asset,
        signature,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateError {
    Network,
    InvalidResponse,
    TooLarge,
    Cancelled,
    BadSignature,
    File,
    ManualRequired,
}

#[cfg(feature = "self-update")]
fn approved_url(url: &str) -> bool {
    let Ok(uri) = url.parse::<ureq::http::Uri>() else {
        return false;
    };
    uri.scheme_str() == Some("https")
        && uri
            .host()
            .map(|host| ALLOWED_HOSTS.contains(&host))
            .unwrap_or(false)
}

#[cfg(feature = "self-update")]
fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .max_redirects(0)
        .user_agent(format!("amategeko-yumuhanda/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .into()
}

#[cfg(feature = "self-update")]
fn get(url: &str) -> Result<ureq::http::Response<ureq::Body>, UpdateError> {
    let agent = agent();
    let mut next = url.to_owned();
    for _ in 0..=5 {
        if !approved_url(&next) {
            return Err(UpdateError::InvalidResponse);
        }
        let response = agent.get(&next).call().map_err(|_| UpdateError::Network)?;
        if !response.status().is_redirection() {
            return Ok(response);
        }
        next = response
            .headers()
            .get("location")
            .and_then(|value| value.to_str().ok())
            .filter(|location| approved_url(location))
            .ok_or(UpdateError::InvalidResponse)?
            .to_owned();
    }
    Err(UpdateError::InvalidResponse)
}

#[cfg(feature = "self-update")]
pub fn check_latest() -> Result<Release, UpdateError> {
    let mut response = get(LATEST_RELEASE_URL)?;
    let json = response
        .body_mut()
        .with_config()
        .limit(MAX_RELEASE_BYTES)
        .read_to_string()
        .map_err(|_| UpdateError::InvalidResponse)?;
    parse_release(&json).map_err(|_| UpdateError::InvalidResponse)
}

#[cfg(feature = "self-update")]
fn read_small(asset: &ReleaseAsset, limit: u64) -> Result<Vec<u8>, UpdateError> {
    let mut response = get(&asset.browser_download_url)?;
    response
        .body_mut()
        .with_config()
        .limit(limit)
        .read_to_vec()
        .map_err(|_| UpdateError::InvalidResponse)
}

#[cfg(feature = "self-update")]
pub fn download_and_verify(
    asset: &ReleaseAsset,
    signature_asset: &ReleaseAsset,
    destination: &Path,
    cancelled: &AtomicBool,
    mut progress: impl FnMut(u8),
) -> Result<PathBuf, UpdateError> {
    if asset.size == 0 || asset.size > MAX_DOWNLOAD_BYTES {
        return Err(UpdateError::TooLarge);
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(|_| UpdateError::File)?;
    }

    let result = (|| {
        let mut response = get(&asset.browser_download_url)?;
        let mut reader = response.body_mut().as_reader().take(MAX_DOWNLOAD_BYTES + 1);
        let mut output = File::create(destination).map_err(|_| UpdateError::File)?;
        let mut downloaded = 0_u64;
        let mut buffer = [0_u8; 64 * 1024];
        loop {
            if cancelled.load(Ordering::Relaxed) {
                return Err(UpdateError::Cancelled);
            }
            let count = reader.read(&mut buffer).map_err(|_| UpdateError::Network)?;
            if count == 0 {
                break;
            }
            downloaded = downloaded.saturating_add(count as u64);
            if downloaded > MAX_DOWNLOAD_BYTES || downloaded > asset.size {
                return Err(UpdateError::TooLarge);
            }
            output
                .write_all(&buffer[..count])
                .map_err(|_| UpdateError::File)?;
            progress(((downloaded.saturating_mul(100) / asset.size).min(100)) as u8);
        }
        output.flush().map_err(|_| UpdateError::File)?;
        if downloaded != asset.size {
            return Err(UpdateError::InvalidResponse);
        }

        let signature_text = String::from_utf8(read_small(signature_asset, MAX_SIGNATURE_BYTES)?)
            .map_err(|_| UpdateError::BadSignature)?;
        let public_key =
            PublicKey::from_base64(UPDATE_PUBKEY).map_err(|_| UpdateError::BadSignature)?;
        let signature =
            Signature::decode(&signature_text).map_err(|_| UpdateError::BadSignature)?;
        let mut verifier = public_key
            .verify_stream(&signature)
            .map_err(|_| UpdateError::BadSignature)?;
        let mut input = File::open(destination).map_err(|_| UpdateError::File)?;
        loop {
            let count = input.read(&mut buffer).map_err(|_| UpdateError::File)?;
            if count == 0 {
                break;
            }
            verifier.update(&buffer[..count]);
        }
        verifier.finalize().map_err(|_| UpdateError::BadSignature)?;
        Ok(destination.to_path_buf())
    })();

    if result.is_err() {
        let _ = fs::remove_file(destination);
    }
    result
}
