//! Actions exécutables par le gestionnaire de fenêtres Nyxland.
//!
//! Ce module contient les fonctions déclenchées en réponse aux raccourcis clavier
//! ou aux messages IPC (lancement de terminal, fermeture de fenêtre, etc.).

use tracing::{info, error, warn};
use std::os::windows::process::CommandExt;
use std::process::Command;
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, PostMessageW, WM_CLOSE};

const CREATE_NEW_CONSOLE: u32 = 0x00000010;

/// Démarre un terminal Windows en utilisant une stratégie de repli (*fallback*).
///
/// Cette fonction tente de lancer un terminal dans l'ordre de préférence suivant :
/// 1. **Windows Terminal** (`wt.exe`)
/// 2. **PowerShell** (`powershell.exe`)
/// 3. **L'invite de commandes** (`cmd.exe`)
///
/// # Return
///
/// Retourne `Ok(())` si l'un des terminaux a pu être démarré avec succès.
/// Retourne une `std::io::Error` si aucun terminal n'a pu être exécuté.
///
/// # Examples
///
/// ```rust
/// match spawn_terminal() {
///     Ok(_) => println!("Terminal lancé avec succès !"),
///     Err(e) => eprintln!("Impossible de lancer un terminal : {}", e),
/// }
/// ```
pub fn spawn_terminal() -> Result<(), std::io::Error> {
    let candidates = ["wt.exe", "powershell.exe", "cmd.exe"];
    let mut last_error = None;

    for &cmd in &candidates {
        let mut command = Command::new(cmd);
        if cmd != "wt.exe" {
            command.creation_flags(CREATE_NEW_CONSOLE);
        }

        match command.spawn() {
            Ok(_) => {
                info!("Terminal lancé avec succés : {}", cmd);
                return Ok(());
            }
            Err(e) => {
                warn!(error = %e, "Echec du lancement de {}", cmd);
                last_error = Some(e);
            }
        }
    }

    error!("Critique : aucun terminal n'a pu être démarré.");
    Err(last_error.unwrap_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Aucun terminal trouvé")
    }))
}

/// Envoie une demande de fermeture polie à la fenêtre actuellement au premier plan.
///
/// Cette fonction récupère la fenêtre active et lui transmet le message système
/// Win32 `WM_CLOSE`. Cela équivaut à cliquer sur la croix rouge ou à faire Alt+F4.
///
/// # Returns
///
/// * `true` si une fenêtre active valide a été trouvée et que le message a été envoyé.
/// * `false` s'il n'y avait aucune fenêtre active ou si l'envoi a échoué.
///
/// # Safety
///
/// Cette fonction appelle les fonctions FFI Win32 `GetForegroundWindow` et `PostMessageW`.
/// L'appel est considéré sûr car :
/// * `GetForegroundWindow` ne prend aucun pointeur et renvoie un handle opaque géré par l'OS.
/// * `PostMessageW` dépose le message de manière asynchrone sans bloquer notre thread.
pub fn close_active_window() -> bool {
    let hwnd = unsafe {GetForegroundWindow()};

    // Verification -> si aucunes fenetre valide, on quitte direct.
    if hwnd.is_invalid() {
        warn!("No valid window to close !");
        return false;
    }
    // Sinon on envoie le message et on renvoie true ou false.
    let result = unsafe {PostMessageW(hwnd, WM_CLOSE, WPARAM(0), LPARAM(0))};

    if result.is_ok() {
        info!("Close request sent successfully.");
        true
    } else {
        warn!("Failed to send close request.");
        false
    }
}

/// Relance le shell officiel Windows (Explorateur Windows).
///
/// Cette fonction sert de parachute de secours dans l'éventualité où
/// l'utilisateur a besoin de retrouver l'environnement de bureau Windows
/// standard (barre des tâches, zone de notification, menu Démarrer).
///
/// # Returns
///
/// * `Ok(())` si le processus `explorer.exe` a pu être détaché et lancé avec succès.
/// * `Err(std::io::Error)` si le système n'a pas pu exécuter `explorer.exe`.
///
/// # Examples
///
/// ```rust
/// match spawn_explorer() {
///     Ok(_) => println!("Explorer relancé avec succès"),
///     Err(e) => eprintln!("Erreur lors du déploiement du parachute : {}", e),
/// }
/// ```
pub fn spawn_explorer() -> Result<(), std::io::Error> {
    match Command::new("explorer.exe").spawn() {
        Ok(_) => {
            info!("Explorer.exe launched successfully");
            return Ok(());
        }
        Err(e) => {
            eprintln!("An error occurred while launching explorer.exe: {}", e);
            return Err(e);
        }
    }
}