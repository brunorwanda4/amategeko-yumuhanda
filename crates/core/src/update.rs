//! Pure release parsing and update-selection rules.

use semver::Version;
use serde::Deserialize;

pub const UPDATE_PUBKEY: &str = "RWQUzcF1TolAOFbbebadlmvfd4GuFR/cGA4LyE/QONbVbnRgwlZmW2nR";
pub const CHECK_INTERVAL_SECS: u64 = 24 * 60 * 60;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Release {
    pub tag_name: String,
    #[serde(default)]
    pub body: String,
    pub html_url: String,
    #[serde(default)]
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdatePlatform {
    Windows,
    Android,
}

pub fn parse_release(json: &str) -> Result<Release, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn is_newer(current: &str, tag: &str) -> bool {
    let Ok(current) = Version::parse(current.trim_start_matches('v')) else {
        return false;
    };
    let Ok(candidate) = Version::parse(tag.trim_start_matches('v')) else {
        return false;
    };
    candidate > current
}

pub fn pick_asset(
    release: &Release,
    platform: UpdatePlatform,
) -> Option<(ReleaseAsset, ReleaseAsset)> {
    let version = release.tag_name.trim_start_matches('v');
    let suffix = match platform {
        UpdatePlatform::Windows => "windows-setup.exe",
        UpdatePlatform::Android => "android-arm64.apk",
    };
    let name = format!("amategeko-yumuhanda-{version}-{suffix}");
    let signature_name = format!("{name}.minisig");
    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name == name)?
        .clone();
    let signature = release
        .assets
        .iter()
        .find(|asset| asset.name == signature_name)?
        .clone();
    Some((asset, signature))
}

pub fn should_check(now: u64, last_check: Option<u64>, auto_enabled: bool) -> bool {
    auto_enabled
        && last_check
            .map(|last| now.saturating_sub(last) >= CHECK_INTERVAL_SECS)
            .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_rules() {
        assert!(is_newer("0.1.0", "v0.2.0"));
        assert!(is_newer("0.9.0", "0.10.0"));
        assert!(!is_newer("1.0.0", "v0.9.0"));

        let release = Release {
            tag_name: "v1.2.3".into(),
            body: String::new(),
            html_url: "https://github.com/brunorwanda4/amategeko-yumuhanda/releases/tag/v1.2.3"
                .into(),
            assets: vec![ReleaseAsset {
                name: "amategeko-yumuhanda-1.2.3-windows-setup.exe".into(),
                browser_download_url: "https://github.com/example".into(),
                size: 1,
            }],
        };
        assert!(pick_asset(&release, UpdatePlatform::Windows).is_none());
        assert!(pick_asset(&release, UpdatePlatform::Android).is_none());

        assert!(!should_check(1_000, Some(999), true));
        assert!(should_check(1_000 + CHECK_INTERVAL_SECS, Some(1_000), true));
        assert!(!should_check(u64::MAX, None, false));
    }

    #[test]
    fn update_public_key_is_valid() {
        assert!(minisign_verify::PublicKey::from_base64(UPDATE_PUBKEY).is_ok());
    }
}
