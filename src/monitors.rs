use std::io::Write;
use std::io::{self};
use std::mem::size_of;
use std::mem::zeroed;
use windows::Win32::Devices::Display::DisplayConfigGetDeviceInfo;
use windows::Win32::Devices::Display::GetDisplayConfigBufferSizes;
use windows::Win32::Devices::Display::QueryDisplayConfig;
use windows::Win32::Devices::Display::DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME;
use windows::Win32::Devices::Display::DISPLAYCONFIG_MODE_INFO;
use windows::Win32::Devices::Display::DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE;
use windows::Win32::Devices::Display::DISPLAYCONFIG_PATH_INFO;
use windows::Win32::Devices::Display::DISPLAYCONFIG_TARGET_DEVICE_NAME;
use windows::Win32::Devices::Display::QDC_ONLY_ACTIVE_PATHS;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::Graphics::Gdi::DISPLAYCONFIG_PATH_MODE_IDX_INVALID;

#[derive(Debug, Clone)]
pub struct Monitor {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub fn get_monitors() -> Vec<Monitor> {
    let mut num_paths: u32 = 0;
    let mut num_modes: u32 = 0;

    let status = unsafe {
        GetDisplayConfigBufferSizes(QDC_ONLY_ACTIVE_PATHS, &mut num_paths, &mut num_modes)
    };
    if status != ERROR_SUCCESS {
        eprintln!("GetDisplayConfigBufferSizes failed.");
        return Vec::new();
    }

    // Allocate buffers for paths and modes.
    let mut paths: Vec<DISPLAYCONFIG_PATH_INFO> = Vec::with_capacity(num_paths as usize);
    let mut modes: Vec<DISPLAYCONFIG_MODE_INFO> = Vec::with_capacity(num_modes as usize);

    let status = unsafe {
        QueryDisplayConfig(
            QDC_ONLY_ACTIVE_PATHS,
            &mut num_paths,
            paths.as_mut_ptr(),
            &mut num_modes,
            modes.as_mut_ptr(),
            None,
        )
    };
    if status != ERROR_SUCCESS {
        eprintln!("QueryDisplayConfig failed.");
        return Vec::new();
    }

    // Tell Rust the vectors have been fully populated.
    unsafe {
        paths.set_len(num_paths as usize);
        modes.set_len(num_modes as usize);
    }

    let mut monitors = Vec::new();

    for path in &paths {
        // Use DISPLAYCONFIG_TARGET_DEVICE_NAME to retrieve a friendly monitor name.
        let mut device_name: DISPLAYCONFIG_TARGET_DEVICE_NAME = unsafe { zeroed() };
        device_name.header.size = size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>() as u32;
        device_name.header.adapterId = path.targetInfo.adapterId;
        device_name.header.id = path.targetInfo.id;
        device_name.header.r#type = DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME;

        let res = unsafe { DisplayConfigGetDeviceInfo(&mut device_name.header) };
        if res != ERROR_SUCCESS.0 as i32 {
            continue;
        }
        let mut name = String::from_utf16_lossy(&device_name.monitorFriendlyDeviceName);
        name = name.trim_end_matches('\0').to_string();

        // Extract position and size from the source mode info.
        let (x, y, width, height) = {
            let mut x = 0;
            let mut y = 0;
            let mut width = 0;
            let mut height = 0;
            let mode_info_idx = unsafe { path.sourceInfo.Anonymous.modeInfoIdx };
            if mode_info_idx != DISPLAYCONFIG_PATH_MODE_IDX_INVALID {
                let mode_index = mode_info_idx as usize;
                if mode_index < modes.len() {
                    let mode = &modes[mode_index];
                    if mode.infoType == DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE {
                        let source_mode = unsafe { mode.Anonymous.sourceMode };
                        x = source_mode.position.x;
                        y = source_mode.position.y;
                        width = source_mode.width as i32;
                        height = source_mode.height as i32;
                    }
                }
            }
            (x, y, width, height)
        };

        monitors.push(Monitor {
            name,
            x,
            y,
            width,
            height,
        });
    }

    // Sort monitors: first left-to-right (by x) then top-to-bottom (by y)
    monitors.sort_by(|a, b| {
        if a.x == b.x {
            a.y.cmp(&b.y)
        } else {
            a.x.cmp(&b.x)
        }
    });

    monitors
}

pub fn pick_monitor() -> Option<Monitor> {
    let monitors = get_monitors();
    if monitors.is_empty() {
        eprintln!("No monitors found.");
        return None;
    }

    println!("Available monitors:");
    for (i, monitor) in monitors.iter().enumerate() {
        println!(
            "{}: {} ({}x{}, pos: {}x{})",
            i + 1,
            monitor.name,
            monitor.width,
            monitor.height,
            monitor.x,
            monitor.y
        );
    }

    print!("Please select a monitor by entering its number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if let Err(err) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read input: {}", err);
        return None;
    }

    let trimmed = input.trim();
    if let Ok(index) = trimmed.parse::<usize>() {
        if index > 0 && index <= monitors.len() {
            return Some(monitors[index - 1].clone());
        }
    }

    eprintln!("Invalid selection.");
    None
}
