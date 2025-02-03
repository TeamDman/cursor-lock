use eyre::Result;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::OnceLock;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::Accessibility::SetWinEventHook;
use windows::Win32::UI::Accessibility::UnhookWinEvent;
use windows::Win32::UI::Accessibility::HWINEVENTHOOK;
use windows::Win32::UI::WindowsAndMessaging::ClipCursor;
use windows::Win32::UI::WindowsAndMessaging::DispatchMessageW;
use windows::Win32::UI::WindowsAndMessaging::GetMessageW;
use windows::Win32::UI::WindowsAndMessaging::TranslateMessage;
use windows::Win32::UI::WindowsAndMessaging::EVENT_SYSTEM_FOREGROUND;
use windows::Win32::UI::WindowsAndMessaging::MSG;
use windows::Win32::UI::WindowsAndMessaging::WINEVENT_OUTOFCONTEXT;
use windows::Win32::UI::WindowsAndMessaging::WINEVENT_SKIPOWNPROCESS;

// Our state that we want the hook callback to have access to.
pub struct FocusHookState {
    pub rect: RECT,
    pub enabled: Arc<AtomicBool>,
}

// We use once_cell to create a global mutable state for the hook.
static FOCUS_HOOK_STATE: OnceLock<Option<FocusHookState>> = OnceLock::new();

/// The WinEvent hook callback. When a foreground change is detected, if our enabled flag is true,
/// we reapply the clip.
extern "system" fn win_event_proc(
    _h_win_event_hook: HWINEVENTHOOK,
    _event: u32,
    _hwnd: HWND,
    _id_object: i32,
    _id_child: i32,
    _dw_event_thread: u32,
    _dwms_event_time: u32,
) {
    if let Some(guard) = FOCUS_HOOK_STATE.get() {
        if let Some(state) = guard.as_ref() {
            if state.enabled.load(Ordering::SeqCst) {
                // Reapply the clip.
                let _ = unsafe { ClipCursor(Some(&state.rect)) };
            }
        }
    }
}

/// Installs a WinEvent hook that will detect foreground window changes and reapply the clip when needed.
/// A message loop is spawned on a new thread so that the hook continues running.
pub fn run_focus_hook(rect: RECT, enabled: Arc<AtomicBool>) -> Result<()> {
    std::thread::spawn(move || {
        if let Err(e) = run_focus_hook_inner(rect, enabled) {
            eprintln!("Error in focus hook thread: {:?}", e);
        }
    });
    Ok(())
}
pub fn run_focus_hook_inner(rect: RECT, enabled: Arc<AtomicBool>) -> Result<()> {
    {
        // Store our desired state in the global.
        FOCUS_HOOK_STATE
            .set(Some(FocusHookState {
                rect,
                enabled: enabled.clone(),
            }))
            .map_err(|_| eyre::eyre!("Failed to set focus hook state"))?;
    }

    unsafe {
        // Install the hook for EVENT_SYSTEM_FOREGROUND.
        let hook = SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            Some(win_event_proc),
            0,
            0,
            WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
        );
        if hook.0.is_null() {
            eyre::bail!("Failed to set WinEvent hook");
        }

        // Spawn a thread with a message loop to process hook events.
        let mut msg = MSG::default();
        loop {
            // Block until a message is received.
            if GetMessageW(&mut msg, HWND::default(), 0, 0).0 == 0 {
                break;
            }
            _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        // Unhook when the message loop ends.
        let _ = UnhookWinEvent(hook);
    }
    Ok(())
}
