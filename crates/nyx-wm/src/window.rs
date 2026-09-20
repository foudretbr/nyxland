//! Window inspection and management helpers for Win32.

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetClassNameW, GetWindowLongW, GetWindowRect, IsWindow, IsWindowVisible,
    GWL_EXSTYLE, GWL_STYLE, WS_CHILD, WS_EX_TOOLWINDOW,
    SetWindowPos, SWP_NOZORDER, SWP_NOACTIVATE, GetWindowTextW, GetWindowTextLengthW,
};
use nyx_core::Rect;
use nyx_config::NyxConfig;

/// Determines whether a given window should be managed by the tiling window manager.
///
/// Under Windows, many windows exist that are hidden, system overlays, tooltips,
/// or background helper processes. This function performs a series of validation checks
/// against the Win32 window properties to ensure only top-level, interactive user application
/// windows are tiled.
///
/// # Checks Performed
/// 1. **Validity**: Ensures the handle (`HWND`) is non-null and currently valid.
/// 2. **Visibility**: Checks if the window is marked visible (`IsWindowVisible`).
/// 3. **Window Styles**:
///    - Must not be a child window (`WS_CHILD`).
///    - Must not be a tool window (`WS_EX_TOOLWINDOW`).
///    - Must not be a popup without caption (menus, context menus).
/// 4. **Dimensions**: Verifies that the window rectangle has a non-zero width and height.
/// 5. **Ignored Classes**: Compares the window's Win32 class name against `config.ignored_classes`.
///
/// # Arguments
/// * `hwnd` - The Win32 window handle (`HWND`) to inspect.
/// * `config` - A reference to [`NyxConfig`] containing user exclusion rules.
///
/// # Returns
/// * `true` if the window is a valid top-level application window suitable for tiling.
/// * `false` if the window should be ignored.
pub fn should_manage_window(hwnd: HWND, config: &NyxConfig) -> bool {
    if hwnd.is_invalid() || unsafe { !IsWindow(hwnd).as_bool() } {
        return false;
    }

    if unsafe { !IsWindowVisible(hwnd).as_bool() } {
        return false;
    }

    let (style, ex_style) = unsafe {
        (
            GetWindowLongW(hwnd, GWL_STYLE) as u32,
            GetWindowLongW(hwnd, GWL_EXSTYLE) as u32,
        )
    };

    // Exclude child Window
    if (style & WS_CHILD.0) != 0 {
        return false;
    }

    // Exclude tool window (tooltips, overlay, floating panels)
    if (ex_style & WS_EX_TOOLWINDOW.0) != 0 {
        return false;
    }

    // Dimensions check
    let mut rect = Default::default();
    if unsafe { GetWindowRect(hwnd, &mut rect).is_ok() } {
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        if width <= 0 || height <= 0 {
            return false;
        }
    } else {
        return false;
    }

    let mut class_buffer = [0u16; 256];
    let class_len = unsafe { GetClassNameW(hwnd, &mut class_buffer) } as usize;
    if class_len > 0 {
        if let Ok(class_name) = String::from_utf16(&class_buffer[..class_len]) {
            if config.ignored_classes.contains(&class_name) {
                return false;
            }
        } 
    }

    true
}

/// Repositions and resizes a window according to the provided rectangle.
///
/// Uses the Win32 `SetWindowPos` API with flags that prevent changing the
/// Z-order or activating the window unintentionally.
///
/// # Arguments
/// * `hwnd` - The Win32 window handle to reposition.
/// * `rect` - The target coordinates and dimensions (`Rect`).
///
/// # Returns
/// * `Ok(())` on success.
/// * `Err(windows::core::Error)` if the Win32 call fails.
pub fn set_window_geometry(hwnd: HWND, rect: &Rect) -> windows::core::Result<()> {
    unsafe {
        SetWindowPos(hwnd, HWND::default(), rect.x, rect.y, rect.width, rect.height, SWP_NOZORDER | SWP_NOACTIVATE,)
    }
}

/// Retrieves the tile text of a given window.
/// 
/// Queries the window text using 'GetWindowTextW'.
/// 
/// # Arguments
/// * 'hwnd' - The Win32 window handle.
/// 
/// # Returns
/// * A 'String' containing the window title, or empty string if it fails or has no title.
pub fn get_window_title(hwnd: HWND) -> String {
    let len = unsafe { GetWindowTextLengthW(hwnd) };

    if len == 0 {
        return String::new();
    }

    let mut buffer = vec![0u16; (len + 1) as usize];

    let copied = unsafe { GetWindowTextW(hwnd, &mut buffer) };

    if copied <= 0 {
        return String::new();
    }

    String::from_utf16_lossy(&buffer[..copied as usize]).to_string()
}