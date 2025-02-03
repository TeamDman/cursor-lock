# Cursor-Lock

A simple Windows utility for confining the cursor to a specific monitor.



https://github.com/user-attachments/assets/8611340d-15e6-4065-a323-56b80a96ba1a




```pwsh
cursor-lock on  main [$!?] is 📦 v0.1.0 via 🦀 v1.82.0-nightly took 9s
❯ cargo run
   Compiling cursor-lock v0.1.0 (D:\Repos\rust\cursor-lock)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.29s
     Running `target\debug\cursor-lock.exe`
Available monitors:
1: Optix G27C2 (1920x1080, pos: -1920x593)
2: MAG274UPF (3840x2160, pos: 0x0)
3: K222HQL (1920x1080, pos: 3840x576)
Please select a monitor by entering its number: 2
Locking cursor to monitor: MAG274UPF (3840x2160, pos: 0x0)
Please press the key you would like to use as the toggle key... (except F12)
Toggle key set to virtual key code: 0x71
Hotkey listener running (press your chosen key to toggle clipping). Press Ctrl+C to exit.
Hotkey pressed: deactivating clipping.
Hotkey pressed: activating clipping.
Hotkey pressed: deactivating clipping.
Hotkey pressed: activating clipping.
Hotkey pressed: deactivating clipping.
```


## Known Issues

If you have your monitor DPI scaling not at 100% then the program fails to set the clipping bounds properly.

I should be able to fix this but it's time for bed rn lol

See also: https://github.com/teamdman/monitor-scaling/
