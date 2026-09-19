use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NyxConfig {
    pub gaps: i32,
    pub border_width: i32,
    pub border_color_active: String,
    pub border_color_inactive: String,
    pub default_workspaces: usize,
    #[serde(default)]
    pub ignored_classes: Vec<String>,
}

impl Default for NyxConfig {
    fn default() -> Self {
        Self {
            gaps: 8,
            border_width: 2,
            border_color_active: "#bd93f9".to_string(),
            border_color_inactive: "#44475a".to_string(),
            default_workspaces: 9,
            ignored_classes: vec![
                "Shell_TrayWnd".to_string(),
                "Windows.UI.Core.CoreWindow".to_string(),
                "Progman".to_string(),
                "WorkerW".to_string(),
                "TopLevelWindowForOverflowXamlIsland".to_string(),
            ],
        }
    }
}

impl NyxConfig {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}
