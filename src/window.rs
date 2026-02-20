//! Window types for multi-window support.
//!
//! Contains WindowType enum and Window enum.
//! UI state types are co-located in their respective ui/ modules.

use std::ffi::c_void;
#[cfg(target_os = "windows")]
use std::num::NonZeroIsize;

use crate::ui::main_window::MainWindowState;
use crate::ui::settings_window::SettingsUiState;

use iced::window::raw_window_handle::RawWindowHandle;

/// Window type enum for future extensibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowType {
    Main,
    Settings,
}

impl WindowType {
    /// Returns the window settings for this window type.
    pub fn window_settings(&self) -> iced::window::Settings {
        match self {
            WindowType::Main => iced::window::Settings {
                exit_on_close_request: false,
                ..Default::default()
            },
            WindowType::Settings => iced::window::Settings {
                exit_on_close_request: false,
                size: iced::Size::new(400.0, 300.0),
                ..Default::default()
            },
        }
    }
}

/// Window content enum containing state for each window type.
#[derive(Debug)]
pub enum Window {
    Main(MainWindowState),
    Settings(SettingsUiState),
}

impl Window {
    /// Creates a new Window with the specified type.
    pub fn new(window_type: WindowType) -> Self {
        match window_type {
            WindowType::Main => Window::Main(MainWindowState::new()),
            WindowType::Settings => Window::Settings(SettingsUiState::new()),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn set_parent_window(child: RawWindowHandle, parent: RawWindowHandle) {
    let RawWindowHandle::Win32(child) = child else {
        unreachable!()
    };

    let RawWindowHandle::Win32(parent) = parent else {
        unreachable!()
    };

    let hwndchild = windows::Win32::Foundation::HWND(child.hwnd.get() as *mut c_void);
    let hwndparent = windows::Win32::Foundation::HWND(parent.hwnd.get() as *mut c_void);

    unsafe {
        windows::Win32::UI::WindowsAndMessaging::SetParent(hwndchild, Some(hwndparent)).unwrap()
    };
}

/// Stub for non-Windows platforms.
#[cfg(not(target_os = "windows"))]
pub fn set_parent_window(_child: RawWindowHandle, _parent: RawWindowHandle) {}
