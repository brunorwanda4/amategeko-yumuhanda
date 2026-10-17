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

/// Rust project website.
pub const RUST_URL: &str = "https://www.rust-lang.org/";

/// GPUI project website.
pub const GPUI_URL: &str = "https://gpui.rs/";

/// GPUI Kit project website.
pub const GPUI_KIT_URL: &str = "https://gpui-kit.com/";

/// gpui-mobile source repository.
pub const GPUI_MOBILE_URL: &str = "https://github.com/longbridge/gpui-mobile";

/// Next.js project website.
pub const NEXTJS_URL: &str = "https://nextjs.org/";

/// Fumadocs project website.
pub const FUMADOCS_URL: &str = "https://fumadocs.dev/";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_link_starts_with_https_and_has_allowed_host() {
        let links = [
            SITE_URL,
            DOCS_URL,
            REPO_URL,
            RELEASES_URL,
            AUTHOR_URL,
            RUST_URL,
            GPUI_URL,
            GPUI_KIT_URL,
            GPUI_MOBILE_URL,
            NEXTJS_URL,
            FUMADOCS_URL,
        ];
        for link in &links {
            assert!(
                link.starts_with("https://"),
                "{link:?} must start with https://"
            );
            let host_start = "https://".len();
            let rest = &link[host_start..];
            let host = rest.split('/').next().unwrap_or("");
            assert!(
                host == "amategeko-yumuhanda-alpha.vercel.app"
                    || host == "github.com"
                    || host == "www.rust-lang.org"
                    || host == "gpui.rs"
                    || host == "gpui-kit.com"
                    || host == "nextjs.org"
                    || host == "fumadocs.dev",
                "{link:?} has unexpected host {host:?}"
            );
        }
    }
}
