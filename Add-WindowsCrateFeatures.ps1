while ($true) {
    $features=cargo build 2>&1 `
    | rg "the item is gated behind" `
    | ForEach-Object { ($_ -split "``")[1] } `
    | Select-Object -Unique
    if ([string]::IsNullOrEmpty($features)) {
        Write-Host "No Windows features found"
        return
    }
    cargo add windows --features $features
}