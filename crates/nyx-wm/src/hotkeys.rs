//! Gestion des raccourcis clavier globaux via l'API Win32 RegisterHotKey.
//!
//! Ce module associe des combinaisons de touches (modificateurs + touche virtuelle)
//! à des identifiants d'actions uniques pour le gestionnaire de fenêtres.

use tracing::{error, info, warn};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS, MOD_ALT, MOD_NOREPEAT, MOD_SHIFT, MOD_WIN, VK_RETURN,};

/// Identifiants uniques des actions déclenchées par raccourci clavier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyAction {
    /// Open a terminal (default shortcut:  Alt + Enter)
    SpawnTerminal = 1,
    /// Close the active window (default shortcut: Alt + Shift + Q)
    CloseWindow = 2,
    /// Restart the windows window manager (default shortcut: Alt + Shift + E)
    RescueExplorer = 3,
}

impl HotkeyAction {
    /// Convertit un identifiant numérique reçu de l'événement Windows en action.
    ///
    /// # Arguments
    ///
    /// * `id` - L'entier `i32` transmis par Windows dans le message `WM_HOTKEY`.
    ///
    /// # Returns
    ///
    /// Retourne `Some(HotkeyAction)` si l'identifiant est reconnu, sinon `None`.
    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(HotkeyAction::SpawnTerminal),
            2 => Some(HotkeyAction::CloseWindow),
            3 => Some(HotkeyAction::RescueExplorer),
            _ => None,
        }
    }
}

/// Enregistre l'ensemble des raccourcis clavier du gestionnaire auprès de l'OS.
///
/// # Returns
///
/// * `Ok(())` si tous les raccourcis ont été enregistrés.
/// * `Err(windows::core::Error)` si Windows a refusé l'enregistrement
///   (par exemple si une autre application utilise déjà cette combinaison).
///
/// # Safety
///
/// Appelle l'API FFI Win32 `RegisterHotKey`.
pub fn register_all_hotkeys() -> windows::core::Result<()> {
        unsafe {
            // Terminal
            RegisterHotKey(
                HWND::default(),
                HotkeyAction::SpawnTerminal as i32,
                MOD_ALT | MOD_NOREPEAT,
                VK_RETURN.0 as u32,
            )?;
            // Close active window
            RegisterHotKey(
                HWND::default(),
                HotkeyAction::CloseWindow as i32,
                MOD_ALT | MOD_SHIFT | MOD_NOREPEAT,
                b'Q' as u32,
            )?;
            // Rescue Explorer
            RegisterHotKey(
                HWND::default(),
                HotkeyAction::RescueExplorer as i32,
                MOD_ALT | MOD_SHIFT | MOD_NOREPEAT,
                b'E' as u32,
            )?;
        }

        info!("Hotkeys registered successfully");
        Ok(())
    }

/// Libère tous les raccourcis clavier enregistrés auprès de Windows.
///
/// Doit être appelée lors de l'arrêt du gestionnaire pour éviter de laisser
/// des raccourcis fantômes dans le système.
pub fn unregister_all_hotkeys() {
    for id in [1, 2, 3] {
        unsafe {
            let _ = UnregisterHotKey(HWND::default(), id);
        };
    }
    info!("Hotkeys unregistered successfully");
}