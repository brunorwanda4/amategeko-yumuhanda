use anyhow::anyhow;
use gpui::{AssetSource, Result, SharedString};
use gpui_kit::assets::AllAssets;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[exclude = "fonts/*"]
pub struct EmbeddedAppAssets;

#[derive(Clone, Copy, Debug, Default)]
pub struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        let rel_path = path.strip_prefix("assets/").unwrap_or(path);

        // 1. Try embedded application assets (e.g. "images/q229.png")
        if let Some(file) = EmbeddedAppAssets::get(rel_path) {
            return Ok(Some(file.data));
        }
        if let Some(file) = EmbeddedAppAssets::get(path) {
            return Ok(Some(file.data));
        }

        // 2. Try gpui-kit AllAssets (for UI icons)
        if let Ok(Some(data)) = AllAssets.load(path) {
            return Ok(Some(data));
        }
        if let Ok(Some(data)) = AllAssets.load(rel_path) {
            return Ok(Some(data));
        }

        // 3. Fallback: local filesystem (e.g. during development)
        let candidates = [
            path.to_string(),
            format!("assets/{}", rel_path),
            format!("../../assets/{}", rel_path),
        ];
        for candidate in candidates {
            if Path::new(&candidate).is_file() {
                if let Ok(bytes) = std::fs::read(&candidate) {
                    return Ok(Some(Cow::Owned(bytes)));
                }
            }
        }

        Err(anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut list = Vec::new();
        let rel_path = path.strip_prefix("assets/").unwrap_or(path);

        for name in EmbeddedAppAssets::iter() {
            if name.starts_with(rel_path) {
                list.push(format!("assets/{}", name).into());
            }
        }

        if let Ok(all_list) = AllAssets.list(path) {
            list.extend(all_list);
        }

        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_question_images() {
        let assets = AppAssets;
        let img1 = assets
            .load("assets/images/q229.png")
            .expect("load assets/images/q229.png");
        assert!(img1.is_some(), "assets/images/q229.png should not be None");
        assert!(!img1.unwrap().is_empty());

        let img2 = assets
            .load("images/q229.png")
            .expect("load images/q229.png");
        assert!(img2.is_some(), "images/q229.png should not be None");

        let icon = assets
            .load("icons/keyboard.svg")
            .expect("load keyboard icon");
        assert!(icon.is_some());
    }
}
