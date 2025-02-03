use crate::clip_cursor::activate_clipping;
use crate::clip_cursor::deactivate_clipping;
use eyre::Result;
use windows::Win32::UI::Input::KeyboardAndMouse::RegisterHotKey;
use windows::Win32::UI::Input::KeyboardAndMouse::UnregisterHotKey;
use windows::Win32::UI::Input::KeyboardAndMouse::HOT_KEY_MODIFIERS;
use windows::Win32::UI::WindowsAndMessaging::MSG;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_F9;
use windows::Win32::UI::WindowsAndMessaging::GetMessageW;
use windows::Win32::UI::WindowsAndMessaging::WM_HOTKEY;

/// Spawns a thread that registers F9 as a global hotkey and listens for WM_HOTKEY messages.
/// When the hotkey is pressed, the clipping state is toggled.
/// The provided `rect` is used for activation, and `enabled` is a shared flag.
pub fn run_hotkey_listener(rect: RECT, enabled: Arc<AtomicBool>) -> Result<()> {
    // Register F9 (virtual key code 0x78) as hotkey id 1.
    unsafe {
        let hotkey_id = 1;
        // hWnd = HWND(0) means the hotkey is global.
        RegisterHotKey(HWND::default(), hotkey_id, HOT_KEY_MODIFIERS::default(), VK_F9.0 as u32)?;
    }

    std::thread::spawn(move || {
        let mut msg = MSG::default();
        loop {
            // Block until a message is received.
            unsafe {
                if GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
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
                } else {
                    break;
                }
            }
        }
        // Unregister the hotkey when exiting the loop.
        unsafe {
            if let Err(e) = UnregisterHotKey(HWND::default(), 1) {
                eprintln!("Error unregistering hotkey: {:?}", e);
            };
        }
    });

    Ok(())
}
