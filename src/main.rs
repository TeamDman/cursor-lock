mod clip_cursor;
mod monitors;
mod chimes;

use clip_cursor::{activate_clipping, deactivate_clipping};
use eyre::bail;
use monitors::pick_monitor;
use std::thread::sleep;
use std::time::Duration;
use windows::Win32::Foundation::RECT;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    
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

    // Activate clipping and play the activation sound.
    activate_clipping(rect)?;
    println!("Cursor locked. It will be unlocked after 20 seconds.");

    sleep(Duration::from_secs(20));

    // Deactivate clipping and play the deactivation sound.
    deactivate_clipping()?;
    println!("Cursor clipping deactivated. Exiting.");

    Ok(())
}
