use std::path::Path;

use serde::{Deserialize, Serialize};
use steam_shortcuts_util::{shortcut::ShortcutOwned, Shortcut};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ManifestItem {
    #[serde(alias = "LaunchExecutable")]
    pub launch_executable: String,

    #[serde(alias = "ManifestLocation")]
    pub manifest_location: String,

    #[serde(alias = "DisplayName")]
    pub display_name: String,

    #[serde(alias = "InstallLocation")]
    pub install_location: String,

    #[serde(alias = "AppName")]
    pub app_name: String,

    #[serde(alias = "CatalogNamespace")]
    pub catalog_namespace: String,

    #[serde(alias = "CatalogItemId")]
    pub catalog_item_id: String,
}

fn exe_shortcut(manifest: ManifestItem) -> ShortcutOwned {
    let exe = manifest.exe();
    let mut start_dir = manifest.install_location.clone();
    if !manifest.install_location.starts_with('"') {
        start_dir = format!("\"{}\"", manifest.install_location);
    }
    Shortcut::new(
        0,
        manifest.display_name.as_str(),
        exe.as_str(),
        start_dir.as_str(),
        "",
        "",
        "",
    )
    .to_owned()
}

fn launcher_shortcut(manifest: ManifestItem) -> ShortcutOwned {
    let icon = manifest.exe();
    let options = manifest.get_launch_url();
    Shortcut::new(
        0,
        manifest.display_name.as_str(),
        "explorer.exe",
        "",
        icon.as_str(),
        "",
        options.as_str(),
    )
    .to_owned()
}

impl From<ManifestItem> for ShortcutOwned {
    fn from(manifest: ManifestItem) -> Self {
        let mut owned_shortcut = if manifest.needs_launcher() {
            launcher_shortcut(manifest)
        } else {
            exe_shortcut(manifest)
        };
        owned_shortcut.tags.push("EGS".to_owned());
        owned_shortcut.tags.push("Ready TO Play".to_owned());
        owned_shortcut.tags.push("Installed".to_owned());
        owned_shortcut
    }
}

impl ManifestItem {
    fn exe(&self) -> String {
        let manifest = self;
        let exe_path = Path::new(&manifest.install_location)
            .join(&manifest.launch_executable)
            .to_string_lossy()
            .to_string();
        let exe = format!("\"{}\"", exe_path);
        exe
    }

    fn get_launch_url(&self) -> String {
        format!(
            "com.epicgames.launcher://apps/{}%3A{}%3A{}?action=launch&silent=true",
            self.catalog_namespace, self.catalog_item_id, self.app_name
        )
    }
    fn needs_launcher(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_parse_item() {
        let json = include_str!("example_item.json");
        let _: ManifestItem = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn generates_launch_string() {
        let json = include_str!("example_item.json");
        let expected ="com.epicgames.launcher://apps/2a09fb19b47f46dfb11ebd382f132a8f%3A88f4bb0bb06e4962a2042d5e20fb6ace%3A63a665088eb1480298f1e57943b225d8?action=launch&silent=true";

        let manifest: ManifestItem = serde_json::from_str(json).unwrap();
        let actual = manifest.get_launch_url();

        assert_eq!(expected, actual);
    }
}
