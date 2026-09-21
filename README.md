# Nyxland 🌌

> Un **Tiling Window Manager** moderne, performant et modulaire pour Windows (inspiré par l'expérience de Hyprland et Komorebi).

---

## 📁 Architecture du Workspace

Le projet est structuré sous forme de workspace Cargo multi-crates :

- **`crates/nyx-core`** : Moteur géométrique, algorithmes de tuilage (BSP, Columns, Monocle, Floating), gestionnaires d'écrans et d'espaces de travail (workspaces).
- **`crates/nyx-ipc`** : Protocole de communication IPC via Named Pipe (`\\.\pipe\nyxland-ipc`).
- **`crates/nyx-config`** : Chargeur de configuration TOML (marges, gaps, règles de fenêtres exclues, couleurs de bordures).
- **`crates/nyx-wm`** : Démon principal sous Windows (gestion des hooks WinEvent, boucles de messages Win32, application sécurisée des positions avec `SetWindowPos`).
- **`crates/nyx-cli` (`nyxc`)** : Binaire client en ligne de commande pour contrôler Nyxland (changer de workspace, basculer le tuilage, recharger la config).

---

## ⚙️ Configuration (`nyxland.toml`)

Le fichier de configuration principal est situé à la racine : `nyxland.toml`.

```toml
gaps = 8
border_width = 2
border_color_active = "#bd93f9"
border_color_inactive = "#44475a"
default_workspaces = 9

ignored_classes = [
    "Shell_TrayWnd",
    "Windows.UI.Core.CoreWindow",
    "Progman",
    "WorkerW",
    "TopLevelWindowForOverflowXamlIsland"
]
```

---

## 🖥️ Environnement de Test en VM

Tester un Window Manager directement sur la machine hôte peut capturer le focus ou figer l'explorateur en cours de dev.
Le dossier de stockage pour votre VM Windows a été préparé dans :
`D:\Logiciel ou VM  etc\VMs\Windows_Dev`

Pour automatiser l'installation de l'hyperviseur :
```powershell
.\scripts\setup_vm.ps1 -Hypervisor VirtualBox
```

# Windows Alternative Desktop Environment — Master Roadmap

## 0. Vision

Build a complete alternative desktop environment for Windows inspired by the philosophy of Arch Linux + Hyprland + Omarchy:

- Windows remains the underlying operating system.
- Windows NT, the kernel, security model, drivers, networking, audio, and required application APIs remain available.
- The traditional Windows desktop experience is replaced as deeply as practical.
- The project is not intended to be a simple window-management utility such as Komorebi.
- The environment has its own event model, window management, workspaces, shell, renderer, launcher, bar, notifications, configuration system, plugin system, recovery system, IPC and CLI.
- Lua is the primary configuration and extension language.
- A Zsh/Unix-like terminal workflow can be integrated.
- Windows applications and games remain a first-class compatibility target.
- Performance is a first-class requirement: the desktop should be lightweight and should not add meaningful gaming overhead.

Target philosophy:

> Windows becomes the underlying platform; the project's desktop environment becomes the user's actual desktop.

---

# 1. Core Principles

## 1.1 Keep the Windows platform intact initially

Do not begin by modifying:

- Windows kernel
- WDDM
- GPU drivers
- security boundaries
- bootloader
- undocumented kernel internals

Initially remain in user mode and use documented Windows interfaces wherever possible.

Deep replacement of Windows graphical internals is a later research phase.

## 1.2 Native core, Lua control plane

Recommended model:

```text
Rust/C++
    |
    +-- Core
    +-- Window system
    +-- Event system
    +-- Layout engine
    +-- Renderer
    +-- IPC
    +-- Platform integration
            |
            +-- Lua runtime
                    |
                    +-- configuration
                    +-- keybindings
                    +-- rules
                    +-- themes
                    +-- scripting
                    +-- plugins
```

Lua should provide flexibility.

Rust/C++ should perform performance-sensitive work.

Avoid frame-by-frame rendering or heavy layout work in Lua.

## 1.3 Reliability before depth

Every new layer must have:

1. Unit tests.
2. Integration tests.
3. Failure/recovery tests.
4. VM tests.
5. Performance measurements where relevant.

---

# 2. Final Architecture

```text
                         WINDOWS
                            |
                +-----------+-----------+
                |                       |
             NT Kernel              Drivers
                |                       |
                +-----------+-----------+
                            |
                     Windows APIs
                            |
              +-------------v-------------+
              |      YOUR DESKTOP        |
              |                           |
              | Core                      |
              | Window System             |
              | Event/Input System        |
              | Workspace Manager         |
              | Layout Engine             |
              | Renderer                  |
              | Shell                     |
              | Launcher                  |
              | Bar                       |
              | Notifications             |
              | Settings                  |
              | IPC                       |
              | Recovery                  |
              | Plugin System             |
              | Lua Runtime               |
              +-------------+-------------+
                            |
                  +---------+---------+
                  |                   |
             Applications          Games
                  |                   |
              Win32/.NET         DirectX/etc.
```

---

# 3. Repository Structure

Initial repository:

```text
windows-desktop/
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── Cargo.toml
├── Cargo.lock
│
├── crates/
│   ├── core/
│   ├── platform-windows/
│   ├── window/
│   ├── input/
│   ├── workspace/
│   ├── layout/
│   ├── renderer/
│   ├── shell/
│   ├── launcher/
│   ├── notifications/
│   ├── config/
│   ├── lua/
│   ├── ipc/
│   ├── cli/
│   ├── plugins/
│   └── recovery/
│
├── tests/
│   ├── integration/
│   ├── window/
│   ├── input/
│   ├── workspace/
│   ├── layout/
│   ├── lua/
│   ├── config/
│   └── recovery/
│
├── examples/
│   ├── basic-config/
│   └── sample-rice/
│
├── docs/
│   ├── architecture/
│   ├── windows/
│   ├── lua/
│   ├── testing/
│   └── development/
│
├── scripts/
│   ├── build.ps1
│   ├── test.ps1
│   └── benchmark.ps1
│
└── config/
    └── default/
        ├── config.lua
        ├── keybinds.lua
        ├── rules.lua
        └── theme.lua
```

Keep platform-specific code isolated.

---

# 4. Phase 0 — Project Specification

Difficulty: 1/10

## Objectives

Define exactly what the project is and is not.

Create:

```text
docs/architecture/project-scope.md
```

Document:

- supported Windows versions
- minimum hardware
- supported GPU vendors
- supported applications
- gaming compatibility goals
- security model
- update policy
- Lua API stability policy
- plugin policy
- recovery requirements

## Acceptance criteria

The project must have a written boundary:

```text
IN SCOPE
- desktop shell
- window system
- input
- workspaces
- layouts
- Lua
- renderer
- launcher
- bar
- notifications
- themes
- plugins
- IPC
- CLI
- recovery

OUT OF SCOPE initially
- kernel
- GPU driver
- WDDM replacement
- Windows bootloader
```

---

# 5. Phase 1 — Development Environment

Difficulty: 2/10

## 5.1 Use a VM

Development must initially happen inside a Windows VM.

Recommended structure:

```text
Physical machine
|
+-- Windows host
    |
    +-- Windows development VM
         |
         +-- Git
         +-- Rust
         +-- Visual Studio Build Tools
         +-- Windows SDK
         +-- debugger
         +-- project
```

Use VM snapshots/checkpoints.

Before risky changes:

```text
Snapshot
  |
  +-- modify system
  +-- reboot
  +-- test
  |
  +-- rollback if broken
```

## 5.2 Recommended VM strategy

Have at least:

```text
VM-STABLE
    normal supported Windows release

VM-TEST
    aggressive development

VM-FUTURE
    newer Windows build for compatibility testing
```

Do not make the main physical machine your first shell-replacement test system.

## 5.3 Toolchain

Install:

- Git
- Rust toolchain
- rustfmt
- Clippy
- Visual Studio Build Tools
- Windows SDK
- debugger
- terminal
- editor/IDE

Verify:

```powershell
rustc --version
cargo --version
git --version
```

Create a test project:

```powershell
cargo new desktop-core
cd desktop-core
cargo run
```

## Acceptance tests

- Rust builds.
- Windows executable starts.
- Debugger attaches.
- VM snapshot restores.
- Project can be rebuilt from a clean checkout.

---

# 6. Phase 2 — Workspace and Build Infrastructure

Difficulty: 3/10

Create a Cargo workspace.

Example:

```toml
[workspace]
members = [
    "crates/core",
    "crates/platform-windows",
    "crates/window",
    "crates/input",
    "crates/workspace",
    "crates/layout",
    "crates/renderer",
    "crates/shell",
    "crates/lua",
    "crates/ipc",
]
```

Add CI for:

- formatting
- Clippy
- unit tests
- release build

Commands:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test --workspace
cargo build --release
```

---

# 7. Phase 3 — Core Runtime

Difficulty: 4/10

Create:

```text
core/
├── runtime
├── state
├── event_bus
├── logging
├── errors
└── lifecycle
```

The runtime controls:

```text
Initialize
    |
Load platform
    |
Load configuration
    |
Initialize input
    |
Initialize windows
    |
Initialize renderer
    |
Initialize shell
    |
Start event loop
```

## Lifecycle

```text
BOOT
  |
INITIALIZE
  |
READY
  |
RUNNING
  |
STOPPING
  |
STOPPED
```

## Logging

Every subsystem needs structured logs:

```text
INFO  core initialized
INFO  platform initialized
INFO  Lua configuration loaded
INFO  renderer initialized
WARN  application returned unsupported window state
ERROR renderer initialization failed
```

Never rely on random debug prints in production.

---

# 8. Phase 4 — Windows Inspection Tools

Difficulty: 4/10

Before controlling Windows, understand it.

Build diagnostic tools:

```text
window-inspector.exe
process-inspector.exe
monitor-inspector.exe
input-inspector.exe
```

## Window inspector

Display:

```text
Process: firefox.exe
PID: 4212
Window handle: ...
Title: Mozilla Firefox
Position: 100,50
Size: 1200x800
Visible: true
Minimized: false
Maximized: false
```

Test with:

- Notepad
- browser
- VS Code
- terminal
- file manager
- several windows
- popup windows

## Goal

Build a reliable mapping:

```text
Windows native object
        |
        v
Your internal object
```

---

# 9. Phase 5 — Platform Abstraction

Difficulty: 5/10

Do not scatter Windows API calls across the project.

Create a platform boundary:

```rust
trait Platform {
    fn initialize(&mut self) -> Result<()>;
    fn enumerate_windows(&self) -> Result<Vec<NativeWindow>>;
    fn monitors(&self) -> Result<Vec<Monitor>>;
}
```

Then:

```text
Core
 |
 +-- Platform trait
       |
       +-- Windows implementation
```

All Windows-specific behavior goes under:

```text
crates/platform-windows/
```

This is critical for Windows Update compatibility.

---

# 10. Phase 6 — Window Model

Difficulty: 6/10

Create an internal window representation:

```rust
struct Window {
    id: WindowId,
    process_id: u32,
    title: String,
    rect: Rect,
    state: WindowState,
}
```

States:

```text
Normal
Minimized
Maximized
Fullscreen
Hidden
Floating
Tiled
```

Events:

```text
Created
Destroyed
Focused
Unfocused
Moved
Resized
Minimized
Restored
Maximized
```

## Tests

Open and close applications repeatedly.

Verify that internal state never contains stale windows.

Test:

```text
create -> focus -> move -> resize -> minimize -> restore -> close
```

---

# 11. Phase 7 — Event System

Difficulty: 7/10

Build the project's central event architecture.

```rust
enum Event {
    WindowCreated(WindowId),
    WindowDestroyed(WindowId),
    WindowFocused(WindowId),
    KeyPressed(Key),
    KeyReleased(Key),
    MouseMoved(Point),
    MonitorChanged,
    WorkspaceChanged(WorkspaceId),
}
```

Pipeline:

```text
Windows input/events
        |
        v
Native event translator
        |
        v
Your Event
        |
        v
Event dispatcher
        |
        +--> Window manager
        +--> Workspace manager
        +--> Lua
        +--> UI
```

The event system must be deterministic.

## Tests

- key press
- key release
- mouse movement
- window creation
- window destruction
- monitor change
- focus changes

---

# 12. Phase 8 — Input System

Difficulty: 7/10

Build:

```text
InputManager
Keymap
ModifierState
MouseState
KeybindingEngine
```

Represent input independently from Windows.

Example:

```text
SUPER + H
```

becomes:

```text
KeyChord {
    modifiers: SUPER,
    key: H
}
```

Then the keybinding engine maps it to an action.

---

# 13. Phase 9 — Keybinding Engine

Difficulty: 5/10

Lua example:

```lua
bind("SUPER", "ENTER", function()
    spawn("terminal")
end)

bind("SUPER", "Q", function()
    window.close_focused()
end)

bind("SUPER", "H", function()
    window.focus_left()
end)

bind("SUPER", "L", function()
    window.focus_right()
end)
```

Support:

- modifier combinations
- repeated keys
- sequences if desired
- configurable conflict detection

## Test

If two bindings conflict, the configuration loader must report:

```text
Configuration error:
binding conflict:
SUPER+ENTER
```

---

# 14. Phase 10 — Lua Runtime

Difficulty: 6/10

Integrate Lua into the native runtime.

Architecture:

```text
Rust
 |
 +-- Lua VM
      |
      +-- Config API
      +-- Window API
      +-- Workspace API
      +-- Input API
      +-- System API
```

Configuration path:

```text
User config
    |
    v
Lua parser/runtime
    |
    v
validated internal configuration
```

Never allow malformed Lua configuration to destroy the entire session.

---

# 15. Phase 11 — Lua API

Difficulty: 7/10

Initial API:

```lua
window.list()
window.focus(id)
window.close(id)
window.move(id, x, y)
window.resize(id, width, height)

workspace.create()
workspace.focus(1)
workspace.move_focused(2)

monitor.list()

system.notify("Hello")

process.spawn("app.exe")
```

Event API:

```lua
on("window_created", function(window)
    print(window.title)
end)
```

The Lua API should use stable abstractions rather than leaking raw Windows internals.

---

# 16. Phase 12 — Workspace Manager

Difficulty: 6/10

Implement:

```text
Workspace 1
Workspace 2
Workspace 3
...
Workspace 10
```

Model:

```text
Monitor
 |
 +-- Workspace
      |
      +-- Windows
```

Example:

```lua
bind("SUPER", "1", function()
    workspace.focus(1)
end)

bind("SUPER+SHIFT", "1", function()
    workspace.move_focused(1)
end)
```

Test:

- switching
- moving windows
- empty workspace
- closing focused window
- workspace persistence
- monitor assignment

---

# 17. Phase 13 — Layout Engine

Difficulty: 8/10

Start simple.

## Layout 1: Master/Stack

```text
+------------------+---------+
|                  |         |
|                  | Window 2|
|    Window 1      |---------|
|                  | Window 3|
|                  |         |
+------------------+---------+
```

Then add:

- master/stack
- dwindle
- monocle
- floating
- fullscreen

Input:

```text
Windows
Workspace
Monitor
Layout configuration
```

Output:

```text
Window -> Rectangle
```

Keep layout calculations native.

---

# 18. Phase 14 — Window Rules

Difficulty: 6/10

Lua:

```lua
rule({
    process = "discord.exe",
    floating = true,
})

rule({
    process = "firefox.exe",
    workspace = 2,
})

rule({
    process = "code.exe",
    workspace = 1,
})
```

Support matching by:

- process
- executable
- title
- class where available
- workspace
- monitor

---

# 19. Phase 15 — Renderer

Difficulty: 8/10

Only after the window/event model works.

Pipeline:

```text
Application state
       |
       v
Desktop state
       |
       v
Render tree
       |
       v
Native renderer
       |
       v
GPU
```

Features:

- background
- borders
- rounded corners where technically possible
- transparency
- text
- icons
- animations
- transitions

Do not make Lua responsible for per-frame rendering.

---

# 20. Phase 16 — Visual System

Difficulty: 7/10

Build a unified theme model.

Example:

```lua
theme {
    background = "#111111",
    foreground = "#eeeeee",
    accent = "#88c0d0",
    border = "#444444",

    font = "JetBrainsMono",
    font_size = 13,

    rounding = 8,
    border_width = 2,
}
```

Everything visual should consume the same theme state.

---

# 21. Phase 17 — Bar

Difficulty: 5/10

Create a configurable bar.

Example:

```text
+------------------------------------------------------+
| 1  2  3  4        Firefox              RAM 8GB  18:42|
+------------------------------------------------------+
```

Lua:

```lua
bar {
    position = "top",
    height = 28,

    left = {
        workspaces()
    },

    center = {
        window_title()
    },

    right = {
        cpu(),
        memory(),
        clock()
    }
}
```

---

# 22. Phase 18 — Launcher

Difficulty: 6/10

Bind:

```text
SUPER
```

to launcher.

Features:

- application search
- executable search
- command execution
- file search
- calculator
- history

Example:

```text
+-----------------------------+
| > firefox                   |
+-----------------------------+
| Firefox                     |
| Firefox Developer Edition  |
+-----------------------------+
```

---

# 23. Phase 19 — Notifications

Difficulty: 5/10

API:

```lua
notify({
    title = "Build",
    message = "Compilation finished"
})
```

Features:

- queue
- timeout
- priority
- dismissal
- history
- theme integration

---

# 24. Phase 20 — Configuration System

Difficulty: 6/10

Configuration should be declarative where possible.

Suggested structure:

```text
.config/yourdesktop/
├── config.lua
├── keybinds.lua
├── rules.lua
├── theme.lua
├── bar.lua
├── animations.lua
├── monitors.lua
└── plugins/
```

Allow importing:

```lua
require("theme")
require("keybinds")
require("rules")
```

---

# 25. Phase 21 — Hot Reload

Difficulty: 7/10

Workflow:

```text
edit config.lua
      |
      v
save
      |
      v
parse
      |
      v
validate
      |
      v
build new state
      |
      v
apply
```

Never apply an invalid configuration.

If loading fails:

```text
new config
    |
    X
    |
rollback
    |
last known good config
```

---

# 26. Phase 22 — Rice/Theme Ecosystem

Difficulty: 6/10

A rice should be portable.

Example:

```text
rices/
├── minimal/
├── nord/
├── catppuccin/
├── gruvbox/
└── custom/
```

Each rice can contain:

```text
theme.lua
bar.lua
animations.lua
wallpaper.lua
rules.lua
```

A user should be able to install a rice and modify it without recompiling the core.

---

# 27. Phase 23 — Plugin System

Difficulty: 8/10

Plugin structure:

```text
plugins/
├── system-monitor/
│   └── init.lua
├── weather/
│   └── init.lua
└── media/
    └── init.lua
```

Plugin lifecycle:

```text
discover
   |
load
   |
initialize
   |
run
   |
unload
```

Plugins need permissions.

Example:

```lua
plugin.permissions {
    "windows.read",
    "notifications",
}
```

Do not give plugins unrestricted privileged access by default.

---

# 28. Phase 24 — IPC

Difficulty: 7/10

Create a native IPC protocol between:

```text
desktop-core
     |
     +-- CLI
     +-- plugins
     +-- external tools
```

Commands:

```text
reload
workspace 2
focus firefox
close-focused
toggle-floating
windows
```

---

# 29. Phase 25 — CLI

Difficulty: 5/10

Create:

```text
yourctl.exe
```

Examples:

```powershell
yourctl reload
yourctl workspace 2
yourctl windows
yourctl focus firefox
yourctl toggle-floating
```

Machine-readable output should be available for scripting.

---

# 30. Phase 26 — Zsh / Unix-like Workflow

Difficulty: 5/10

The project does not need to reimplement Zsh.

Instead, provide a first-class terminal workflow.

Target:

```text
Your Desktop
    |
    +-- Terminal
          |
          +-- Zsh
                |
                +-- git
                +-- ssh
                +-- curl
                +-- ripgrep
                +-- fd
                +-- fzf
                +-- tmux
                +-- neovim
                +-- cargo
                +-- python
```

The CLI should integrate naturally:

```bash
yourctl workspace 3
yourctl reload
yourctl windows
```

---

# 31. Phase 27 — Session Architecture

Difficulty: 8/10

Move from:

```text
Windows
  |
Explorer
  |
Your program
```

toward:

```text
Windows login/session
       |
       v
Your desktop session
       |
       +-- Core
       +-- Renderer
       +-- Shell
       +-- Lua
```

This phase must be done only in a dedicated VM.

Always maintain a recovery path.

---

# 32. Phase 28 — Recovery System

Difficulty: 7/10

Required modes:

```text
Normal
Safe
Recovery
```

Recovery options:

```text
1. Start without user config
2. Disable plugins
3. Restore last working configuration
4. Open terminal
5. Reset configuration
6. Collect logs
```

Example failure:

```text
config.lua
   |
syntax error
   |
Desktop refuses new config
   |
old config remains active
```

This should be automatic.

---

# 33. Phase 29 — Multi-monitor

Difficulty: 8/10

Support:

- multiple monitors
- different resolutions
- different refresh rates
- monitor hotplugging
- primary monitor
- workspace-to-monitor assignment
- monitor-specific configuration

Example:

```lua
monitor("Display1", {
    workspaces = { 1, 2, 3, 4, 5 }
})

monitor("Display2", {
    workspaces = { 6, 7, 8, 9, 10 }
})
```

---

# 34. Phase 30 — DPI and Scaling

Difficulty: 8/10

Test:

```text
100%
125%
150%
175%
200%
```

Test combinations:

```text
1080p + 125%
1440p + 100%
4K + 150%
4K + 200%
```

Check:

- text
- icons
- windows
- launcher
- bar
- notifications
- cursor
- popup placement

---

# 35. Phase 31 — Window Edge Cases

Difficulty: 8/10

Test:

- popup windows
- modal dialogs
- child windows
- tool windows
- transparent windows
- always-on-top windows
- system dialogs
- UAC prompts
- hidden windows
- applications spawning secondary windows

Create a compatibility matrix.

---

# 36. Phase 32 — Fullscreen and Gaming

Difficulty: 9/10

Test:

```text
Windowed
Borderless
Fullscreen
```

Test:

- Alt+Tab
- focus loss
- focus restoration
- resolution change
- monitor switching
- overlays
- controller input
- high refresh rates
- HDR where relevant
- multi-monitor

Never assume every game behaves identically.

---

# 37. Phase 33 — Gaming Compatibility Matrix

Difficulty: 9/10

Create a test table:

| Application | Windowed | Borderless | Fullscreen | Multi-monitor | Overlay |
|---|---|---|---|---|---|
| Game A | | | | | |
| Game B | | | | | |
| Game C | | | | | |

Target categories:

- competitive games
- anti-cheat protected games
- DirectX games
- Vulkan games where applicable
- older Win32 games
- Steam games
- launchers

The environment must not bypass or weaken anti-cheat security.

---

# 38. Phase 34 — Performance Benchmarking

Difficulty: 7/10

Measure Windows baseline first.

## Desktop metrics

```text
Idle RAM
Idle CPU
Idle GPU
Process count
Startup/session time
Background activity
```

Then measure the custom environment.

## Gaming metrics

```text
Average FPS
1% low
0.1% low
Frame time
CPU utilization
GPU utilization
RAM
```

Compare:

```text
Windows baseline
vs
Your environment
```

Desired result:

```text
RAM            lower
CPU idle       lower
background     lower
FPS            approximately unchanged
latency        approximately unchanged
```

Do not claim performance improvements without measured data.

---

# 39. Phase 35 — Windows Update Compatibility

Difficulty: 9/10

Maintain a dedicated update-testing VM.

Workflow:

```text
Known-good snapshot
        |
        v
Windows Update
        |
        v
Boot
        |
        v
Automated tests
        |
        +-- pass
        |
        +-- fail -> investigate/rollback
```

Test:

```text
Desktop starts
Core starts
Lua loads
Input works
Windows detected
Workspaces work
Renderer works
Launcher works
Bar works
Notifications work
Applications work
Fullscreen works
Recovery works
```

---

# 40. Phase 36 — Version Abstraction

Difficulty: 9/10

Keep version-sensitive Windows behavior isolated.

Example:

```text
platform-windows/
├── common/
├── version/
├── input/
├── windows/
├── monitor/
└── graphics/
```

Conceptually:

```text
Core
 |
Platform interface
 |
+-- Windows backend
      |
      +-- version-specific implementation
```

If Windows changes an implementation detail, only the relevant backend should require adaptation.

Prefer documented APIs.

---

# 41. Phase 37 — Automated Integration Tests

Difficulty: 8/10

Create tests for:

```text
Window creation
Window destruction
Focus
Move
Resize
Minimize
Restore
Workspace switching
Layout calculation
Lua loading
Lua errors
Configuration rollback
IPC
CLI
Monitor changes
Recovery
```

Example:

```text
start environment
    |
launch test application
    |
verify window appears
    |
move to workspace 2
    |
verify workspace state
    |
reload config
    |
verify state remains valid
```

---

# 42. Phase 38 — Fuzzing

Difficulty: 8/10

Fuzz:

- Lua configuration
- config values
- malformed IPC
- window events
- unexpected window states
- plugin loading
- serialized state

Examples of invalid configuration:

```lua
gaps_inner = "hello"
```

or:

```lua
workspace.count = -500
```

The environment must reject invalid values cleanly.

---

# 43. Phase 39 — Security Model

Difficulty: 9/10

Keep the main environment user-mode whenever possible.

Avoid:

```text
Desktop
  |
  +-- arbitrary kernel code
```

Prefer:

```text
Desktop
  |
  +-- user-mode Core
        |
        +-- Lua
        +-- plugins
```

Define permissions for plugins.

Possible capabilities:

```text
windows.read
windows.manage
notifications
process.spawn
filesystem.read
network
```

Sensitive capabilities should be opt-in.

---

# 44. Phase 40 — Crash Isolation

Difficulty: 8/10

Do not put everything in one process if doing so creates a single point of failure.

Possible model:

```text
desktop-core
    |
    +-- input-service
    +-- renderer-service
    +-- shell-service
    +-- notification-service
```

Only split processes where the complexity/overhead is justified.

A crash should ideally result in:

```text
renderer crash
    |
    v
renderer restart
    |
    v
desktop survives
```

not:

```text
renderer crash
    |
    v
whole session dead
```

---

# 45. Phase 41 — Installer

Difficulty: 7/10

Installer responsibilities:

- install binaries
- install default configuration
- install documentation
- configure user session
- provide recovery
- preserve existing configuration
- provide uninstall

Never make installation irreversible.

---

# 46. Phase 42 — Configuration Migration

Difficulty: 7/10

When configuration syntax changes:

```text
config v1
   |
migration
   |
config v2
```

Keep compatibility where reasonable.

Do not silently reinterpret dangerous configuration.

---

# 47. Phase 43 — Release Channels

Use:

```text
nightly
beta
stable
```

Development:

```text
nightly
```

Testing:

```text
beta
```

Production:

```text
stable
```

---

# 48. Phase 44 — Physical Hardware Testing

Only after VM stability:

```text
Stage 1
VM

Stage 2
secondary physical PC

Stage 3
secondary laptop

Stage 4
main PC
```

Test real:

- GPU
- audio
- Wi-Fi
- Bluetooth
- multiple monitors
- sleep/wake
- laptop lid
- external displays
- high refresh rate
- different GPU drivers

---

# 49. Phase 45 — Deep Windows Graphics Research

Difficulty: 10/10

This is where the project becomes significantly deeper than a conventional shell replacement.

Study the relationship between:

```text
Win32
  |
User32
  |
DWM
  |
DirectComposition / graphics components
  |
DXGI
  |
WDDM
  |
GPU driver
  |
GPU
```

Goal:

Determine precisely:

- what can be replaced
- what must remain
- what can be intercepted through supported APIs
- what is undocumented
- what changes between Windows builds
- what applications depend on
- what games depend on

Do not replace deep components merely because they exist.

Each proposed replacement needs:

```text
Compatibility analysis
Performance analysis
Security analysis
Update-risk analysis
Recovery plan
```

---

# 50. Phase 46 — Deep Composition Experiments

Difficulty: 10/10

Run experiments only in disposable VMs or isolated test systems.

For each experiment:

```text
Hypothesis
    |
Prototype
    |
Benchmark
    |
Compatibility test
    |
Crash test
    |
Windows Update test
    |
Decision
```

Possible outcomes:

```text
KEEP
REPLACE
WRAP
ABSTRACT
ABANDON
```

Do not allow experimental internals to become mandatory dependencies of the stable branch until thoroughly tested.

---

# 51. Phase 47 — Performance Goals

The project should target measurable goals, not marketing numbers.

Possible goals:

## Idle

```text
Low RAM usage
Low CPU usage
Low GPU activity
Low process overhead
```

## Interactive desktop

```text
Smooth animations
Consistent frame pacing
No noticeable input lag
```

## Gaming

```text
No meaningful FPS regression
No meaningful frametime regression
No unnecessary GPU overhead
No unnecessary CPU overhead
```

Potential savings from replacing/removing desktop components can vary substantially between Windows installations.

A reasonable engineering target is to measure the baseline and optimize against it rather than assume a fixed percentage.

---

# 52. Phase 48 — Documentation

Documentation structure:

```text
docs/
├── architecture/
│   ├── overview.md
│   ├── core.md
│   ├── events.md
│   ├── windows.md
│   ├── renderer.md
│   └── recovery.md
│
├── lua/
│   ├── getting-started.md
│   ├── api.md
│   ├── events.md
│   ├── plugins.md
│   └── examples.md
│
├── development/
│   ├── setup.md
│   ├── vm.md
│   ├── testing.md
│   ├── debugging.md
│   └── releases.md
│
└── compatibility/
    ├── windows.md
    ├── games.md
    └── applications.md
```

---

# 53. Phase 49 — Community/Rice Ecosystem

Eventually users should be able to share:

```text
themes
rices
plugins
layouts
keybind configurations
bar configurations
launcher configurations
```

Example:

```text
my-rice/
├── theme.lua
├── bar.lua
├── keybinds.lua
├── rules.lua
├── animations.lua
└── README.md
```

A rice should never need to modify the core executable.

---

# 54. Phase 50 — Version 1.0 Definition

V1 should aim for:

```text
[Core]
[x] Native runtime
[x] Lifecycle
[x] Logging
[x] Error handling

[Window system]
[x] Window discovery
[x] Focus
[x] Move
[x] Resize
[x] Rules
[x] Tiling
[x] Floating
[x] Fullscreen

[Input]
[x] Keybindings
[x] Event system

[Workspaces]
[x] Multiple workspaces
[x] Window movement
[x] Monitor assignment

[Lua]
[x] Configuration
[x] Keybindings
[x] Rules
[x] Events
[x] API
[x] Hot reload

[Desktop]
[x] Renderer
[x] Bar
[x] Launcher
[x] Notifications
[x] Themes

[System]
[x] IPC
[x] CLI
[x] Recovery
[x] Logging

[Compatibility]
[x] Multi-monitor
[x] DPI
[x] Normal Windows applications
[x] Gaming testing
[x] Windows Update testing

[Security]
[x] User-mode architecture where possible
[x] Plugin permissions
[x] Recovery
```

Deep kernel/WDDM replacement is not required for V1.

---

# 55. Recommended Milestones

## M0 — Foundation

```text
VM
Toolchain
Repository
CI
Core
Logging
```

Result:

```text
desktop-core.exe
```

starts reliably.

## M1 — Windows awareness

```text
Window inspector
Process inspector
Monitor inspector
Platform abstraction
```

Result:

Your program understands the Windows desktop.

## M2 — Window management

```text
Window model
Events
Input
Keybindings
Workspace
Layout
```

Result:

A functional native window-management environment.

## M3 — Lua

```text
Lua runtime
Lua API
Configuration
Rules
Hot reload
```

Result:

The user can configure the system without recompiling.

## M4 — Desktop

```text
Renderer
Bar
Launcher
Notifications
Themes
Animations
```

Result:

A complete custom desktop experience.

## M5 — System integration

```text
IPC
CLI
Zsh workflow
Session
Recovery
```

Result:

The environment behaves like a real desktop system.

## M6 — Compatibility

```text
Multi-monitor
DPI
Fullscreen
Games
Applications
Windows Update
```

Result:

Production-quality compatibility.

## M7 — Deep research

```text
DWM
composition
graphics internals
```

Result:

Determine how much deeper the platform can be replaced without compromising Windows compatibility.

---

# 56. Example Final User Experience

After installation:

```text
Windows
   |
   v
Your Desktop Session
   |
   +-- Lua configuration
   |
   +-- Tiling WM
   |
   +-- Workspaces
   |
   +-- Launcher
   |
   +-- Bar
   |
   +-- Notifications
   |
   +-- Terminal
         |
         +-- Zsh
```

User configuration:

```lua
-- config.lua

desktop {
    gaps_inner = 8,
    gaps_outer = 12,
    border_width = 2,
    rounding = 8,
}

layout("dwindle")

theme {
    font = "JetBrainsMono",
    font_size = 13,
}

bind("SUPER", "ENTER", function()
    spawn("wt.exe")
end)

bind("SUPER", "1", function()
    workspace.focus(1)
end)

bind("SUPER", "2", function()
    workspace.focus(2)
end)

rule({
    process = "firefox.exe",
    workspace = 2,
})

rule({
    process = "discord.exe",
    floating = true,
})
```

The user can change the entire experience without recompiling the native core.

---

# 57. Development Rules

## Rule 1

Never optimize before measuring.

## Rule 2

Never depend on undocumented Windows behavior unless the dependency is isolated and explicitly documented.

## Rule 3

Never make the experimental graphics layer mandatory for the stable build.

## Rule 4

Never test risky shell changes first on the main PC.

## Rule 5

Every subsystem gets logs and tests.

## Rule 6

Every configuration change must be recoverable.

## Rule 7

Lua controls behavior; native code performs performance-critical work.

## Rule 8

Keep Windows-specific code behind a platform abstraction.

## Rule 9

Treat Windows Update as an external compatibility variable.

## Rule 10

Gaming compatibility is tested, not assumed.

---

# 58. First 10 Concrete Tasks

Do not start with DWM.

Start exactly here:

```text
1. Create Windows development VM.
2. Create VM snapshot.
3. Install Rust + Visual Studio Build Tools + Windows SDK.
4. Create Git repository.
5. Create Cargo workspace.
6. Implement core runtime.
7. Implement structured logging.
8. Implement Windows window inspector.
9. Implement platform abstraction.
10. Implement internal Window model.
```

At the end of task 10, you should have:

```text
Windows VM
    |
    +-- desktop-core.exe
    |
    +-- window-inspector.exe
```

and your program should be able to reliably observe Windows windows without attempting to replace the desktop yet.

---

# 59. The Long-Term Architecture

The final conceptual model is:

```text
                         WINDOWS
                            |
             +--------------+--------------+
             |                             |
          Kernel                         Drivers
             |                             |
             +--------------+--------------+
                            |
                    Windows platform
                            |
              +-------------v-------------+
              |       DESKTOP CORE        |
              |                           |
              | Event system              |
              | Window system             |
              | Workspace system          |
              | Layout engine             |
              | Renderer                  |
              | Session                   |
              | IPC                       |
              +-------------+-------------+
                            |
                  +---------+---------+
                  |                   |
                Lua                 Shell
                  |                   |
       +----------+----------+        |
       |          |          |        |
    Config      Rules     Plugins     |
                                      |
                         +------------+------------+
                         |                         |
                      Launcher                   Bar
                         |
                    Notifications
```

The end result should feel like:

```text
Arch
+
Hyprland
+
Omarchy
+
Rice
+
Lua
+
Zsh
```

while the underlying platform remains:

```text
Windows
+
Windows drivers
+
Windows security
+
Windows application compatibility
+
Windows gaming ecosystem
```

The key engineering objective is not simply to make Windows look different.

It is to create a **real alternative desktop environment with its own architecture**, while progressively reducing dependence on the traditional Windows desktop experience without breaking the platform underneath it.
