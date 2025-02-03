use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::ClipCursor;
use eyre::Result;

// Import our chimes module.
use crate::chimes;

pub fn activate_clipping(rect: RECT) -> Result<()> {
    unsafe {
        // Clip the cursor to the given rectangle.
        // Using Some(&rect) to pass a valid clipping region.
        ClipCursor(Some(&rect))?;
    }
    // Play the activation chime.
    chimes::play_sound("src/assets/activate.mp3")?;
    Ok(())
}

pub fn deactivate_clipping() -> Result<()> {
    unsafe {
        // Passing None removes any clipping region.
        ClipCursor(None)?;
    }
    // Play the deactivation chime.
    chimes::play_sound("src/assets/deactivate.mp3")?;
    Ok(())
}
