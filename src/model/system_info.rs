use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};

use super::{AbsolutePath, AbsolutePathBuf};

#[derive(Debug)]
pub struct SystemInfo {
    pub os: &'static str,
    pub is_unix: bool,
    pub home: Arc<AbsolutePath>,
    pub root: Arc<AbsolutePath>,
    pub windows: WindowsPaths,
    pub mac: MacPaths,
    pub xdg: XdgPaths,
}

#[derive(Debug)]
pub struct WindowsPaths {
    pub local_app_data: Arc<AbsolutePath>,
    pub roaming_app_data: Arc<AbsolutePath>,
    pub local_low_app_data: Arc<AbsolutePath>,
    pub documents: Arc<AbsolutePath>,
}

#[derive(Debug)]
pub struct MacPaths {
    pub applications: Arc<AbsolutePath>,
}

#[derive(Debug)]
pub struct XdgPaths {
    pub config: Arc<AbsolutePath>,
    pub data: Arc<AbsolutePath>,
    pub state: Arc<AbsolutePath>,
    pub cache: Arc<AbsolutePath>,
}

impl SystemInfo {
    pub fn detect() -> Self {
        let home = HOME_DIR.clone();

        let root: Arc<AbsolutePath> = home
            .ancestors()
            .last()
            .expect("an absolute path always has a root")
            .into();

        let windows = WindowsPaths {
            local_app_data: env_path_or_default("LOCALAPPDATA", &home, "AppData/Local"),
            roaming_app_data: env_path_or_default("APPDATA", &home, "AppData/Roaming"),
            local_low_app_data: home.join("AppData/LocalLow").into(),
            documents: home.join("Documents").into(),
        };

        let mac = MacPaths {
            applications: root.join("Applications").into(),
        };

        let xdg = XdgPaths {
            config: env_path_or_default("XDG_CONFIG_HOME", &home, ".config"),
            data: env_path_or_default("XDG_DATA_HOME", &home, ".local/share"),
            state: env_path_or_default("XDG_STATE_HOME", &home, ".local/state"),
            cache: env_path_or_default("XDG_CACHE_HOME", &home, ".cache"),
        };

        Self {
            os: env::consts::OS,
            is_unix: cfg!(unix),
            home,
            root,
            windows,
            mac,
            xdg,
        }
    }
}

pub(super) static CURRENT_DIR: LazyLock<Arc<AbsolutePath>> = LazyLock::new(|| {
    let cwd = env::current_dir().expect("could not determine the current directory");
    AbsolutePathBuf::new(cwd)
        .expect("current directory must be an absolute path without indirection")
        .into()
});

pub(super) static HOME_DIR: LazyLock<Arc<AbsolutePath>> = LazyLock::new(|| {
    let home = env::home_dir().expect("could not determine the home directory");
    AbsolutePathBuf::new(home)
        .expect("home directory must be an absolute path without indirection")
        .into()
});

fn env_path_or_default(variable: &str, home: &AbsolutePath, default: &str) -> Arc<AbsolutePath> {
    env::var_os(variable)
        .map(PathBuf::from)
        .and_then(|path| normalize_path(&path))
        .unwrap_or_else(|| home.join(default))
        .into()
}

fn normalize_path(path: &Path) -> Option<AbsolutePathBuf> {
    let root = path.ancestors().last().and_then(AbsolutePath::new)?;
    Some(root.join(path))
}
