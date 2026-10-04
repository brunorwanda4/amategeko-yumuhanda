/// Author credit shown in the app and linked from its footer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectCredit {
    /// Display name used in the author credit.
    pub name: &'static str,
    /// Public profile opened from the author credit.
    pub url: &'static str,
}

/// The project's single source of truth for the author name and profile URL.
pub const PROJECT_CREDIT: ProjectCredit = ProjectCredit {
    name: "Rwanda Bruno",
    url: "https://github.com/brunorwanda4",
};

/// Application version sourced from the workspace package metadata.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
