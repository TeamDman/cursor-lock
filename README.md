# Cursor-Lock

A simple Windows utility for confining the cursor to a specific monitor.

THIS IS A WORK IN PROGRESS.

```pwsh
cursor-lock on  main [?] is 📦 v0.1.0 via 🦀 v1.82.0-nightly 
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target\debug\monitor-scaling.exe`
Available monitors:
1: Optix G27C2 (1920x1080, pos: -1920x593)
2: MAG274UPF (3840x2160, pos: 0x0)
3: K222HQL (1920x1080, pos: 3840x576)
Please select a monitor by entering its number: 2
Locking cursor to monitor: MAG274UPF (3840x2160, pos: 0x0)
Cursor locked. It will be unlocked after 20 seconds.
Cursor clipping deactivated. Exiting.
```
