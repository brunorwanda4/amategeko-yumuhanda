use anyhow::anyhow;
use gpui::{AssetSource, Result, SharedString};
use gpui_kit::assets::icon_assets;
use gpui_kit::assets::Assets;
#[cfg(not(target_arch = "wasm32"))]
use rust_embed::RustEmbed;
use std::borrow::Cow;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
#[derive(RustEmbed)]
#[folder = "../../assets"]
#[exclude = "fonts/*"]
pub struct EmbeddedAppAssets;

icon_assets!(
    pub KitIcons,
    [
        ALargeSmall,
        ArrowDown,
        ArrowLeft,
        ArrowRight,
        ArrowUp,
        Asterisk,
        Battery,
        BatteryCharging,
        BatteryFull,
        BatteryLow,
        BatteryMedium,
        BatteryWarning,
        Bell,
        BookOpen,
        Bot,
        Building2,
        Calendar,
        CaseSensitive,
        ChartPie,
        Check,
        ChevronDown,
        ChevronLeft,
        ChevronRight,
        ChevronUp,
        ChevronsUpDown,
        CircleAlert,
        CircleCheck,
        CircleUser,
        CircleX,
        Clock,
        Close,
        Copy,
        Cpu,
        Dash,
        Delete,
        Ellipsis,
        EllipsisVertical,
        ExternalLink,
        Eye,
        EyeOff,
        File,
        FileText,
        Flame,
        Folder,
        FolderClosed,
        FolderOpen,
        Frame,
        GalleryVerticalEnd,
        Github,
        Globe,
        GraduationCap,
        HardDrive,
        Heart,
        HeartOff,
        House,
        Inbox,
        Info,
        Inspector,
        LayoutDashboard,
        Loader,
        LoaderCircle,
        Map,
        Maximize,
        MemoryStick,
        Menu,
        Minimize,
        Minus,
        Moon,
        Network,
        Palette,
        PanelBottom,
        PanelBottomOpen,
        PanelLeft,
        PanelLeftClose,
        PanelLeftOpen,
        PanelRight,
        PanelRightClose,
        PanelRightOpen,
        Pause,
        Play,
        Plus,
        Redo,
        Redo2,
        Replace,
        ResizeCorner,
        RotateCw,
        Search,
        Settings,
        Settings2,
        SortAscending,
        SortDescending,
        SquareTerminal,
        Star,
        StarFill,
        StarOff,
        Sun,
        Target,
        ThumbsDown,
        ThumbsUp,
        TriangleAlert,
        Undo,
        Undo2,
        User,
        WindowClose,
        WindowMaximize,
        WindowMinimize,
        WindowRestore,
    ]
);

#[derive(Clone, Copy, Debug, Default)]
pub struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        let rel_path = path.strip_prefix("assets/").unwrap_or(path);

        // 1. Try embedded application assets (e.g. "images/q229.png")
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(file) = EmbeddedAppAssets::get(rel_path) {
                return Ok(Some(file.data));
            }
            if let Some(file) = EmbeddedAppAssets::get(path) {
                return Ok(Some(file.data));
            }
        }

        // 2. Try embedded UI kit icons (instant synchronous load for native & wasm)
        if let Ok(Some(data)) = KitIcons.load(path) {
            return Ok(Some(data));
        }
        if let Ok(Some(data)) = KitIcons.load(rel_path) {
            return Ok(Some(data));
        }

        // 3. Try gpui-kit Assets (for UI icons)
        let kit_assets = Assets::new("");
        if let Ok(Some(data)) = kit_assets.load(path) {
            return Ok(Some(data));
        }
        if let Ok(Some(data)) = kit_assets.load(rel_path) {
            return Ok(Some(data));
        }

        // 4. Fallback: local filesystem (e.g. during development)
        #[cfg(not(target_arch = "wasm32"))]
        {
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
        }

        Err(anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut list = Vec::new();

        #[cfg(not(target_arch = "wasm32"))]
        {
            let rel_path = path.strip_prefix("assets/").unwrap_or(path);
            for name in EmbeddedAppAssets::iter() {
                if name.starts_with(rel_path) {
                    list.push(format!("assets/{}", name).into());
                }
            }
        }

        if let Ok(kit_list) = KitIcons.list(path) {
            list.extend(kit_list);
        }

        let kit_assets = Assets::new("");
        if let Ok(all_list) = kit_assets.list(path) {
            list.extend(all_list);
        }

        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_assets_loads_png() {
        let assets = AppAssets;
        let result = assets.load("images/q229.png");
        assert!(result.is_ok(), "Expected Ok result");
        assert!(result.unwrap().is_some(), "Expected Some data");
    }

    #[test]
    fn app_assets_handles_assets_prefix() {
        let assets = AppAssets;
        let result = assets.load("assets/images/q229.png");
        assert!(result.is_ok(), "Expected Ok result");
        assert!(result.unwrap().is_some(), "Expected Some data");
    }

    #[test]
    fn app_assets_loads_icons() {
        let assets = AppAssets;
        let play = assets.load("icons/play.svg");
        assert!(play.is_ok(), "Expected Ok result for icons/play.svg");
        assert!(
            play.unwrap().is_some(),
            "Expected Some data for icons/play.svg"
        );

        let book = assets.load("icons/book-open.svg");
        assert!(book.is_ok(), "Expected Ok result for icons/book-open.svg");
        assert!(
            book.unwrap().is_some(),
            "Expected Some data for icons/book-open.svg"
        );
    }

    #[test]
    fn app_assets_empty_path_returns_none() {
        let assets = AppAssets;
        let result = assets.load("");
        assert!(result.is_ok(), "Expected Ok result");
        assert!(result.unwrap().is_none(), "Expected None for empty path");
    }

    #[test]
    fn app_assets_nonexistent_returns_err() {
        let assets = AppAssets;
        let result = assets.load("nonexistent/path/file.xyz");
        assert!(result.is_err(), "Expected Err for nonexistent asset");
    }
}
