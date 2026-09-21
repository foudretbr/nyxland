mod window;
mod actions;
mod hotkeys;

use hotkeys::HotkeyAction;
use nyx_config::NyxConfig;
use nyx_core::{Rect, Workspace};
use std::sync::{Arc, Mutex};
use tracing::{error, info, warn};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, TranslateMessage, MSG, WM_HOTKEY,
};

struct WindowManager {
    config: NyxConfig,
    workspaces: Vec<Workspace>,
    active_workspace: usize,
}

impl WindowManager {
    pub fn new(config: NyxConfig) -> Self {
        let mut workspaces = Vec::new();
        for i in 1..=config.default_workspaces {
            workspaces.push(Workspace::new(i, format!("WS-{}", i)));
        }

        Self {
            config,
            workspaces,
            active_workspace: 0,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    println!("====================================================");
    println!("  Nyxland Window Manager démarré avec succès !");
    println!("  Raccourcis actifs :");
    println!("    * Alt + Entrée     : Ouvrir un terminal");
    println!("    * Alt + Shift + Q  : Fermer la fenêtre active");
    println!("    * Alt + Shift + E  : Relancer Explorer");
    println!("  Appuyez sur Ctrl + C pour quitter proprement.");
    println!("====================================================");

    info!("Starting Nyxland Window Manager daemon...");
    
    let config = NyxConfig::default();
    let wm = Arc::new(Mutex::new(WindowManager::new(config)));

    info!("Nyxland initialized with {} workspaces", wm.lock().unwrap().workspaces.len());
    info!("Listening for window management events and IPC commands on {}", nyx_ipc::DEFAULT_PIPE_NAME);

    // Thread d'écoute des raccourcis clavier Win32
    std::thread::spawn(|| {
        if let Err(e) = hotkeys::register_all_hotkeys() {
            error!("Impossible d'enregistrer les raccourcis clavier : {:?}", e);
            return;
        }

        info!("Nyxland est prêt ! Appuyez sur Alt + Entrée pour lancer un terminal.");

        let mut msg = MSG::default();
        while unsafe { GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() } {
            if msg.message == WM_HOTKEY {
                let id = msg.wParam.0 as i32;
                match HotkeyAction::from_id(id) {
                    Some(HotkeyAction::SpawnTerminal) => {
                        let _ = actions::spawn_terminal();
                    }
                    Some(HotkeyAction::CloseWindow) => {
                        actions::close_active_window();
                    }
                    Some(HotkeyAction::RescueExplorer) => {
                        let _ = actions::spawn_explorer();
                    }
                    None => {
                        warn!("Message WM_HOTKEY reçu avec un ID inconnu : {}", id);
                    }
                }
            }

            // Dispatch en dehors du if pour traiter tous les messages
            unsafe {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        } // Ferme la boucle while

        hotkeys::unregister_all_hotkeys();
    }); // Ferme le thread::spawn

    // Garde le daemon en vie jusqu'au Ctrl + C
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Nyxland Window Manager.");
    hotkeys::unregister_all_hotkeys();
    Ok(())
}