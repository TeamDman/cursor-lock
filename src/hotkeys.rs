use crate::clip_cursor::activate_clipping;
use crate::clip_cursor::deactivate_clipping;
use eyre::Context;
use eyre::Result;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Foundation::RECT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::KeyboardAndMouse::RegisterHotKey;
use windows::Win32::UI::Input::KeyboardAndMouse::UnregisterHotKey;
use windows::Win32::UI::Input::KeyboardAndMouse::HOT_KEY_MODIFIERS;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_F9;
use windows::Win32::UI::WindowsAndMessaging::CreateWindowExW;
use windows::Win32::UI::WindowsAndMessaging::DefWindowProcW;
use windows::Win32::UI::WindowsAndMessaging::DestroyWindow;
use windows::Win32::UI::WindowsAndMessaging::DispatchMessageW;
use windows::Win32::UI::WindowsAndMessaging::GetMessageW;
use windows::Win32::UI::WindowsAndMessaging::PostQuitMessage;
use windows::Win32::UI::WindowsAndMessaging::RegisterClassW;
use windows::Win32::UI::WindowsAndMessaging::TranslateMessage;
use windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;
use windows::Win32::UI::WindowsAndMessaging::MSG;
use windows::Win32::UI::WindowsAndMessaging::WM_DESTROY;
use windows::Win32::UI::WindowsAndMessaging::WM_HOTKEY;
use windows::Win32::UI::WindowsAndMessaging::WNDCLASSW;

/// This is our window procedure for the hidden window. We don’t need to process many messages;
/// just WM_HOTKEY and WM_DESTROY.
extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_HOTKEY => {
            // We'll process WM_HOTKEY in our message loop in the spawned thread.
            // (We can simply call DefWindowProcW here.)
            unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
        }
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

/// Creates a hidden window that will be used to receive hotkey messages.
fn create_message_window() -> Result<HWND> {
    // Define a class name (must be wide string).
    let class_name = "HiddenHotkeyWindow";
    let class_name_w: Vec<u16> = class_name.encode_utf16().chain(Some(0)).collect();

    unsafe {
        let hinstance = GetModuleHandleW(None)?;
        let wnd_class = WNDCLASSW {
            hInstance: hinstance.into(),
            lpszClassName: PCWSTR(class_name_w.as_ptr()),
            lpfnWndProc: Some(wnd_proc),
            ..Default::default()
        };

        // Register the window class.
        let atom = RegisterClassW(&wnd_class);
        if atom == 0 {
            eyre::bail!("Failed to register window class");
        }

        // Create a hidden window.
        let hwnd = CreateWindowExW(
            Default::default(),
            PCWSTR(class_name_w.as_ptr()),
            PCWSTR(class_name_w.as_ptr()),
            Default::default(),
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            HWND::default(), // No parent.
            None,            // No menu.
            hinstance,
            None, // No additional parameters.
        )?;

        if hwnd.0 == std::ptr::null_mut() {
            eyre::bail!("Failed to create hidden window");
        }
        Ok(hwnd)
    }
}

/// Spawns a thread that registers F9 as a global hotkey (using a hidden window) and listens for WM_HOTKEY messages.
/// When the hotkey is pressed, the clipping state is toggled. The provided `rect` is used for activation,
/// and `enabled` is a shared flag.
pub fn run_hotkey_listener(rect: RECT, enabled: Arc<AtomicBool>) -> Result<()> {
    // Spawn a thread to run the message loop.
    std::thread::spawn(move || {
        if let Err(e) = run_hotkey_listener_inner(rect, enabled) {
            eprintln!("Error in hotkey listener thread: {:?}", e);
        }
    });

    Ok(())
}

pub fn run_hotkey_listener_inner(rect: RECT, enabled: Arc<AtomicBool>) -> Result<()> {
    // Create the hidden message window.
    let hwnd = create_message_window()?;

    // Register F9 as a hotkey (using hotkey id 1).
    unsafe {
        let hotkey_id = 1;
        // Use no modifiers.
        let modifiers = HOT_KEY_MODIFIERS::default();
        RegisterHotKey(hwnd, hotkey_id, modifiers, VK_F9.0 as u32)
            .wrap_err("Failed to register hotkey")?;
    }

    let mut msg = MSG::default();
    loop {
        unsafe {
            // Block until a message is received.
            if GetMessageW(&mut msg, hwnd, 0, 0).as_bool() {
                if msg.message == WM_HOTKEY {
                    // Toggle the enabled state.
                    let current = enabled.load(Ordering::SeqCst);
                    let new_state = !current;
                    enabled.store(new_state, Ordering::SeqCst);
                    if new_state {
                        println!("Hotkey pressed: activating clipping.");
                        if let Err(e) = activate_clipping(rect) {
                            eprintln!("Error activating clipping: {:?}", e);
                        }
                    } else {
                        println!("Hotkey pressed: deactivating clipping.");
                        if let Err(e) = deactivate_clipping() {
                            eprintln!("Error deactivating clipping: {:?}", e);
                        }
                    }
                }
                _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            } else {
                break;
            }
        }
    }
    // Unregister the hotkey and destroy the hidden window.
    unsafe {
        if let Err(e) = UnregisterHotKey(hwnd, 1) {
            eprintln!("Error unregistering hotkey: {:?}", e);
        }
        DestroyWindow(hwnd)?;
    }

    Ok(())
}
