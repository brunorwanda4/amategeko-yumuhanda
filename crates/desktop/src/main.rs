use amategeko_app::{AppState, ShellView};
use amategeko_core::{Storage, SystemClock};
use directories::ProjectDirs;
use gpui_kit::assets::Assets;
use gpui_kit::component::Root;
use gpui_kit::*;
use std::path::PathBuf;
use std::sync::Arc;

struct DesktopStorage {
    storage_dir: PathBuf,
}

impl Storage for DesktopStorage {
    fn storage_dir(&self) -> PathBuf {
        self.storage_dir.clone()
    }
}

fn main() {
    let storage_dir =
        if let Some(proj_dirs) = ProjectDirs::from("rw", "amategeko", "AmategekoYumuhanda") {
            proj_dirs.data_dir().to_path_buf()
        } else {
            PathBuf::from("data")
        };

    let storage = Arc::new(DesktopStorage { storage_dir });
    let clock = Arc::new(SystemClock);

    gpui_kit::application()
        .with_assets(Assets)
        .run(move |cx: &mut App| {
            gpui_kit::init(cx);

            let app_state = AppState::new(storage.clone(), clock.clone());

            let options = WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Amategeko y'Umuhanda".into()),
                    ..Default::default()
                }),
                ..Default::default()
            };

            cx.open_window(options, |window, cx| {
                let shell = cx.new(|_| ShellView::new(app_state));
                cx.new(|cx| Root::new(shell, window, cx))
            })
            .expect("Failed to open desktop window");
        });
}
