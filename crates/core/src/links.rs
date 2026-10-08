//! Canonical URLs for the project's public web presence.
//!
//! All external links the app is allowed to open live here.
//! Keep in sync with the rule in AGENTS.md.

use crate::about::PROJECT_CREDIT;

/// The project's public website (also the docs root).
pub const SITE_URL: &str = "https://amategeko-yumuhanda-alpha.vercel.app/";

/// Online user guide / documentation.
pub const DOCS_URL: &str = "https://amategeko-yumuhanda-alpha.vercel.app/docs";

/// GitHub source-code repository.
pub const REPO_URL: &str = "https://github.com/brunorwanda4/amategeko-yumuhanda";

/// GitHub Releases page (downloads / changelog).
pub const RELEASES_URL: &str = "https://github.com/brunorwanda4/amategeko-yumuhanda/releases";

/// Author's public GitHub profile (re-uses the value from `about`).
pub const AUTHOR_URL: &str = PROJECT_CREDIT.url;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_link_starts_with_https_and_has_allowed_host() {
        let links = [SITE_URL, DOCS_URL, REPO_URL, RELEASES_URL, AUTHOR_URL];
        for link in &links {
            assert!(
                link.starts_with("https://"),
                "{link:?} must start with https://"
            );
            let host_start = "https://".len();
            let rest = &link[host_start..];
            let host = rest.split('/').next().unwrap_or("");
            assert!(
                host == "amategeko-yumuhanda-alpha.vercel.app" || host == "github.com",
                "{link:?} has unexpected host {host:?}"
            );
        }
    }
}
