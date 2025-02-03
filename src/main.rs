mod clip_cursor;
mod monitors;
mod chimes;
mod hotkeys;

use clip_cursor::{activate_clipping, deactivate_clipping};
use eyre::bail;
use monitors::pick_monitor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::thread;
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

    // Activate clipping immediately.
    activate_clipping(rect)?;
    // The global "enabled" state starts as true.
    let enabled = Arc::new(AtomicBool::new(true));

    // Launch the hotkey listener in a separate thread.
    hotkeys::run_hotkey_listener(rect, enabled.clone())?;

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

    println!("Hotkey listener running (F9 to toggle clipping). Press Ctrl+C to exit.");
    // Wait indefinitely.
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
