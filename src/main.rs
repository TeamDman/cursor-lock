mod chimes;
mod clip_cursor;
mod focus;
mod hotkeys;
mod monitors;
mod toggle_key;

use clip_cursor::{activate_clipping, deactivate_clipping};
use eyre::bail;
use monitors::pick_monitor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use toggle_key::pick_toggle_key;
use windows::Win32::Foundation::RECT;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    // Ask the user to pick a monitor.
    let monitor = match pick_monitor() {
        Some(m) => m,
        None => {
            bail!("No monitor selected.");
        }
    };

    // Compute the rectangle for cursor clipping.
    let rect = RECT {
        left: monitor.x,
        top: monitor.y,
        right: monitor.x + monitor.width,
        bottom: monitor.y + monitor.height,
    };

    println!(
        "Locking cursor to monitor: {} ({}x{}, pos: {}x{})",
        monitor.name, monitor.width, monitor.height, monitor.x, monitor.y
    );

    // Wait for the user to press a key for the toggle key.
    let toggle_key = pick_toggle_key().unwrap_or(0x78); // Default to F9 (0x78) if something goes wrong.

    // Now activate clipping.
    activate_clipping(rect)?;
    // The global "enabled" state starts as true.
    let enabled = Arc::new(AtomicBool::new(true));

    // Launch the hotkey listener in a separate thread using the chosen key.
    hotkeys::run_hotkey_listener(rect, enabled.clone(), toggle_key)?;
    // Launch the focus hook to reapply clipping on foreground changes.
    focus::run_focus_hook(rect, enabled.clone())?;

    // Install a Ctrl+C handler to ensure clipping is deactivated on exit.
    {
        let enabled_clone = enabled.clone();
        ctrlc::set_handler(move || {
            if enabled_clone.load(Ordering::SeqCst) {
                if let Err(e) = deactivate_clipping() {
                    eprintln!("Error deactivating clipping: {:?}", e);
                }
            }
            std::process::exit(0);
        })?;
    }

    println!("Hotkey listener running (press your chosen key to toggle clipping). Press Ctrl+C to exit.");
    // Wait indefinitely.
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
