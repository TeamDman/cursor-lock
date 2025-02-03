use crossterm::event::{read, Event, KeyCode, KeyEvent};
use eyre::Result;

/// Waits for the user to press a key and returns its virtual-key code.
pub fn pick_toggle_key() -> Result<u32> {
    println!("Please press the key you would like to use as the toggle key... (except F12)");

    loop {
        if let Event::Key(KeyEvent { code, .. }) = read()? {
            let vk = match code {
                // check if F12
                KeyCode::F(12) => {
                    println!("F12 is not allowed, see https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey#remarks");
                    continue;
                }
                KeyCode::F(n) => {
                    // Windows virtual key code for F1 is 0x70.
                    0x70 + (n as u32) - 1
                }
                KeyCode::Char(c) => {
                    // Convert to uppercase and use its ASCII code.
                    c.to_ascii_uppercase() as u32
                }
                // Ignore other keys (e.g. Enter, Esc) and continue looping.
                _ => continue,
            };
            println!("Toggle key set to virtual key code: {:#X}", vk);
            return Ok(vk);
        }
    }
}
