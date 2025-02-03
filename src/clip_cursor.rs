use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::ClipCursor;

pub fn activate_clipping(rect: RECT) -> eyre::Result<()> {
    unsafe {
        ClipCursor(Some(&rect))?;
    }
    Ok(())
}

pub fn deactivate_clipping() -> eyre::Result<()> {
    unsafe {
        // Passing a null pointer removes any clipping region.
        ClipCursor(None)?;
    }
    Ok(())
}
