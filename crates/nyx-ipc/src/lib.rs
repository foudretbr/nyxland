use serde::{Deserialize, Serialize};

pub const DEFAULT_PIPE_NAME: &str = r"\\.\pipe\nyxland-ipc";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NyxCommand {
    SwitchWorkspace(usize),
    MoveToWorkspace(usize),
    ToggleFloat,
    SetLayout(String),
    Retile,
    FocusNext,
    FocusPrev,
    ReloadConfig,
    Quit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NyxResponse {
    Ok,
    Error(String),
    Status {
        active_workspace: usize,
        window_count: usize,
        layout: String,
    },
}
